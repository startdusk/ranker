use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, Request},
    middleware::Next,
    response::IntoResponse,
};

use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::{state::AppState, Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub exp: usize,

    pub poll_id: String,
    pub name: String,
}

impl TokenClaims {
    pub fn is_expired(&self) -> bool {
        self.exp >= chrono::Utc::now().timestamp() as usize
    }
}

// guard types
#[derive(Debug)]
pub struct AuthPayload {
    pub user_id: String,
    pub poll_id: String,
    pub name: String,
}

pub async fn auth<B>(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, Error> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    let token = token.ok_or_else(|| Error::MissingCredentials)?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| Error::InvalidToken)?
    .claims;

    if claims.is_expired() {
        return Err(Error::InvalidToken);
    }

    let auth_payload = AuthPayload {
        user_id: claims.sub,
        poll_id: claims.poll_id,
        name: claims.name,
    };

    req.extensions_mut().insert(auth_payload);
    Ok(next.run(req).await)
}
