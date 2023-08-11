use serde::{Deserialize, Serialize};

use super::{AddNominationReq, NominationID, Poll, RankingList};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebSocketEvent {
    Exception(String),
    PollUpdated(Box<Poll>),
    RemoveParticipant(String),
    Nomination(AddNominationReq),
    RemoveNomination(NominationID),
    StartVote,
    SubmitRankings(RankingList),
    CancelPoll,
    ClosePoll,
    PollCancelled,
}

impl WebSocketEvent {
    pub fn message(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl From<String> for WebSocketEvent {
    fn from(value: String) -> Self {
        let res = serde_json::from_str::<WebSocketEvent>(&value);
        if let Err(err) = res {
            return WebSocketEvent::Exception(err.to_string());
        };
        res.unwrap()
    }
}
