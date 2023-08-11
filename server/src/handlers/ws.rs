use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State, TypedHeader,
    },
    headers::{self, authorization::Bearer},
    response::IntoResponse,
    Extension,
};
use redis::aio::ConnectionManager;
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
    data::redis::polls,
    ids::create_nomination_id,
    models::{
        authed::{self, Authed},
        room::RoomClient,
        Nomination, Poll, WebSocketEvent,
    },
    state::AppState,
    Error,
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
    let Ok(auth) = authed::verify(token.to_string()) else {
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

    let mut rooms = state.rooms.clone();

    rooms
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
    let _ = state.tx.send(msg);

    // By splitting, we can send and receive at the same time.
    let (sender, mut receiver) = socket.split();
    let sender = Arc::new(Mutex::new(RefCell::new(sender)));
    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    let mut rx = state.tx.subscribe();

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

    let client_sender = sender.clone();
    // Clone things we want to pass (move) to the receiving task.
    let tx = state.tx.clone();
    let poll_id = poll_id.clone();
    let mut rooms = state.rooms.clone();
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
                    let _ = tx.send(message);
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
                            polls::remove_nomination(&mut con, poll_id.clone(), nomination_id).await
                        }
                        WebSocketEvent::StartVote => {
                            polls::start_poll(&mut con, poll_id.clone()).await
                        }
                        WebSocketEvent::SubmitRankings(rankings) => {
                            polls::add_participant_rankings(
                                &mut con,
                                poll_id.clone(),
                                user_id.clone(),
                                *rankings,
                            )
                            .await
                        }

                        WebSocketEvent::ClosePoll => {
                            let poll_res = polls::get_poll(&mut con, poll_id.clone()).await;
                            if let Err(err) = poll_res {
                                Err(err)
                            } else {
                                let poll = poll_res.unwrap();
                                if poll.admin_id != user_id {
                                    Err(Error::AdminPrivilegesRequired)
                                } else {
                                    let results = poll.get_results();
                                    polls::add_results(&mut con, poll_id.clone(), results).await
                                }
                            }
                        }
                        WebSocketEvent::CancelPoll => {
                            let poll_res = polls::get_poll(&mut con, poll_id.clone()).await;
                            if let Err(err) = poll_res {
                                Err(err)
                            } else {
                                let poll = poll_res.unwrap();
                                if poll.admin_id != user_id {
                                    Err(Error::AdminPrivilegesRequired)
                                } else {
                                    let poll_res = polls::del_poll(&mut con, poll_id.clone()).await;
                                    if let Err(err) = poll_res {
                                        Err(err)
                                    } else {
                                        Err(Error::PollCancelled)
                                    }
                                }
                            }
                        }
                        _ => Err(Error::UnsupportedWebsocketEvent),
                    };
                    if let Err(err) = event {
                        match err {
                            Error::PollCancelled => {
                                let message = WebSocketEvent::PollCancelled.message();
                                let _ = tx.send(message);

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
                    let _ = tx.send(message);
                }
                _ => {}
            }
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    let msg = format!("{} left.", name);
    let _ = state.tx.send(msg.to_string());

    println!("{msg}");
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
