use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{handlers::sse, state::AppState};

pub fn service(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(sse::event_stream))
        .with_state(state)
}
