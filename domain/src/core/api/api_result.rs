use crate::core::api::api_error::ApiError;
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct ApiResult<'a, TData>
where
    TData: serde::Serialize,
{
    pub data: Option<TData>,
    pub err: Option<ApiError<'a>>,
}

impl<'a, TData> ApiResult<'a, TData>
where
    TData: serde::Serialize,
{
    pub fn success(data: TData) -> ApiResult<'a, TData> {
        ApiResult {
            data: Some(data),
            err: None,
        }
    }

    pub fn failure(error: ApiError<'a>) -> ApiResult<'a, TData> {
        ApiResult {
            data: None,
            err: Some(error),
        }
    }
}
