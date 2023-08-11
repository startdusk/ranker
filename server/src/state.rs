use serde::Deserialize;
use tokio::sync::broadcast;

use crate::models::room::Rooms;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server_http_port: u16,
    pub client_domain: String,
    pub client_port: u16,
    pub redis_host: String,
    pub redis_port: String,
    pub poll_duration: usize,
    pub jwt_secret: String,
}

#[derive(Clone)]
pub struct AppState {
    pub env: Config,
    // Channel used to send messages to all connected clients.
    pub tx: broadcast::Sender<String>,

    pub rooms: Rooms,
}
