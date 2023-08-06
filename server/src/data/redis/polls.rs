use redis::{aio::ConnectionLike, cmd};

use crate::{models::Poll, Error};

pub async fn add_poll<C: ConnectionLike>(
    mut con: C,
    ttl: usize,
    poll_id: String,
    topic: String,
    votes_per_voter: usize,
    user_id: String,
) -> Result<Poll, Error> {
    let key = make_key(poll_id.clone());
    let poll = Poll::new(poll_id, topic, votes_per_voter, user_id);
    let value = serde_json::to_string(&poll).unwrap();
    let _ = cmd("JSON.SET")
        .arg(&[key, ".".to_string(), value])
        .arg(&["expire".to_string(), ttl.to_string()])
        .query_async(&mut con)
        .await
        .map_err(Error::RedisError)?;
    Ok(poll)
}

pub async fn get_poll<C: ConnectionLike>(mut con: C, poll_id: String) -> Result<Poll, Error> {
    let key = make_key(poll_id);
    let poll_json: String = cmd("JSON.GET")
        .arg(&[key, ".".to_string()])
        .query_async(&mut con)
        .await
        .map_err(Error::RedisError)?;
    let poll: Poll = serde_json::from_str(&poll_json).map_err(Error::DeserializeJsonError)?;
    Ok(poll)
}

fn make_key(poll_id: String) -> String {
    format!("polls:{}", poll_id)
}
