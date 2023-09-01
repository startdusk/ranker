use axum::{
    http::{HeaderValue, Method, Request},
    Extension, Router,
};
use server::{
    data::redis::{polls::POLL_KEY_PREFIX, redis_keyspace_notifications},
    handlers::not_found,
    models::room::Rooms,
    services::{polls, ws},
    state::{AppState, EnvConfig},
};
use std::{net::SocketAddr, sync::Arc};
use tokio::{signal, sync::broadcast};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, info_span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let config = envy::prefixed("RANKER_").from_env::<EnvConfig>()?;

    let redis_url = config.redis_url.clone();
    let client = redis::Client::open(redis_url)?;
    let redis_mgr = redis::aio::ConnectionManager::new(client.clone()).await?;

    let middleware_stack = ServiceBuilder::new().layer(Extension(redis_mgr));

    let (notify_tx, _rx) = broadcast::channel(100);
    let rooms = Arc::new(Rooms::default());
    let app_state = Arc::new(AppState {
        env: config.clone(),
        rooms: rooms.clone(),
        notify_tx: notify_tx.clone(),
    });

    let client_allow_origin = format!("{}:{}", config.client_domain, config.client_port);
    let cors_layer = CorsLayer::new()
        .allow_origin(client_allow_origin.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST]);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "ranker-server=info,tower_http=info,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with a route
    let app = Router::new()
        .nest(
            "/api",
            Router::new().nest("/polls", polls::service(app_state.clone())),
        )
        .nest("/polls", ws::service(app_state))
        .fallback(not_found::handler_404)
        .layer(middleware_stack)
        .layer(cors_layer)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request.uri().to_string();

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }), // .on_request(|_request: &Request<_>, _span: &Span| {
                //     // You can use `_span.record("some_other_field", value)` in one of these
                //     // closures to attach a value to the initially empty field in the info_span
                //     // created above.
                // })
                // .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                //     // ...
                // })
                // .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                //     // ...
                // })
                // .on_eos(
                //     |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                //         // ...
                //     },
                // )
                // .on_failure(
                //     |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                //         // ...
                //     },
                // ),
        );

    let notifier = async {
        redis_keyspace_notifications(client, |mut key| {
            let key = key.split_off(POLL_KEY_PREFIX.len());
            let _ = notify_tx.send(key);
        })
        .await?;

        Ok(()) as anyhow::Result<()>
    };

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_http_port));
    let server = async {
        info!("listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .with_graceful_shutdown(shutdown_signal())
            .await?;

        Ok(()) as anyhow::Result<()>
    };

    tokio::select! {
        res = server => {
            Ok(res?)
        }
        res = notifier => {
            Ok(res?)
        }
    }
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

    info!("signal received, starting graceful shutdown");
}
