use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{handlers::ws, state::AppState};

pub fn service(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(ws::ws_handler))
        .with_state(state)
}
