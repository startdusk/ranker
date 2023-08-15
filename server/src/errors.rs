use axum::extract::rejection::FormRejection;
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

    #[error("Unknown nomination")]
    UnknownNomination,

    #[error("No nomination")]
    NoNomination,
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // They have same type
            (Self::ValidationError(_l0), Self::ValidationError(_r0)) => true,
            (Self::AxumFormRejection(_l0), Self::AxumFormRejection(_r0)) => true,
            (Self::DeserializeJsonError(_l0), Self::DeserializeJsonError(_r0)) => true,
            (Self::RedisError(l0), Self::RedisError(r0)) => l0.kind() == r0.kind(),

            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
