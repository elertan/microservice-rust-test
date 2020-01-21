use actix_web::{web, Responder};
use serde_derive::Deserialize;

use domain::core::api::api_error::ApiError;
use domain::core::api::api_result::ApiResult;

use crate::models::log::{Log, LogLevel};
//use crate::service::v1::logs::create_log;
use crate::State;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub level: LogLevel,
    pub message: String,
}

pub async fn handler(params: web::Json<Params>, data: web::Data<State>) -> impl Responder {
    ApiResult::<Log>::failure(ApiError {
        code: 0,
        message: "oops".to_string(),
    })
    //    let result = create_log::handler(create_log::Data {
    //        level: params.level,
    //        message: params.message.clone(),
    //    })
    //    .await;

    //    match result {
    //        Ok(log) => {
    ////            let mut state = data.lock().unwrap();
    ////            state.logs.push(log.clone());
    //            ApiResult::success(log)
    //        }
    //        Err(_) => ApiResult::failure(ApiError {
    //            code: 0,
    //            message: "test".to_string(),
    //        }),
    //    }
}
