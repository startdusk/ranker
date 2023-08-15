use async_trait::async_trait;
use axum::{extract::FromRequest, http::Request, Json, RequestExt};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::errors::Error;

#[derive(Debug, Clone, Copy, Default)]
pub struct Input<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for Input<T>
where
    T: DeserializeOwned + Validate + Send + 'static,
    S: Send + Sync,
    Json<T>: FromRequest<(), B>,
    B: Send + 'static,
{
    type Rejection = Error;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = req
            .extract::<Json<T>, _>()
            .await
            .map_err(|_| Error::ValidationJsonError)?;
        value.validate()?;
        Ok(Input(value))
    }
}
