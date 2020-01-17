use actix_web::{web, Scope};

pub mod login;
pub mod logout;

pub fn generate_web_scope() -> Scope {
    web::scope("/auth")
        .route("/login", web::post().to(login::handler))
        .route("/logout", web::post().to(logout::handler))
}
