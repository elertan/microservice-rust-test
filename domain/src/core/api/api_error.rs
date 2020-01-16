use serde_derive::Serialize;

#[derive(Serialize)]
pub struct ApiError<'a> {
    pub code: u32,
    pub message: &'a str,
}
