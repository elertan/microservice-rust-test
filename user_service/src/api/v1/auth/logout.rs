use actix_web::{web, Responder};

use domain::core::api::api_result::ApiResult;

use crate::User;

pub async fn handler() -> impl Responder {
    ApiResult::success(User {
        name: "Dennis!".to_string(),
    })
}
