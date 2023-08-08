use redis::{aio::ConnectionManager, cmd};

use crate::{
    models::{Nomination, NominationID, Poll, Rankings, Results},
    Error,
};

pub async fn add_poll(
    con: &mut ConnectionManager,
    ttl: usize,
    poll_id: String,
    topic: String,
    votes_per_voter: usize,
    user_id: String,
) -> Result<Poll, Error> {
    let key = make_key(poll_id.clone());
    let path = ".".to_string();
    let poll = Poll::new(poll_id, topic, votes_per_voter, user_id);
    let value = poll.string();
    redis::Script::new(
        r#"
        local key = KEYS[1]
        local path = ARGV[1]
        local value = ARGV[2]
        local ttl = ARGV[3]
        redis.call('JSON.SET', key, path, value)
        redis.call('EXPIRE', key, ttl)
        return 0
    "#,
    )
    .key(key)
    .arg(path)
    .arg(value)
    .arg(ttl.to_string())
    .invoke_async(con)
    .await
    .map_err(Error::RedisError)?;

    Ok(poll)
}

pub async fn get_poll(con: &mut ConnectionManager, poll_id: String) -> Result<Poll, Error> {
    let key = make_key(poll_id);
    let path = ".".to_string();
    let poll_json: String = redis::Script::new(
        r#"
        local key = KEYS[1]
        local path = ARGV[1]
        if redis.call('EXISTS', key) == 1 then  
            return redis.call('JSON.GET', key, path) 
        else
            return '-1'
        end
    "#,
    )
    .key(key)
    .arg(path)
    .invoke_async(con)
    .await
    .map_err(Error::RedisError)?;

    if poll_json == "-1" {
        return Err(Error::PollNotFound);
    }

    let poll: Poll = poll_json.try_into()?;
    Ok(poll)
}

pub async fn add_participant(
    con: &mut ConnectionManager,
    poll_id: String,
    user_id: String,
    name: String,
) -> Result<Poll, Error> {
    let key = make_key(poll_id);
    let path = make_participant_path(user_id);
    let value = serde_json::to_string(&name).unwrap();
    set_path_value(con, key, path, value).await
}

pub async fn remove_participant(
    con: &mut ConnectionManager,
    poll_id: String,
    user_id: String,
) -> Result<Poll, Error> {
    let key = make_key(poll_id);
    let path = make_participant_path(user_id);

    remove_path_value(con, key, path).await
}

pub async fn add_nomination(
    con: &mut ConnectionManager,
    poll_id: String,
    nomination_id: NominationID,
    nomination: Nomination,
) -> Result<Poll, Error> {
    let key = make_key(poll_id);
    let path = make_nomination_path(nomination_id);
    let value = serde_json::to_string(&nomination).unwrap();

    set_path_value(con, key, path, value).await
}

pub async fn remove_nomination(
    con: &mut ConnectionManager,
    poll_id: String,
    nomination_id: NominationID,
) -> Result<Poll, Error> {
    let key = make_key(poll_id);
    let path = make_nomination_path(nomination_id);

    remove_path_value(con, key, path).await
}

pub async fn start_poll(con: &mut ConnectionManager, poll_id: String) -> Result<Poll, Error> {
    let key = make_key(poll_id);
    let path = ".has_started".to_string();
    let started = true;
    let value = serde_json::to_string(&started).unwrap();

    set_path_value(con, key, path, value).await
}

pub async fn add_participant_rankings(
    con: &mut ConnectionManager,
    poll_id: String,
    user_id: String,
    rankings: Rankings,
) -> Result<Poll, Error> {
    let key = make_key(poll_id);
    let path = make_rankings_path(user_id);
    let value = serde_json::to_string(&rankings).unwrap();

    set_path_value(con, key, path, value).await
}

pub async fn add_results(
    con: &mut ConnectionManager,
    poll_id: String,
    results: Results,
) -> Result<Poll, Error> {
    let key = make_key(poll_id);
    let path = ".results".to_string();
    let value = serde_json::to_string(&results).unwrap();

    set_path_value(con, key, path, value).await
}

pub async fn del_poll(con: &mut ConnectionManager, poll_id: String) -> Result<(), Error> {
    let key = make_key(poll_id);
    cmd("JSON.DEL")
        .arg(key)
        .query_async(con)
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

async fn set_path_value(
    con: &mut ConnectionManager,
    key: String,
    path: String,
    value: String,
) -> Result<Poll, Error> {
    let poll_json: String = redis::Script::new(
        r#"
        local key = KEYS[1]
        local path = ARGV[1]
        local value = ARGV[2]
        if redis.call('EXISTS', key) == 1 then  
            redis.call('JSON.SET', key, path, value)
            return redis.call('JSON.GET', key, '.') 
        else
            return '-1'
        end
    "#,
    )
    .key(key)
    .arg(path)
    .arg(value)
    .invoke_async(con)
    .await
    .map_err(Error::RedisError)?;

    if poll_json == "-1" {
        return Err(Error::PollNotFound);
    }

    let poll: Poll = poll_json.try_into()?;
    Ok(poll)
}

async fn remove_path_value(
    con: &mut ConnectionManager,
    key: String,
    path: String,
) -> Result<Poll, Error> {
    let poll_json: String = redis::Script::new(
        r#"
        local key = KEYS[1]
        local path = ARGV[1]
        if redis.call('EXISTS', key) == 1 then  
            redis.call('JSON.DEL', key, path) 
            return redis.call('JSON.GET', key, '.') 
        else
            return '-1'
        end
    "#,
    )
    .key(key)
    .arg(path)
    .invoke_async(con)
    .await
    .map_err(Error::RedisError)?;

    if poll_json == "-1" {
        return Err(Error::PollNotFound);
    }

    let poll: Poll = poll_json.try_into()?;
    Ok(poll)
}
