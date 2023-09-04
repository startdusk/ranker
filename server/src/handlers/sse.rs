use std::convert::Infallible;
use std::sync::Arc;

use async_stream::try_stream;
use axum::extract::State;
use axum::response::sse::{Event, KeepAlive, Sse};
use futures::Stream;

use crate::state::AppState;

pub async fn event_stream(
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
