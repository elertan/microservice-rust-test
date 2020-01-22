use actix_web::{web, Responder};
use serde_derive::Deserialize;

use domain::core::api::api_error::ApiError;
use domain::core::api::api_result::ApiResult;

use crate::models::log::LogLevel;
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
        create_log::Data {
            message: &params.message,
            level: &params.level,
            json_data,
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
