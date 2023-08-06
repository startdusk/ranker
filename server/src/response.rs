use async_trait::async_trait;
use axum::{extract::FromRequest, http::Request, Json, RequestExt};
use serde::{de::DeserializeOwned, Serialize};
use validator::Validate;

use crate::ServerError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedInput<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedInput<T>
where
    T: DeserializeOwned + Validate + Send + 'static,
    S: Send + Sync,
    Json<T>: FromRequest<(), B>,
    B: Send + 'static,
{
    type Rejection = ServerError;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = req
            .extract::<Json<T>, _>()
            .await
            .map_err(|_| ServerError::ValidationJsonError)?;
        value.validate()?;
        Ok(ValidatedInput(value))
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
    pub fn ok(code: i32, data: Option<T>) -> Self {
        Self::new(code, "OK".to_string(), true, data)
    }

    pub fn err(code: i32, message: String) -> Self {
        Self::new(code, message, false, None)
    }
}
