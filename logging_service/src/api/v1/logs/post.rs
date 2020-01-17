use actix_web::{web, Responder};
use chrono::prelude::*;
use serde_derive::Deserialize;

use domain::core::api::api_result::ApiResult;

use crate::models::log::{Log, LogLevel};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub level: LogLevel,
    pub message: String,
}

pub async fn handler(params: web::Json<Params>) -> impl Responder {
    let api_result = ApiResult::success(Log {
        level: params.level,
        message: params.message.clone(),
        created_at: Utc::now(),
    });
    web::Json(api_result)
}
