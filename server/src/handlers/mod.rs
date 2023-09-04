use crate::errors::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use axum::Json;
use serde::Serialize;

pub mod not_found;
pub mod polls;
pub mod sse;
pub mod ws;

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

            // websocket error!
            _ => unreachable!(),
        };
        (
            status_code,
            UnifyResponse::<()>::err(err_code, error_message).json(),
        )
            .into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct UnifyResponse<T> {
    pub code: i32,
    pub message: String,
    pub success: bool,
    pub data: Option<T>,
}

impl<T> UnifyResponse<T>
where
    T: Serialize,
{
    pub fn new(code: i32, message: String, success: bool, data: Option<T>) -> Self {
        Self {
            code,
            message,
            success,
            data,
        }
    }
    pub fn ok(data: Option<T>) -> Self {
        Self::new(0, "OK".to_string(), true, data)
    }

    pub fn err(code: i32, message: String) -> Self {
        Self::new(code, message, false, None)
    }

    pub fn json(self) -> Json<Self> {
        Json(self)
    }
}
