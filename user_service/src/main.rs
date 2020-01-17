use actix_web::{App, HttpServer};
use serde_derive::Serialize;

mod api;

#[derive(Serialize)]
struct User {
    pub name: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(api::generate_web_scope()))
        .bind("0.0.0.0:80")?
        .run()
        .await
}
