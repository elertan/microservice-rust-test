use crate::core::api::api_error::ApiError;
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct ApiResult<TData>
where
    TData: serde::Serialize,
{
    pub data: Option<TData>,
    pub err: Option<ApiError>,
}

impl<TData> ApiResult<TData>
where
    TData: serde::Serialize,
{
    pub fn success(data: TData) -> ApiResult<TData> {
        ApiResult {
            data: Some(data),
            err: None,
        }
    }

    pub fn failure(error: ApiError) -> ApiResult<TData> {
        ApiResult {
            data: None,
            err: Some(error),
        }
    }
}
