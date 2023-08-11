use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Event {
    Message(String),
    PollUpdated(String),
    StartVote,
    SubmitRankings(Vec<String>),
}
fn main() {
    let message = Event::Message("hello world".to_string());
    let value = serde_json::to_string(&message).unwrap();
    println!("{value}"); // print: {"message":"hello world"}

    let message = Event::PollUpdated("poll update".to_string());
    let value = serde_json::to_string(&message).unwrap();
    println!("{value}"); // print: {"poll_update":"poll update"}

    let message = Event::StartVote;
    let value = serde_json::to_string(&message).unwrap();
    println!("{value}"); // "start_vote"

    let message = Event::SubmitRankings(vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    let value = serde_json::to_string(&message).unwrap();
    println!("{value}"); // "start_vote"
}
