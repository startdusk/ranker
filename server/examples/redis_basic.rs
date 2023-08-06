use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Get after expiry time mustn't return value
    sleep(Duration::from_secs(1));
}
