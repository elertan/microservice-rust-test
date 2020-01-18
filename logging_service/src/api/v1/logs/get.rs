use std::sync::Mutex;

use actix_web::{web, Responder};

use domain::core::api::api_result::ApiResult;

use crate::State;

pub async fn handler(data: web::Data<Mutex<State>>) -> impl Responder {
    let state = data.lock().unwrap();
    ApiResult::success(state.logs.clone())
}
