use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::Error;

pub type NominationID = String;

pub type Participants = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Nomination {
    pub text: String,
    pub user_id: String,
}

pub type Nominations = HashMap<NominationID, Nomination>;

pub type Rankings = HashMap<String, Vec<NominationID>>;

#[derive(Debug, Serialize, Clone, Deserialize, PartialEq, Eq)]
pub struct Result {
    pub nomination_id: NominationID,
    pub nomination_text: String,
    pub score: usize,
}

pub type Results = Vec<Result>;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
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

    pub fn string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl TryFrom<String> for Poll {
    type Error = Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        let poll: Poll = serde_json::from_str(&value).map_err(Error::DeserializeJsonError)?;
        Ok(poll)
    }
}

// =============================================================================
// DTO object

#[derive(Debug, Deserialize, Validate)]
pub struct AddPollReq {
    #[validate(length(min = 1, max = 100, message = "Can not be empty"))]
    pub topic: String,
    #[validate(range(min = 1, max = 5))]
    pub votes_per_voter: usize,
    #[validate(length(min = 1, max = 25, message = "Can not be empty"))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct JoinPollReq {
    #[validate(length(min = 6, max = 6, message = "Can not be empty"))]
    pub poll_id: String,

    #[validate(length(min = 1, max = 25, message = "Can not be empty"))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddNominationReq {
    #[validate(length(min = 1, max = 100, message = "Can not be empty"))]
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct AddPollResp {
    pub poll: Poll,
    pub access_token: String,
}

pub type JoinPollResp = AddPollResp;
