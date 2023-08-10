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

use std::net::SocketAddr;
use std::sync::Arc;

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;

//allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};

use crate::{
    data::redis::polls,
    models::{
        authed::{self, Authed},
        WebSocketEvent,
    },
    state::AppState,
    Error,
};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(con): Extension<ConnectionManager>,
    State(state): State<Arc<AppState>>,
    authorization: Option<TypedHeader<headers::Authorization<Bearer>>>,
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let Some(TypedHeader(token)) = authorization else {
        return Error::MissingCredentials.into_response()
    };

    let token = token.0.token();
    let Ok(auth) = authed::verify(token.to_string()) else {
        return Error::InvalidToken.into_response();
    };
    // println!("`{}` at {addr} connected.", auth.sub);
    let con = con.clone();
    ws.on_upgrade(move |socket| handle_socket(socket, con, auth, state))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(
    socket: WebSocket,
    mut con: ConnectionManager,
    auth: Authed,
    state: Arc<AppState>,
) {
    let user_id = auth.sub.clone();
    let poll_id = auth.poll_id.clone();
    let name = auth.name;

    let Ok(poll) =
        polls::add_participant(&mut con, poll_id.clone(), user_id.clone(), name.clone()).await else {
        return;
    };

    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = socket.split();

    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    let mut rx = state.tx.subscribe();

    // Now send the "joined" message to all subscribers.
    let msg = WebSocketEvent::PollUpdated(poll).message();
    let _ = state.tx.send(msg);

    // server -> client
    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Clone things we want to pass (move) to the receiving task.
    let tx = state.tx.clone();

    // client -> server
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Close(_) => {
                    // listen client exit
                    let Ok(poll) = polls::remove_participant(&mut con, poll_id, user_id).await else {
                        // just exit if err
                        break;
                    };
                    let message = WebSocketEvent::PollUpdated(poll).message();
                    let _ = tx.send(message);
                    break;
                }
                Message::Text(text) => {
                    let event: WebSocketEvent = text.into();
                    // TODO: handle this event
                    // let message = match event {
                    //     WebSocketEvent::CancelPoll =>
                    // };
                    let _ = tx.send(event.message());
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
