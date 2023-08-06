use crate::UnifyResponse;
use axum::{
    extract::rejection::FormRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),

    #[error("Invalid json")]
    ValidationJsonError,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status_code, err_code, error_message) = match self {
            ServerError::ValidationJsonError => {
                let message = format!("Input validation json error");
                (StatusCode::BAD_REQUEST, 100, message)
            }
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{}]", self).replace('\n', ", ");
                (StatusCode::BAD_REQUEST, 200, message)
            }
            ServerError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, 300, self.to_string()),
        };
        (
            status_code,
            Json(UnifyResponse::<()>::err(err_code, error_message)),
        )
            .into_response()
    }
}
