use std::thread::sleep;
use std::time::Duration;

use redis_test::MockCmd;

#[tokio::test]
async fn test_redis() {
    MockCmd::new::<&mut redis::Cmd, &str>(
        redis::cmd("ZSCORE").arg("foo").arg("bar"),
        Err(redis::RedisError::from((
            redis::ErrorKind::ClientError,
            "TEST",
            "mock error".to_string(),
        ))),
    );
    // Get after expiry time mustn't return value
    sleep(Duration::from_secs(1));
}
