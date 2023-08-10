use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State, TypedHeader,
    },
    headers::{self, authorization::Bearer},
    response::IntoResponse,
};

use std::net::SocketAddr;
use std::sync::Arc;

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;

//allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};

use crate::{
    models::authed::{self, Authed},
    state::AppState,
    Error,
};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
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
    println!("`{}` at {addr} connected.", auth.sub);

    ws.on_upgrade(move |socket| handle_socket(socket, addr, auth, state))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(socket: WebSocket, who: SocketAddr, auth: Authed, state: Arc<AppState>) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = socket.split();

    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    let mut rx = state.tx.subscribe();

    // Now send the "joined" message to all subscribers.
    let msg = format!("{} joined.", auth.name);
    let _ = state.tx.send(msg);

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

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Close(_) => {
                    // listen client exit
                    break;
                }
                Message::Text(text) => {
                    let _ = tx.send(format!("IP[{}]: {}", who, text));
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

    let msg = format!("{} left.", who);
    let _ = state.tx.send(msg.to_string());

    println!("{msg}");
}
