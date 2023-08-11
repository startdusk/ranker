use axum::{
    http::{HeaderValue, Method},
    Extension, Router,
};
use server::{
    handlers::not_found,
    models::room::Rooms,
    services::{polls, ws},
    state::{AppState, Config},
};
use std::{net::SocketAddr, sync::Arc};
use tokio::{signal, sync::broadcast};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let Ok(config) = envy::prefixed("RANKER_").from_env::<Config>() else {
        panic!("env not set")
    };

    let redis_url = format!("redis://{}:{}/", config.redis_host, config.redis_port);
    let client = redis::Client::open(redis_url)?;
    let redis_mgr = redis::aio::ConnectionManager::new(client).await?;
    let (tx, _rx) = broadcast::channel(100);

    let middleware_stack = ServiceBuilder::new().layer(Extension(redis_mgr));

    let app_state = Arc::new(AppState {
        env: config.clone(),
        tx,
        rooms: Rooms::new(),
    });

    let client_allow_origin = format!("{}:{}", config.client_domain, config.client_port);
    let cors_layer = CorsLayer::new()
        .allow_origin(client_allow_origin.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST]);

    // build our application with a route
    let app = Router::new()
        .nest(
            "/api",
            Router::new().nest("/polls", polls::service(app_state.clone())),
        )
        .nest("/polls", ws::service(app_state))
        .fallback(not_found::handler_404)
        .layer(middleware_stack)
        .layer(cors_layer);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], config.server_http_port));
    println!("listening on {}", addr);
    hyper::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
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
