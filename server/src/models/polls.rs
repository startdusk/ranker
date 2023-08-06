use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use validator::Validate;

pub type NominationID = String;

pub type Participants = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Nomination {
    pub text: String,
    pub user_id: String,
}

pub type Nominations = HashMap<NominationID, String>;

pub type Rankings = HashMap<String, Vec<NominationID>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    pub nomination_id: NominationID,
    pub nomination_text: String,
    pub score: usize,
}

pub type Results = Vec<Result>;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Poll {
    pub id: String,
    pub topic: String,
    pub votes_per_voter: usize,
    pub participants: Participants,
    pub admin_id: String,
    pub nominations: Nominations,
    pub rankings: Rankings,
    pub results: Results,
    pub has_started: bool,
}

impl Poll {
    pub fn new(poll_id: String, topic: String, votes_per_voter: usize, user_id: String) -> Self {
        Self {
            id: poll_id,
            topic,
            votes_per_voter,
            admin_id: user_id,
            ..Default::default()
        }
    }
}

// =============================================================================
// DTO object

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
pub struct AddNomination {
    #[validate(length(min = 1, max = 100, message = "Can not be empty"))]
    pub text: String,
}
