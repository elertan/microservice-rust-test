#[macro_use]
extern crate diesel;

use std::env;

use actix_web::{App, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::*;

mod api;
mod models;
mod schema;
mod service;

pub struct State {
    pub db: PgConnection,
}

async fn state_factory() -> std::io::Result<State> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    println!("Connected to database: {}", &database_url);

    Ok(State { db })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let addr = "0.0.0.0:80";

    let server = HttpServer::new(move || {
        App::new()
            .data_factory(state_factory)
            .service(api::generate_web_scope())
    })
    .bind(addr)?;

    println!("Listening on {}!", &addr);

    server.run().await
}
