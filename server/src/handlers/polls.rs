use axum::Json;

use crate::{
    ids::{create_poll_id, create_user_id},
    models::{AddPoll, JoinPoll, JoinPollResult, Poll, RejoinPoll},
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
    ValidatedInput(input): ValidatedInput<JoinPoll>,
) -> Result<Json<UnifyResponse<JoinPollResult>>, Error> {
    let poll = JoinPollResult {
        poll_id: input.poll_id,
        name: input.name,
        user_id: create_user_id(),
    };
    Ok(UnifyResponse::ok(Some(poll)).json())
}

pub async fn rejoin(
    ValidatedInput(input): ValidatedInput<RejoinPoll>,
) -> Result<Json<UnifyResponse<JoinPollResult>>, Error> {
    let poll = JoinPollResult {
        name: input.name,
        poll_id: create_poll_id(),
        user_id: create_user_id(),
    };

    Ok(UnifyResponse::ok(Some(poll)).json())
}
