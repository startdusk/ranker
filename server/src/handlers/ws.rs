use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State, TypedHeader,
    },
    headers::{self, authorization::Bearer},
    response::IntoResponse,
    Extension,
};
use redis::aio::{ConnectionLike, ConnectionManager};
use tokio::sync::Mutex;
use validator::Validate;

use std::{cell::RefCell, sync::Arc};

// allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use std::net::SocketAddr;

// allows to split the websocket stream into separate TX and RX branches
use futures::{
    sink::SinkExt,
    stream::{SplitSink, StreamExt},
};

use crate::{
    auth::{self, Authed},
    data::redis::polls,
    errors::Error,
    models::{
        room::{RoomClient, Rooms},
        Nomination, Poll, RankingList, WebSocketEvent,
    },
    shared::ids::create_nomination_id,
    state::AppState,
};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(con): Extension<ConnectionManager>,
    State(state): State<Arc<AppState>>,
    authorization: Option<TypedHeader<headers::Authorization<Bearer>>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let Some(TypedHeader(token)) = authorization else {
        return Error::MissingCredentials.into_response()
    };

    let token = token.0.token();
    let Ok(auth) = auth::verify(token.to_string()) else {
        return Error::InvalidToken.into_response();
    };

    let con = con.clone();
    ws.on_upgrade(move |socket| handle_socket(socket, con, auth, state, addr))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(
    socket: WebSocket,
    mut con: ConnectionManager,
    auth: Authed,
    state: Arc<AppState>,
    addr: SocketAddr,
) {
    let user_id = auth.sub.clone();
    let poll_id = auth.poll_id.clone();
    let name = auth.name;

    let Ok(poll) =
        polls::add_participant(&mut con, poll_id.clone(), user_id.clone(), name.clone()).await else {
        return;
    };

    let rooms = state.rooms.clone();
    let mut vote = rooms
        .add_client(
            poll_id.clone(),
            RoomClient {
                id: user_id.clone(),
                addr: addr.to_string(),
                name: name.clone(),
                join_time: chrono::Utc::now().timestamp(),
            },
        )
        .await;

    // Now send the "joined" message to all subscribers.
    let msg = WebSocketEvent::PollUpdated(Box::new(poll)).message();
    vote.broadcast(msg);

    // By splitting, we can send and receive at the same time.
    let (sender, mut receiver) = socket.split();
    let sender = Arc::new(Mutex::new(RefCell::new(sender)));
    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    let mut rx = vote.subscribe();

    let server_sender = sender.clone();
    // server -> client
    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if !send_message(server_sender.clone(), msg).await {
                break;
            }
        }
    });

    // notify
    // let mut rooms = state.rooms.clone();
    let room_id_clone = poll_id.clone();
    let mut rx = state.notify_tx.subscribe();
    let mut notify_task = tokio::spawn(async move {
        while let Ok(room_id) = rx.recv().await {
            if room_id_clone == room_id {
                break;
            }
        }

        // TODO: how to delete room cache
        rooms.remove(room_id_clone).await;
    });

    let client_sender = sender.clone();
    // Clone things we want to pass (move) to the receiving task.
    let mut tx = vote.clone();
    let poll_id = poll_id.clone();
    let rooms = state.rooms.clone();
    // client -> server
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Close(_) => {
                    rooms.remove_client(poll_id.clone(), user_id.clone()).await;
                    // listen client exit
                    let Ok(poll) = polls::remove_participant(&mut con, poll_id.clone(), user_id.clone()).await else {
                        // just exit if err
                        break;
                    };
                    let message = WebSocketEvent::PollUpdated(Box::new(poll)).message();
                    tx.broadcast(message);
                    break;
                }
                Message::Text(text) => {
                    let event: WebSocketEvent = text.into();
                    let event: Result<Poll, Error> = match event {
                        WebSocketEvent::RemoveParticipant(user_id) => {
                            polls::remove_participant(&mut con, poll_id.clone(), user_id).await
                        }

                        WebSocketEvent::Nomination(nomination) => {
                            if let Err(err) = nomination.validate() {
                                Err(Error::ValidationError(err))
                            } else {
                                let nomination_id = create_nomination_id();
                                rooms
                                    .add_nomination(poll_id.clone(), nomination_id.clone())
                                    .await;
                                let nomination = Nomination {
                                    text: nomination.text,
                                    user_id: user_id.clone(),
                                };
                                polls::add_nomination(
                                    &mut con,
                                    poll_id.clone(),
                                    nomination_id,
                                    nomination,
                                )
                                .await
                            }
                        }
                        WebSocketEvent::RemoveNomination(nomination_id) => {
                            rooms
                                .remove_nomination(poll_id.clone(), nomination_id.clone())
                                .await;
                            polls::remove_nomination(&mut con, poll_id.clone(), nomination_id).await
                        }
                        WebSocketEvent::StartVote => {
                            start_vote(&mut con, poll_id.clone(), user_id.clone()).await
                        }
                        WebSocketEvent::SubmitRankings(rankings) => {
                            submit_rankings(
                                &mut con,
                                poll_id.clone(),
                                user_id.clone(),
                                rooms.clone(),
                                rankings,
                            )
                            .await
                        }

                        WebSocketEvent::ClosePoll => {
                            close_poll(&mut con, poll_id.clone(), user_id.clone()).await
                        }
                        WebSocketEvent::CancelPoll => {
                            cancel_poll(&mut con, poll_id.clone(), user_id.clone()).await
                        }
                        _ => Err(Error::UnsupportedWebsocketEvent),
                    };
                    if let Err(err) = event {
                        match err {
                            Error::PollCancelled => {
                                let message = WebSocketEvent::PollCancelled.message();
                                tx.broadcast(message);

                                // we're completed this vote
                                // delete room info
                                rooms.remove(poll_id.clone()).await;
                                break;
                            }
                            _ => {
                                let message = WebSocketEvent::Exception(err.to_string()).message();
                                if !send_message(client_sender.clone(), message).await {
                                    break;
                                }
                            }
                        }
                        continue;
                    }
                    let poll = event.unwrap();
                    let message = WebSocketEvent::PollUpdated(Box::new(poll)).message();
                    tx.broadcast(message);
                }
                _ => {}
            }
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => {
            notify_task.abort();
            recv_task.abort()
        }
        _ = (&mut recv_task) => {
            notify_task.abort();
            send_task.abort()
        },
        _ = (&mut notify_task) => {
            recv_task.abort();
            send_task.abort()
        }
    };
}

