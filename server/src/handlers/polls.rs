use axum::Json;

use crate::{models::AddPoll, UnifyResponse, ValidatedInput};

pub async fn add(ValidatedInput(input): ValidatedInput<AddPoll>) -> Json<UnifyResponse<AddPoll>> {
    Json(UnifyResponse::ok(0, Some(input)))
}
pub async fn join() {}
pub async fn rejoin() {}
