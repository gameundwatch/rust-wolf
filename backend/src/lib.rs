use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessagePayload {
    pub to: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    #[serde(rename = "type")]
    pub event_type: String,
    pub time: String,
    pub from: String,
    pub message: Option<MessagePayload>,
}

pub struct AppState {
    pub events: Mutex<Vec<Event>>,
}