async fn cancel_poll<C>(con: &mut C, poll_id: String, user_id: String) -> Result<Poll, Error>
where
    C: ConnectionLike,
{
    let poll = polls::get_poll(con, poll_id.clone()).await?;
    if poll.admin_id != user_id {
        return Err(Error::AdminPrivilegesRequired);
    }
    polls::del_poll(con, poll_id.clone()).await?;
    Err(Error::PollCancelled)
}

async fn close_poll<C>(con: &mut C, poll_id: String, user_id: String) -> Result<Poll, Error>
where
    C: ConnectionLike,
{
    let poll = polls::get_poll(con, poll_id.clone()).await?;
    if poll.admin_id != user_id {
        return Err(Error::AdminPrivilegesRequired);
    }
    let results = poll.get_results();
    polls::add_results(con, poll_id.clone(), results).await
}

async fn start_vote<C>(con: &mut C, poll_id: String, user_id: String) -> Result<Poll, Error>
where
    C: ConnectionLike,
{
    let poll = polls::get_poll(con, poll_id.clone()).await?;
    if poll.admin_id != user_id {
        return Err(Error::AdminPrivilegesRequired);
    }
    if poll.nominations.is_empty() {
        return Err(Error::NoNomination);
    }
    polls::start_poll(con, poll_id.clone()).await
}

async fn submit_rankings<C>(
    con: &mut C,
    poll_id: String,
    user_id: String,
    rooms: Arc<Rooms>,
    rankings: RankingList,
) -> Result<Poll, Error>
where
    C: ConnectionLike,
{
    if !rooms
        .contains_nomination(poll_id.clone(), rankings.clone())
        .await
    {
        return Err(Error::UnknownNomination);
    }
    polls::add_participant_rankings(con, poll_id, user_id, rankings).await
}

async fn send_message(
    sender: Arc<Mutex<RefCell<SplitSink<WebSocket, Message>>>>,
    message: String,
) -> bool {
    sender
        .lock()
        .await
        .get_mut()
        .send(Message::Text(message))
        .await
        .is_ok()
}
