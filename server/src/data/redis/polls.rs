use redis::{aio::ConnectionLike, cmd};

use crate::{
    models::{Nomination, NominationID, Poll, Rankings, Results},
    Error,
};

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

pub async fn add_participant<C: ConnectionLike>(
    mut con: C,
    poll_id: String,
    user_id: String,
    name: String,
) -> Result<Poll, Error> {
    let key = make_key(poll_id.clone());
    let participant_path = make_participant_path(user_id);
    let name = serde_json::to_string(&name).unwrap();
    let _ = cmd("JSON.SET")
        .arg(&[key, participant_path, name])
        .query_async(&mut con)
        .await
        .map_err(Error::RedisError)?;

    Ok(get_poll(con, poll_id).await?)
}

pub async fn remove_participant<C: ConnectionLike>(
    mut con: C,
    poll_id: String,
    user_id: String,
) -> Result<Poll, Error> {
    let key = make_key(poll_id.clone());
    let participant_path = make_participant_path(user_id);

    let _ = cmd("JSON.DEL")
        .arg(&[key, participant_path])
        .query_async(&mut con)
        .await
        .map_err(Error::RedisError)?;

    Ok(get_poll(con, poll_id).await?)
}

pub async fn add_nomination<C: ConnectionLike>(
    mut con: C,
    poll_id: String,
    nomination_id: NominationID,
    nomination: Nomination,
) -> Result<Poll, Error> {
    let key = make_key(poll_id.clone());
    let nomination_path = make_nomination_path(nomination_id);
    let nomination = serde_json::to_string(&nomination).unwrap();
    let _ = cmd("JSON.SET")
        .arg(&[key, nomination_path, nomination])
        .query_async(&mut con)
        .await
        .map_err(Error::RedisError)?;

    Ok(get_poll(con, poll_id).await?)
}

pub async fn remove_nomination<C: ConnectionLike>(
    mut con: C,
    poll_id: String,
    nomination_id: NominationID,
) -> Result<Poll, Error> {
    let key = make_key(poll_id.clone());
    let nomination_path = make_nomination_path(nomination_id);
    let _ = cmd("JSON.DEL")
        .arg(&[key, nomination_path])
        .query_async(&mut con)
        .await
        .map_err(Error::RedisError)?;

    Ok(get_poll(con, poll_id).await?)
}

pub async fn start_poll<C: ConnectionLike>(mut con: C, poll_id: String) -> Result<Poll, Error> {
    let key = make_key(poll_id.clone());
    let started = true;
    let started = serde_json::to_string(&started).unwrap();
    let _ = cmd("JSON.SET")
        .arg(&[key, ".has_started".to_string(), started])
        .query_async(&mut con)
        .await
        .map_err(Error::RedisError)?;

    Ok(get_poll(con, poll_id).await?)
}

pub async fn add_participant_rankings<C: ConnectionLike>(
    mut con: C,
    poll_id: String,
    user_id: String,
    rankings: Rankings,
) -> Result<Poll, Error> {
    let key = make_key(poll_id.clone());
    let rankings_path = make_rankings_path(user_id);
    let rankings = serde_json::to_string(&rankings).unwrap();
    let _ = cmd("JSON.SET")
        .arg(&[key, rankings_path, rankings])
        .query_async(&mut con)
        .await
        .map_err(Error::RedisError)?;
    Ok(get_poll(con, poll_id).await?)
}

pub async fn add_results<C: ConnectionLike>(
    mut con: C,
    poll_id: String,
    results: Results,
) -> Result<Poll, Error> {
    let key = make_key(poll_id.clone());
    let results = serde_json::to_string(&results).unwrap();
    let _ = cmd("JSON.SET")
        .arg(&[key, ".results".to_string(), results])
        .query_async(&mut con)
        .await
        .map_err(Error::RedisError)?;
    Ok(get_poll(con, poll_id).await?)
}

pub async fn del_poll<C: ConnectionLike>(mut con: C, poll_id: String) -> Result<(), Error> {
    let key = make_key(poll_id);
    let _ = cmd("JSON.DEL")
        .arg(&[key])
        .query_async(&mut con)
        .await
        .map_err(Error::RedisError)?;
    Ok(())
}

fn make_key(poll_id: String) -> String {
    format!("polls:{}", poll_id)
}

fn make_participant_path(user_id: String) -> String {
    format!(".participants.{}", user_id)
}

fn make_nomination_path(nomination_id: NominationID) -> String {
    format!(".nominations.{}", nomination_id)
}

fn make_rankings_path(user_id: String) -> String {
    format!(".rankings.{}", user_id)
}
