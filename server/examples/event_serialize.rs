use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum Event {
    Message(String),
}
fn main() {
    let message = Event::Message("hello world".to_string());
    let value = serde_json::to_string(&message).unwrap();
    println!("{value}"); // print: {"Message":"hello world"}
}
