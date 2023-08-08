use axum::{Extension, Json};
use redis::aio::ConnectionManager;

use crate::{
    data::redis::polls,
    middlewares::jwt::AuthPayload,
    models::{AddPoll, JoinPoll, Poll},
    Error, UnifyResponse, ValidatedInput,
};

pub async fn add(
    ValidatedInput(input): ValidatedInput<AddPoll>,
) -> Result<Json<UnifyResponse<Poll>>, Error> {
    let poll = Poll {
        topic: input.topic,
        votes_per_voter: input.votes_per_voter,
        ..Default::default()
    };
    Ok(UnifyResponse::ok(Some(poll)).json())
}

pub async fn join(
    ValidatedInput(_input): ValidatedInput<JoinPoll>,
) -> Result<Json<UnifyResponse<Poll>>, Error> {
    let poll = Poll {
        ..Default::default()
    };
    Ok(UnifyResponse::ok(Some(poll)).json())
}

pub async fn rejoin(
    Extension(mut con): Extension<ConnectionManager>,
    Extension(auth_payload): Extension<AuthPayload>,
) -> Result<Json<UnifyResponse<Poll>>, Error> {
    let poll = polls::add_participant(
        &mut con,
        auth_payload.poll_id,
        auth_payload.user_id,
        auth_payload.name,
    )
    .await?;
    Ok(UnifyResponse::ok(Some(poll)).json())
}
