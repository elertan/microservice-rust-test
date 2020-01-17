use actix_web::{web, App, HttpServer, Responder};
use serde_derive::Serialize;

use domain::core::api::api_result::ApiResult;

mod api;

#[derive(Serialize)]
struct User {
    pub name: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(api::generate_web_scope()))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
