use crate::UnifyResponse;
use axum::{
    extract::rejection::FormRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
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

    #[error("Poll cancelled")]
    PollCancelled,

    #[error("Poll no start")]
    PollNoStart,

    #[error("Poll has started")]
    PollHasStarted,

    #[error("Wrong credentials")]
    WrongCredentials,

    #[error("Missing credentials")]
    MissingCredentials,

    #[error("Token creation")]
    TokenCreation,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Deserialize websocket event error")]
    DeserializeWebsocketEventError,

    #[error("Unsupported websocket event")]
    UnsupportedWebsocketEvent,

    #[error("Admin privileges required")]
    AdminPrivilegesRequired,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status_code, err_code, error_message) = match self {
            Error::ValidationJsonError => {
                let message = "Input validation json error".to_string();
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
                println!("{}", message);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    500,
                    "Internal server error".to_string(),
                )
            }
            Error::PollNotFound => (StatusCode::BAD_REQUEST, 500, "Poll not found".to_string()),

            Error::WrongCredentials => (
                StatusCode::UNAUTHORIZED,
                600,
                "Wrong credentials".to_string(),
            ),
            Error::MissingCredentials => (
                StatusCode::BAD_REQUEST,
                700,
                "Missing credentials".to_string(),
            ),
            Error::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                800,
                "Token creation error".to_string(),
            ),
            Error::InvalidToken => (StatusCode::BAD_REQUEST, 900, "Invalid token".to_string()),

            Error::PollNoStart => (StatusCode::BAD_REQUEST, 1000, "Poll no start".to_string()),
            Error::PollHasStarted => (
                StatusCode::BAD_REQUEST,
                1100,
                "Poll has started".to_string(),
            ),
            Error::DeserializeWebsocketEventError => (
                StatusCode::BAD_REQUEST,
                1200,
                "Deserialize Websocket event error".to_string(),
            ),
            Error::PollCancelled => (StatusCode::BAD_REQUEST, 1300, "Poll cancelled".to_string()),
            Error::UnsupportedWebsocketEvent => (
                StatusCode::BAD_REQUEST,
                1400,
                "Unsupported websocket event".to_string(),
            ),

            Error::AdminPrivilegesRequired => (
                StatusCode::BAD_REQUEST,
                1500,
                "Admin privileges required".to_string(),
            ),
        };
        (
            status_code,
            UnifyResponse::<()>::err(err_code, error_message).json(),
        )
            .into_response()
    }
}
