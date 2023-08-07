use crate::UnifyResponse;
use axum::{
    extract::rejection::FormRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),

    #[error("Invalid json")]
    ValidationJsonError,

    #[error(transparent)]
    DeserializeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    RedisError(#[from] redis::RedisError),

    #[error("Poll not found")]
    PollNotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status_code, err_code, error_message) = match self {
            Error::ValidationJsonError => {
                let message = format!("Input validation json error");
                (StatusCode::BAD_REQUEST, 100, message)
            }
            Error::ValidationError(_) => {
                let message = format!("Input validation error: [{}]", self).replace('\n', ", ");
                (StatusCode::BAD_REQUEST, 200, message)
            }
            Error::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, 300, self.to_string()),
            Error::DeserializeJsonError(_) => {
                let message = format!("Deserialize json error: {}", self);
                (StatusCode::BAD_REQUEST, 400, message)
            }
            Error::RedisError(_) => {
                let message = format!("Redis error: {}", self);
                // TODO: log this error
                dbg!(message);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    500,
                    "Internal server error".to_string(),
                )
            }
            Error::PollNotFound => {
                let message = format!("Error: {}", self);
                (StatusCode::BAD_REQUEST, 400, message)
            }
        };
        (
            status_code,
            Json(UnifyResponse::<()>::err(err_code, error_message)),
        )
            .into_response()
    }
}
