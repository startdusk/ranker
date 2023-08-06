use axum::{routing::post, Router};
use server::handlers::polls;
use std::net::SocketAddr;
use tokio::signal;

#[tokio::main]
async fn main() {
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
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
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
