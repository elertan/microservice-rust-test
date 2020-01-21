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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let addr = "0.0.0.0:80";

    let server = HttpServer::new(move || {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_conn = PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        println!("Connected to database: {}", &database_url);

        let state = State { db: db_conn };
        App::new().data(state).service(api::generate_web_scope())
    })
    .bind(addr)?;

    println!("Listening on {}!", &addr);

    server.run().await
}
