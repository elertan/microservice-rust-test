use actix_web::{web, Scope};

pub mod auth;

pub fn generate_web_scope() -> Scope {
    web::scope("/v1").service(auth::generate_web_scope())
}
