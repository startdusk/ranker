use std::sync::Arc;

use axum::{middleware::from_extractor, routing::post, Router};
use tower::ServiceBuilder;

use crate::{
    handlers::polls,
    models::authed::{Authed, Tokened},
    state::AppState,
};

pub fn service(state: Arc<AppState>) -> Router {
    let middleware_stack = ServiceBuilder::new()
        .layer(from_extractor::<Tokened>())
        .layer(from_extractor::<Authed>());
    Router::new()
        .route("/", post(polls::add))
        .route("/join", post(polls::join))
        .route("/rejoin", post(polls::rejoin).route_layer(middleware_stack))
        .with_state(state)
}
