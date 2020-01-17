use actix_web::{web, Responder, Scope};
use serde_derive::Deserialize;

use domain::core::api::api_result::ApiResult;

use crate::User;

pub fn generate_web_scope() -> Scope {
    web::scope("/auth")
        .route("/login", web::post().to(login))
        .route("/logout", web::post().to(logout))
}

#[derive(Deserialize)]
struct LoginParams {
    pub email: String,
    pub password: String,
}

async fn login(params: web::Json<LoginParams>) -> impl Responder {
    let api_result = ApiResult::success(User {
        name: params.email.clone(),
    });
    web::Json(api_result)
}

async fn logout() -> impl Responder {
    let api_result = ApiResult::success(User {
        name: "Dennis!".to_string(),
    });
    web::Json(api_result)
}
