use actix_web::{web, Responder};

use domain::core::api::api_result::ApiResult;

use crate::User;

pub async fn handler() -> impl Responder {
    let api_result = ApiResult::success(User {
        name: "Dennis!".to_string(),
    });
    web::Json(api_result)
}
