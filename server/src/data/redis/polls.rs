use redis::{aio::ConnectionLike, cmd};

use crate::{
    models::{Nomination, NominationID, Poll, RankingList, Results},
    Error,
};

pub const POLL_KEY_PREFIX: &str = "polls:";

pub async fn add_poll<C>(
    con: &mut C,
    ttl: usize,
    poll_id: String,
    topic: String,
    votes_per_voter: usize,
    user_id: String,
) -> Result<Poll, Error>
where
    C: ConnectionLike,
{
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

pub async fn get_poll<C>(con: &mut C, poll_id: String) -> Result<Poll, Error>
where
    C: ConnectionLike,
{
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

pub async fn add_participant<C>(
    con: &mut C,
    poll_id: String,
    user_id: String,
    name: String,
) -> Result<Poll, Error>
where
    C: ConnectionLike,
{
    let key = make_key(poll_id);
    let path = make_participant_path(user_id);
    let value = serde_json::to_string(&name).unwrap();
    set_path_value(con, key, path, value).await
}

pub async fn remove_participant<C>(
    con: &mut C,
    poll_id: String,
    user_id: String,
) -> Result<Poll, Error>
where
    C: ConnectionLike,
{
    let key = make_key(poll_id);
    let path = make_participant_path(user_id);
    // can remove if no has_started == 'false'
    let poll_json: String = redis::Script::new(
        r#"
        local key = KEYS[1]
        local path = ARGV[1]
        local value = ARGV[2]
        if redis.call('EXISTS', key) == 1 then  
            if redis.call('JSON.GET', key, '.has_started') == 'false' then
                redis.call('JSON.DEL', key, path)
                return redis.call('JSON.GET', key, '.') 
            else
                return '-2'
            end
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

    if poll_json == "-2" {
        return Err(Error::PollHasStarted);
    }

    let poll: Poll = poll_json.try_into()?;
    Ok(poll)
}

pub async fn add_nomination<C>(
    con: &mut C,
    poll_id: String,
    nomination_id: NominationID,
    nomination: Nomination,
) -> Result<Poll, Error>
where
    C: ConnectionLike,
{
    let key = make_key(poll_id);
    let path = make_nomination_path(nomination_id);
    let value = serde_json::to_string(&nomination).unwrap();

    set_path_value(con, key, path, value).await
}

pub async fn remove_nomination<C>(
    con: &mut C,
    poll_id: String,
    nomination_id: NominationID,
) -> Result<Poll, Error>
where
    C: ConnectionLike,
{
    let key = make_key(poll_id);
    let path = make_nomination_path(nomination_id);

    remove_path_value(con, key, path).await
}

pub async fn start_poll<C>(con: &mut C, poll_id: String) -> Result<Poll, Error>
where
    C: ConnectionLike,
{
    let key = make_key(poll_id);
    let path = ".has_started".to_string();
    let started = true;
    let value = serde_json::to_string(&started).unwrap();
    set_path_value(con, key, path, value).await
}

pub async fn add_participant_rankings<C>(
    con: &mut C,
    poll_id: String,
    user_id: String,
    rankings: RankingList,
) -> Result<Poll, Error>
where
    C: ConnectionLike,
{
    let key = make_key(poll_id);
    let path = make_rankings_path(user_id);
    let value = serde_json::to_string(&rankings).unwrap();

    let poll_json: String = redis::Script::new(
        r#"
        local key = KEYS[1]
        local path = ARGV[1]
        local value = ARGV[2]
        if redis.call('EXISTS', key) == 1 then  
            if redis.call('JSON.GET', key, '.has_started') == 'true' then
                redis.call('JSON.SET', key, path, value)
                return redis.call('JSON.GET', key, '.') 
            else
                return '-2'
            end
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

    if poll_json == "-2" {
        return Err(Error::PollNoStart);
    }

    let poll: Poll = poll_json.try_into()?;
    Ok(poll)
}

pub async fn add_results<C>(con: &mut C, poll_id: String, results: Results) -> Result<Poll, Error>
where
    C: ConnectionLike,
{
    let key = make_key(poll_id);
    let path = ".results".to_string();
    let value = serde_json::to_string(&results).unwrap();

    set_path_value(con, key, path, value).await
}

pub async fn del_poll<C>(con: &mut C, poll_id: String) -> Result<(), Error>
where
    C: ConnectionLike,
{
    let key = make_key(poll_id);
    cmd("JSON.DEL")
        .arg(key)
        .query_async(con)
        .await
        .map_err(Error::RedisError)?;

    Ok(())
}

async fn set_path_value<C>(
    con: &mut C,
    key: String,
    path: String,
    value: String,
) -> Result<Poll, Error>
where
    C: ConnectionLike,
{
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

async fn remove_path_value<C>(con: &mut C, key: String, path: String) -> Result<Poll, Error>
where
    C: ConnectionLike,
{
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

fn make_key(poll_id: String) -> String {
    format!("{}{}", POLL_KEY_PREFIX, poll_id)
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

#[cfg(test)]
mod tests {
    use super::*;
    use futures::FutureExt;
    use mockall::mock;
    use redis::{Cmd, ErrorKind, Pipeline, RedisError, RedisFuture, Value};

    mock! {
        pub ConnectionLike {
            fn mock_returning(&mut self) -> Result<String, RedisError>;
        }
    }

    // The redis::aio::ConnectionLike trait has methods with lifetimes and generic
    // arguments, which mockall cannot mock directly. However, you can create your
    // own trait without lifetimes and generics and implement it for redis::aio::ConnectionLike.
    impl redis::aio::ConnectionLike for MockConnectionLike {
        fn req_packed_command<'a>(&'a mut self, _cmd: &'a Cmd) -> RedisFuture<'a, Value> {
            match self.mock_returning() {
                Ok(data) => {
                    let data = data.as_bytes().to_vec();
                    (async move { Ok(redis::Value::Data(data)) }).boxed()
                }
                Err(err) => (async move { Err(err) }).boxed(),
            }
        }

        fn req_packed_commands<'a>(
            &'a mut self,
            _cmd: &'a Pipeline,
            _offset: usize,
            _count: usize,
        ) -> RedisFuture<'a, Vec<Value>> {
            todo!()
        }

        fn get_db(&self) -> i64 {
            0
        }
    }

    #[tokio::test]
    async fn test_set_path_value_should_error() {
        let mut con = MockConnectionLike::new();

        con.mock_returning.expect().returning(|| {
            Err(RedisError::from((
                ErrorKind::TypeError,
                "custom_redis_error",
                "".to_string(),
            )))
        });
        let Err(err) = set_path_value(
            &mut con,
            "key".to_string(),
            "path".to_string(),
            "value".to_string(),
        )
        .await else {
            panic!("error")
        };
        assert_eq!(
            err,
            Error::RedisError(RedisError::from((
                ErrorKind::TypeError,
                "custom_redis_error",
                "".to_string()
            )))
        )
    }

    #[tokio::test]
    async fn test_remove_path_value_should_error() {
        let mut con = MockConnectionLike::new();

        con.mock_returning.expect().returning(|| {
            Err(RedisError::from((
                ErrorKind::TypeError,
                "custom_redis_error",
                "".to_string(),
            )))
        });
        let Err(err) = remove_path_value(
            &mut con,
            "key".to_string(),
            "path".to_string(),
        )
        .await else {
            panic!("error")
        };
        assert_eq!(
            err,
            Error::RedisError(RedisError::from((
                ErrorKind::TypeError,
                "custom_redis_error",
                "".to_string()
            )))
        )
    }

    #[tokio::test]
    async fn test_set_path_value_should_not_found() {
        let mut con = MockConnectionLike::new();
        con.mock_returning
            .expect()
            .returning(|| Ok("-1".to_string()));
        let Err(err) = set_path_value(
            &mut con,
            "key".to_string(),
            "path".to_string(),
            "value".to_string(),
        )
        .await else {
            panic!("error")
        };
        assert_eq!(err, Error::PollNotFound)
    }

    #[tokio::test]
    async fn test_remove_path_value_should_not_found() {
        let mut con = MockConnectionLike::new();
        con.mock_returning
            .expect()
            .returning(|| Ok("-1".to_string()));
        let Err(err) = remove_path_value(
            &mut con,
            "key".to_string(),
            "path".to_string(),
        )
        .await else {
            panic!("error")
        };
        assert_eq!(err, Error::PollNotFound)
    }
}
