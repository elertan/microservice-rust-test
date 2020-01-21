use crate::core::api::api_error::ApiError;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use chrono::prelude::*;
use futures::future::{ready, Ready};
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct ApiResult<TData>
where
    TData: serde::Serialize,
{
    pub data: Option<TData>,
    pub err: Option<ApiError>,
    pub time: DateTime<Utc>,
}

impl<TData> ApiResult<TData>
where
    TData: serde::Serialize,
{
    pub fn success(data: TData) -> ApiResult<TData> {
        ApiResult {
            data: Some(data),
            err: None,
            time: Utc::now(),
        }
    }

    pub fn failure(error: ApiError) -> ApiResult<TData> {
        ApiResult {
            data: None,
            err: Some(error),
            time: Utc::now(),
        }
    }
}

// Make it easy to respond with an api result
impl<T> Responder for ApiResult<T>
where
    T: serde::Serialize,
{
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}
