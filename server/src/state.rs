use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server_http_port: u16,
    pub client_port: u16,
    pub redis_host: String,
    pub redis_port: String,
    pub poll_duration: usize,
    pub jwt_secret: String,
}

pub struct AppState {
    pub env: Config,
}
