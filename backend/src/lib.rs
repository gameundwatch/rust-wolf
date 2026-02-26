use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use ts_rs::TS;

#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[ts(export, export_to = "../../packages/bindings/")]
pub struct MessagePayload {
    pub to: String,
    pub content: String,
}

#[derive(TS, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[ts(export, export_to = "../../packages/bindings/")]
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

#[get("/api/events")]
pub async fn handle_get_events(state: web::Data<AppState>) -> HttpResponse {
  let events = state.events.lock().unwrap();
  HttpResponse::Ok().json(events.clone())
}

#[post("/api/events")]
pub async fn handle_post_events(state: web::Data<AppState>, body: web::Json<Event>) -> HttpResponse {
  let mut events = state.events.lock().unwrap();
  events.push(body.into_inner());
  HttpResponse::Created().into()
}
