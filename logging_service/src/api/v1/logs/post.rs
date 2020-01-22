use actix_web::{web, Responder};
use chrono::prelude::*;
use serde_derive::Deserialize;

use domain::core::api::api_error::ApiError;
use domain::core::api::api_result::ApiResult;

use crate::models::log::{InsertableLog, Log, LogLevel};
use crate::service::v1::logs::create_log;
use crate::State;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub level: LogLevel,
    pub message: String,
    pub json_data: Option<String>,
}

pub async fn handler(params: web::Json<Params>, state: web::Data<State>) -> impl Responder {
    let json_data = match &params.json_data {
        Some(v) => Some(v.as_str()),
        None => None,
    };
    let result = create_log::handler(
        InsertableLog {
            message: params.message.as_str(),
            level: params.level.clone(),
            json_data,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        },
        &state.db,
    )
    .await;

    match result {
        Ok(v) => ApiResult::success(v),
        Err(_) => ApiResult::failure(ApiError {
            code: 0,
            message: "Failure".to_string(),
        }),
    }
}
