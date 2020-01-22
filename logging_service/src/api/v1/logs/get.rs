use actix_web::{web, Responder};
use diesel::prelude::*;

use domain::core::api::api_result::ApiResult;

use crate::models::log::Log;
use crate::State;

pub async fn handler(data: web::Data<State>) -> impl Responder {
    use crate::schema::log::dsl::*;

    let db = &data.db;
    let logs = log
        .limit(50)
        .order(created_at.desc())
        .load::<Log>(db)
        .expect("fail");

    ApiResult::success(logs)
}
