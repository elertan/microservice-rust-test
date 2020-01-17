use serde_derive::Serialize;

#[derive(Serialize)]
pub struct ApiError {
    pub code: u32,
    pub message: String,
}
