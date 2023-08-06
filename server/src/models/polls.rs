use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Debug, Serialize)]
pub struct Poll {
    pub topic: String,
    pub votes_per_voter: usize,
    pub name: String,
    pub poll_id: String,
    pub user_id: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddPoll {
    #[validate(length(min = 1, max = 100, message = "Can not be empty"))]
    pub topic: String,
    #[validate(range(min = 1, max = 5))]
    pub votes_per_voter: usize,
    #[validate(length(min = 1, max = 25, message = "Can not be empty"))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct JoinPoll {
    #[validate(length(min = 6, max = 6, message = "Can not be empty"))]
    pub poll_id: String,

    #[validate(length(min = 1, max = 25, message = "Can not be empty"))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RejoinPoll {
    #[validate(length(min = 1, max = 25, message = "Can not be empty"))]
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct JoinPollResult {
    pub poll_id: String,
    pub user_id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct Nomination {
    #[validate(length(min = 1, max = 100, message = "Can not be empty"))]
    pub text: String,
}
