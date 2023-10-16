use async_stream::try_stream;
use axum::{
    extract::State,
    http::HeaderValue,
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
    routing::get,
    Router,
};

use futures::Stream;
use hyper::Method;
use serde::Serialize;
use std::{convert::Infallible, net::SocketAddr, sync::Arc};
use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};
use tower_http::cors::CorsLayer;

use nanoid::nanoid;

#[derive(Debug, Serialize, Clone)]
struct Notification {
    pub notify_type: NotifyType,
    pub username: String,
    pub poll_id: String,
    pub topic: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
enum NotifyType {
    JoinPoll,
}

#[derive(Debug, Clone)]
struct AppState {
    pub sse_tx: broadcast::Sender<Notification>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (sse_tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState {
        sse_tx: sse_tx.clone(),
    });

    let sse_tx = sse_tx.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_millis(3000)).await;

            let _ = sse_tx.send(Notification {
                notify_type: NotifyType::JoinPoll,
                username: nanoid!(10),
                poll_id: nanoid!(6),
                topic: nanoid!(8),
            });
        }
    });
    let client_allow_origin = "http://localhost:8080".to_string();
    let cors_layer = CorsLayer::new()
        .allow_origin(client_allow_origin.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST]);
    let app = Router::new()
        .route("/sse", get(event_stream))
        .with_state(app_state)
        .layer(cors_layer);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn event_stream(
    State(state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let mut receiver = state.sse_tx.subscribe();

    Sse::new(try_stream! {
        loop {
            match receiver.recv().await {
                Ok(msg) => {
                    let event = Event::default().json_data(msg).unwrap();
                    yield event;
                },

                Err(e) => {
                    tracing::error!(error = ?e, "Failed to event stream");
                }
            }
        }
    })
    .keep_alive(KeepAlive::default())
}
