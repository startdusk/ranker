use serde::Deserialize;
use tokio::sync::broadcast;

use crate::models::room::Rooms;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server_http_port: u16,
    pub client_domain: String,
    pub client_port: u16,
    pub redis_url: String,
    pub poll_duration: usize,
    pub jwt_secret: String,
}

#[derive(Clone)]
pub struct AppState {
    pub env: Config,

    pub rooms: Rooms,

    pub notify_tx: broadcast::Sender<String>,
}
