use actix_web::{get, post, App, web, HttpResponse, HttpServer};
use actix_cors::Cors;
use dotenvy::dotenv;
use std::env;
use std::sync::Mutex;

pub use backend::{AppState, Event, MessagePayload};

#[get("/api/events")]
async fn handle_get_events(state: web::Data<AppState>) -> HttpResponse {
  let events = state.events.lock().unwrap();
  HttpResponse::Ok().json(events.clone())
}

#[post("/api/events")]
async fn handle_post_events(state: web::Data<AppState>, body: web::Json<Event>) -> HttpResponse {
  let mut events = state.events.lock().unwrap();
  events.push(body.into_inner());
  HttpResponse::Created().into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

  dotenv().expect("Failed to load .env file");
  let origin = env::var("ALLOWED_ORIGIN").expect("ALLOWED_ORIGIN not found");
  println!("Server running at http://localhost:8080");

  let state = web::Data::new(AppState {
    events: Mutex::new(Vec::new()),
  });

  HttpServer::new(move || {
    let cors = Cors::default()
      .allowed_origin(&origin)
      .allowed_methods(vec!["GET", "POST"])
      .allowed_headers(vec!["Content-Type"]);
    App::new()
      .wrap(cors)
      .app_data(state.clone())
      .service(handle_get_events)
      .service(handle_post_events)
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
