use axum::Json;

use crate::{
    ids::{create_poll_id, create_user_id},
    models::{AddPoll, JoinPoll, JoinPollResult, Poll, RejoinPoll},
    UnifyResponse, ValidatedInput,
};

pub async fn add(ValidatedInput(input): ValidatedInput<AddPoll>) -> Json<UnifyResponse<Poll>> {
    let poll = Poll {
        topic: input.topic,
        votes_per_voter: input.votes_per_voter,
        name: input.name,
        poll_id: create_poll_id(),
        user_id: create_user_id(),
    };
    Json(UnifyResponse::ok(Some(poll)))
}

pub async fn join(
    ValidatedInput(input): ValidatedInput<JoinPoll>,
) -> Json<UnifyResponse<JoinPollResult>> {
    let poll = JoinPollResult {
        poll_id: input.poll_id,
        name: input.name,
        user_id: create_user_id(),
    };
    Json(UnifyResponse::ok(Some(poll)))
}

pub async fn rejoin(
    ValidatedInput(input): ValidatedInput<RejoinPoll>,
) -> Json<UnifyResponse<JoinPollResult>> {
    let poll = JoinPollResult {
        name: input.name,
        poll_id: create_poll_id(),
        user_id: create_user_id(),
    };
    Json(UnifyResponse::ok(Some(poll)))
}
