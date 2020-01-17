use actix_web::{App, HttpServer};

mod api;
mod models;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let addr = "0.0.0.0:80";

    let server = HttpServer::new(|| App::new().service(api::generate_web_scope())).bind(addr)?;

    println!("Listening on {}", &addr);

    server.run().await
}
