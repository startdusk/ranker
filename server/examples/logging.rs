use axum::{
    body::{self, Body, BoxBody, Bytes, Full},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::Serialize;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
use tracing_appender::rolling;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let info_file = rolling::daily("./logs", "log");
    // tracing_subscriber::fmt()
    //     .without_time() // For early local development.
    //     .with_writer(info_file)
    //     .with_env_filter(
    //         tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
    //             // axum logs rejections from built-in extractors with the `axum::rejection`
    //             // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
    //             "logging=info,tower_http=info,axum::rejection=trace".into()
    //         }),
    //     )
    //     .json()
    //     .with_level(true)
    //     .with_target(true)
    //     .init();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "logging=info,tower_http=info,axum::rejection=trace".into()
            }),
        )
        .with(
            tracing_subscriber::fmt::layer()
                // .with_span_list(false)
                // .with_current_span(true)
                .with_writer(info_file)
                .json()
                .flatten_event(true)
                .with_ansi(false),
        )
        .init();

    let app = Router::new().route("/", post(handler)).layer(
        ServiceBuilder::new()
            .map_request_body(body::boxed)
            .layer(middleware::from_fn(log_request_and_response_body)),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8888));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// middleware that shows how to consume the request body upfront
async fn log_request_and_response_body(
    req: Request<BoxBody>,
    next: Next<BoxBody>,
) -> Result<impl IntoResponse, Response> {
    let (parts, body) = req.into_parts();

    let request_id = Uuid::new_v4();
    let bytes = buffer_and_print(request_id, "request ", body).await?;

    let req = Request::from_parts(parts, body::boxed(Full::from(bytes)));
    let res = next.run(req).await;

    let (parts, body) = res.into_parts();

    let bytes = buffer_and_print(request_id, "response", body).await?;

    let res = Response::from_parts(parts, Body::from(bytes));
    Ok(res)
}

async fn buffer_and_print<B>(request_id: Uuid, direction: &str, body: B) -> Result<Bytes, Response>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = hyper::body::to_bytes(body)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?;
    tracing_body(request_id, direction, bytes.clone());
    Ok(bytes)
}

fn tracing_body(request_id: Uuid, direction: &str, body: Bytes) {
    let print_type = format!("{direction} body");
    match serde_json::from_slice::<serde_json::Value>(&body) {
        Ok(body) => {
            let body = body.to_string();
            tracing::info!(request_id=%request_id, print_type=%print_type, body);
        }
        Err(_) => {
            let body = format!("{:?}", body);
            tracing::info!(request_id=%request_id, print_type=%print_type, body)
        }
    }
}

#[derive(Debug, Serialize)]
struct JsonResponse {
    message: String,
    status: u16,
    code: usize,
}

async fn handler() -> impl IntoResponse {
    Json(JsonResponse {
        message: "return a json".to_string(),
        status: 200,
        code: 1200,
    })
}
