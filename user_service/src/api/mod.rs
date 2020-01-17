use actix_web::{web, Scope};

pub mod v1;

pub fn generate_web_scope() -> Scope {
    web::scope("/api").service(v1::generate_web_scope())
}
