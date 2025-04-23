use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use actix_web::{services, web, App, HttpServer};
use actix_cors::Cors;
use chrono::Local;
use crate::routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let server_address = dotenv::var("SERVER_ADDR")
    .unwrap_or_else(|_| "[::1]:8080".to_string());

  println!(
    "[{}] Listening on http://{}」!",
    Local::now().format(DATE_FORMAT),
    chrono::Local::now().format("%H:%M:%S")
    &server_address
  );

  HttpServer::new(move || {
    App::new()
      .wrap(Cors::permissive())
      .service(services)
      .server_hostname("poivre")
      .bind(server_address)?
      .run()
      .await
}
