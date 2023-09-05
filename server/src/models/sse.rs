use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Notification {
    pub notify_type: NotifyType,
    pub username: String,
    pub poll_id: String,
    pub topic: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum NotifyType {
    JoinPoll,
}
