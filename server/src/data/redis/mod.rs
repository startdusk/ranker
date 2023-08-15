use redis::{cmd, Client};

pub mod polls;

// Redis keyspace notifications: https://redis.io/docs/manual/keyspace-notifications/
// github.com/Nurrl/shrekd/blob/main/src/main.rs#L79

pub async fn redis_keyspace_notifications<F>(client: Client, f: F) -> anyhow::Result<()>
where
    F: Fn(String),
{
    /*! Listen for `del` and `expired` Redis keyspace events to cleanup expired files */
    use futures::StreamExt;

    let mut conn = client.get_async_connection().await?;

    /* Enable keyspace events in the redis server */
    cmd("CONFIG")
        .arg("SET")
        .arg("notify-keyspace-events")
        .arg("Egx") /* `Egx` means E: keyevent events, with types g: general and x: expired */
        .query_async(&mut conn)
        .await?;

    /* Subscribe to the relevant events */
    let mut pubsub = client.get_async_connection().await?.into_pubsub();
    pubsub.psubscribe("__keyevent@0__:expired").await?;
    pubsub.psubscribe("__keyevent@0__:del").await?;

    let mut events = pubsub.into_on_message();

    loop {
        let Some(msg) = events.next().await else {
			continue
   	    };

        /* Retrieve the key, and split it into prefix and slug */
        let key: String = msg.get_payload()?;

        f(key)
    }
}
