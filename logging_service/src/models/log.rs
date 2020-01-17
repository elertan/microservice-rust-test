use chrono::prelude::*;
use serde_derive::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub level: LogLevel,
    pub message: String,
    pub created_at: DateTime<Utc>,
}
