use actix_web::{web, Scope};

pub mod logs;

pub fn generate_web_scope() -> Scope {
    web::scope("/v1").service(logs::generate_web_scope())
}
