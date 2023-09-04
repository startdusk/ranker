use std::sync::Arc;

use serde::Deserialize;
use tokio::sync::broadcast;

use crate::models::{room::Rooms, Notification};

#[derive(Debug, Clone, Deserialize)]
pub struct EnvConfig {
    pub server_http_port: u16,
    pub client_domain: String,
    pub client_port: u16,
    pub redis_url: String,
    pub poll_duration: usize,
    pub jwt_secret: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: EnvConfig,

    pub rooms: Arc<Rooms>,

    pub notify_tx: broadcast::Sender<String>,

    pub sse_tx: broadcast::Sender<Notification>,
}
