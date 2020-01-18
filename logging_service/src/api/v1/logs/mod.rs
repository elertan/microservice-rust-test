use actix_web::{web, Scope};

pub mod get;
pub mod post;

pub fn generate_web_scope() -> Scope {
    web::scope("/logs")
        .route("", web::post().to(post::handler))
        .route("", web::get().to(get::handler))
}
