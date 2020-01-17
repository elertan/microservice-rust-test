use actix_web::{web, Responder};
use serde_derive::Deserialize;

use domain::core::api::api_result::ApiResult;

use crate::User;

#[derive(Deserialize)]
pub struct Params {
    pub email: String,
    pub password: String,
}

pub async fn handler(params: web::Json<Params>) -> impl Responder {
    let api_result = ApiResult::success(User {
        name: params.email.clone(),
    });
    web::Json(api_result)
}
