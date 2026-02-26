use actix_web::{App, web, HttpServer};
use actix_cors::Cors;
use dotenvy::dotenv;
use std::env;
use std::sync::Mutex;

pub use backend::{AppState, Event, MessagePayload, handle_get_events, handle_post_events};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // load environment variables
  dotenv().expect("Failed to load .env file");
  let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS not found");
  let origin = env::var("ALLOWED_ORIGIN").expect("ALLOWED_ORIGIN not found");
  let state = web::Data::new(AppState {
    events: Mutex::new(Vec::new()),
  });

  println!("Server running at {}", bind_address);
  HttpServer::new(move || {
    let origin = origin.clone();
    let cors = if origin == "*" {
      Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec!["Content-Type"])
    } else {
      Cors::default()
        .allowed_origin(&origin)
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec!["Content-Type"])
    };

    App::new()
      .wrap(cors)
      .app_data(state.clone())
      .service(handle_get_events)
      .service(handle_post_events)
  })
  .bind(bind_address)?
  .run()
  .await
}
