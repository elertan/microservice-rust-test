use std::sync::Mutex;

use actix_web::{App, HttpServer};

use crate::models::log::Log;

mod api;
mod models;
mod service;

#[derive(Debug, Clone)]
pub struct State {
    pub logs: Vec<Log>,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let addr = "0.0.0.0:80";

    let server = HttpServer::new(move || {
        let state = Mutex::new(State { logs: vec![] });
        App::new().data(state).service(api::generate_web_scope())
    })
    .bind(addr)?;

    println!("Listening on {}!", &addr);

    server.run().await
}
