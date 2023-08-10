use serde::{Deserialize, Serialize};

use super::{AddNominationReq, NominationID, Poll, Rankings};

#[derive(Debug, Serialize, Deserialize)]
pub enum WebSocketEvent {
    Exception(String),
    PollUpdated(Poll),
    RemoveParticipant(String),
    Nomination(AddNominationReq),
    RemoveNomination(NominationID),
    StartVote,
    SubmitRankings(Rankings),
    CancelPoll,
    ClosePoll,
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
