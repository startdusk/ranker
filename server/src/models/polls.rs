use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
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
