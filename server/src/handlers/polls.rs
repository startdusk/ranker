use std::sync::Arc;

use axum::{extract::State, Extension, Json};
use redis::aio::ConnectionManager;

use crate::{
    auth::{self, Authed},
    data::redis::polls,
    errors::Error,
    handlers::UnifyResponse,
    models::{AddPollReq, AddPollResp, JoinPollReq, JoinPollResp, Poll},
    shared::ids::{create_poll_id, create_user_id},
    state::AppState,
    validate::Input,
};

pub async fn add(
    State(state): State<Arc<AppState>>,
    Extension(mut con): Extension<ConnectionManager>,
    Input(input): Input<AddPollReq>,
) -> Result<Json<UnifyResponse<AddPollResp>>, Error> {
    let ttl = state.env.poll_duration;
    let poll_id = create_poll_id();
    let user_id = create_user_id();
    let poll = polls::add_poll(
        &mut con,
        ttl,
        poll_id.clone(),
        input.topic,
        input.votes_per_voter,
        user_id.clone(),
    )
    .await?;
    let access_token = auth::token_gen(poll_id, user_id, input.name, ttl)?;
    let add_poll_resp = AddPollResp { poll, access_token };
    Ok(UnifyResponse::ok(Some(add_poll_resp)).json())
}

pub async fn join(
    State(state): State<Arc<AppState>>,
    Extension(mut con): Extension<ConnectionManager>,
    Input(input): Input<JoinPollReq>,
) -> Result<Json<UnifyResponse<JoinPollResp>>, Error> {
    let ttl = state.env.poll_duration;
    let user_id = create_user_id();
    let poll_id = input.poll_id;
    let poll = polls::get_poll(&mut con, poll_id.clone()).await?;

    let access_token = auth::token_gen(poll_id, user_id, input.name, ttl)?;
    let join_poll_resp = JoinPollResp { poll, access_token };
    Ok(UnifyResponse::ok(Some(join_poll_resp)).json())
}

pub async fn rejoin(
    Extension(mut con): Extension<ConnectionManager>,
    authed: Authed,
) -> Result<Json<UnifyResponse<Poll>>, Error> {
    let poll = polls::add_participant(&mut con, authed.poll_id, authed.sub, authed.name).await?;
    Ok(UnifyResponse::ok(Some(poll)).json())
}
