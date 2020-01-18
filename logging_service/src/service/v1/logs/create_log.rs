use chrono::prelude::*;

use crate::models::log::{Log, LogLevel};

pub struct Data {
    pub level: LogLevel,
    pub message: String,
}

pub async fn handler(data: Data) -> Result<Log, failure::Error> {
    // Save to database
    Ok(Log {
        level: data.level,
        message: data.message,
        created_at: Utc::now(),
    })
}
