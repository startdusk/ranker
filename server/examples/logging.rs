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
use tower_http::{
    trace::{self, TraceLayer},
    ServiceBuilderExt,
};
use tracing::Level;
use tracing_appender::rolling;

#[tokio::main]
async fn main() {
    let info_file = rolling::daily("./logs", "log");
    tracing_subscriber::fmt()
        .with_writer(info_file)
        .with_target(false)
        .json()
        .init();

    let app = Router::new().route("/", post(handler)).layer(
        ServiceBuilder::new()
            .map_request_body(body::boxed)
            .layer(middleware::from_fn(log_request_and_response_body))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                    .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
                    .on_body_chunk(trace::DefaultOnBodyChunk::new())
                    .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
            ),
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

    let bytes = buffer_and_print("request ", body).await?;

    let req = Request::from_parts(parts, body::boxed(Full::from(bytes)));
    let res = next.run(req).await;

    let (parts, body) = res.into_parts();

    let bytes = buffer_and_print("response", body).await?;

    let res = Response::from_parts(parts, Body::from(bytes));
    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, Response>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = hyper::body::to_bytes(body)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?;
    tracing_body(direction, bytes.clone());
    Ok(bytes)
}

fn tracing_body(direction: &str, body: Bytes) {
    match serde_json::from_slice::<serde_json::Value>(&body) {
        Ok(body) => {
            tracing::info!("{} body = {:?}", direction, body);
        }
        Err(_) => {
            tracing::info!("{} body = {:?}", direction, body)
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
