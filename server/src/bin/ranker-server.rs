use axum::{routing::post, Router};
use serde::Deserialize;
use server::handlers::polls;
use std::net::SocketAddr;
use tokio::signal;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server_http_port: u16,
    pub client_port: u16,
    pub redis_host: String,
    pub redis_port: String,
    pub poll_duration: usize,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("cannot find .env file!");
    let Ok(config) = envy::prefixed("RANKER_").from_env::<Config>() else {
        panic!("config file error")
    };

    dbg!(&config);

    // build our application with a route
    let app = Router::new().nest(
        "/api",
        Router::new().nest(
            "/polls",
            Router::new()
                .route("/", post(polls::add))
                .route("/join", post(polls::join))
                .route("/rejoin", post(polls::rejoin)),
        ),
    );

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
