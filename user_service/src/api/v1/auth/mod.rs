use actix_web::{web, Responder, Scope};

use domain::core::api::api_result::ApiResult;

use crate::User;

pub fn generate_web_scope() -> Scope {
    web::scope("/auth")
        .route("/login", web::post().to(login))
        .route("/logout", web::post().to(logout))
}

async fn login() -> impl Responder {
    let api_result = ApiResult::success(User {
        name: "Dennis!".to_string(),
    });
    web::Json(api_result)
}

async fn logout() -> impl Responder {
    let api_result = ApiResult::success(User {
        name: "Dennis!".to_string(),
    });
    web::Json(api_result)
}
