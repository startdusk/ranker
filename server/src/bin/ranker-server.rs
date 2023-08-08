use axum::{middleware, routing::post, Extension, Router};
use server::{
    handlers::{not_found, polls},
    middlewares::jwt,
    state::{AppState, Config},
};
use std::{net::SocketAddr, sync::Arc};
use tokio::signal;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let Ok(config) = envy::prefixed("RANKER_").from_env::<Config>() else {
        panic!("config file error")
    };

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut redis_mgr = redis::aio::ConnectionManager::new(client).await.unwrap();
    let app_state = Arc::new(AppState { env: config });
    // build our application with a route
    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .nest(
                    "/polls",
                    Router::new()
                        .route("/", post(polls::add))
                        .route("/join", post(polls::join))
                        .route("/rejoin", post(polls::rejoin))
                        .route_layer(middleware::from_fn_with_state(app_state, jwt::auth)),
                )
                .with_state(app_state),
        )
        .fallback(not_found::handler_404);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], config.server_http_port));
    println!("listening on {}", addr);
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
