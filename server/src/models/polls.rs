use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::errors::Error;

pub type NominationID = String;

pub type Participants = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Nomination {
    pub text: String,
    pub user_id: String,
}

pub type Nominations = HashMap<NominationID, Nomination>;

pub type UserID = String;
pub type Rankings = HashMap<UserID, RankingList>;
pub type RankingList = Vec<NominationID>;

#[derive(Debug, Serialize, Clone, Deserialize, PartialEq, Eq)]
pub struct Result {
    pub nomination_id: NominationID,
    pub nomination_text: String,
    pub score: String,
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

    pub fn get_results(&self) -> Results {
        // 1. Each value of `rankings` key values is an array of a participants'
        // vote. Points for each array element corresponds to following formula:
        // r_n = ((votesPerVoter - 0.5*n) / votesPerVoter)^(n+1), where n corresponds
        // to array index of rankings.
        // Accumulate score per NominationID
        let mut scores: HashMap<NominationID, f64> = HashMap::new();

        let votes_per_voter = self.votes_per_voter as f64;
        for pre_rankings in self.rankings.values() {
            for (n, nomination_id) in pre_rankings.iter().enumerate() {
                let vote_value =
                    ((votes_per_voter - 0.5 * n as f64) / votes_per_voter).powf(n as f64 + 1.0);

                *scores.entry(nomination_id.clone()).or_insert(0.0) += vote_value;
            }
        }

        // 2. Take NominationID to score mapping, and merge in nomination_text
        // and NominationID into value
        let mut results: Results = scores
            .into_iter()
            .map(|(nomination_id, score)| Result {
                nomination_id: nomination_id.clone(),
                nomination_text: self.nominations[&nomination_id].text.clone(),
                score: score.to_string(),
            })
            .collect();

        // 3. Sort values by score in descending order
        results.sort_by(|res1, res2| {
            let score1 = res1.score.parse::<f64>().unwrap();
            let score2 = res2.score.parse::<f64>().unwrap();
            score1.partial_cmp(&score2).unwrap()
        });

        results
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

#[derive(Debug, Serialize, Deserialize, Validate)]
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
