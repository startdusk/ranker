use crate::Error;
use async_trait::async_trait;
use axum::extract::{FromRequestParts, Query};
use axum::http::request::Parts;
use axum::Extension;
use hyper::http;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use tower_cookies::Cookies;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("RANKER_JWT_SECRET").expect("RANKER_JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authed {
    pub poll_id: String,
    pub name: String,

    pub sub: String,
    pub company: String,
    pub exp: usize,
}

impl Authed {
    pub fn is_expired(&self) -> bool {
        self.exp <= chrono::Utc::now().timestamp() as usize
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Authed
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let already_authed = parts.extensions.get::<Authed>();
        if let Some(authed) = already_authed {
            return Ok(authed.clone());
        } else {
            let already_tokened = parts.extensions.get::<Tokened>();
            let token_o = if let Some(token) = already_tokened {
                Some(token.token.clone())
            } else {
                extract_token(parts, state).await
            };
            let Some(token) = token_o else {
				return Err(Error::MissingCredentials)
			};

            verify(token)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tokened {
    pub token: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for Tokened
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let already_tokened = parts.extensions.get::<Tokened>();
        if let Some(tokened) = already_tokened {
            Ok(tokened.clone())
        } else {
            let token_o = extract_token(parts, state).await;
            if let Some(token) = token_o {
                let tokened = Self { token };
                parts.extensions.insert(tokened.clone());
                Ok(tokened)
            } else {
                Err(Error::WrongCredentials)
            }
        }
    }
}

const COOKIE_NAME: &str = "token";

async fn extract_token<S: Send + Sync>(parts: &mut Parts, state: &S) -> Option<String> {
    // 1.from headers
    let auth_header = parts
        .headers
        .get(http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    // 2.from cookie
    let from_cookie = match auth_header {
        Some(x) => Some(x.to_owned()),
        None => Extension::<Cookies>::from_request_parts(parts, state)
            .await
            .ok()
            .and_then(|cookies| cookies.get(COOKIE_NAME).map(|c| c.value().to_owned())),
    };

    #[derive(Deserialize)]
    struct Token {
        token: Option<String>,
    }
    match from_cookie {
        Some(token) => Some(token),
        None => Query::<Token>::from_request_parts(parts, state)
            .await
            .ok()
            .and_then(|token| token.token.clone()),
    }
}

pub fn token_gen(
    poll_id: String,
    user_id: String,
    name: String,
    expire_time: usize,
) -> Result<String, Error> {
    let claims = Authed {
        poll_id,
        name,
        sub: user_id,
        company: "Ranker Inc.".to_string(),
        exp: (chrono::Utc::now().timestamp() + expire_time as i64) as usize,
    };
    let token =
        encode(&Header::default(), &claims, &KEYS.encoding).map_err(|_| Error::TokenCreation)?;

    Ok(token)
}

pub fn verify(token: String) -> Result<Authed, Error> {
    let token_data = decode::<Authed>(&token, &KEYS.decoding, &Validation::default())
        .map_err(|_| Error::InvalidToken)?;
    let authed = token_data.claims;
    if authed.is_expired() {
        return Err(Error::WrongCredentials);
    }
    Ok(authed)
}
