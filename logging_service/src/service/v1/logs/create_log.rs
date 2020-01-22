use chrono::prelude::*;
use diesel::{PgConnection, RunQueryDsl};

use crate::models::log::{InsertableLog, Log, LogLevel};
use crate::schema::log;

pub struct Data<'a> {
    pub level: &'a LogLevel,
    pub message: &'a str,
    pub json_data: Option<&'a str>,
}

impl<'a> Into<InsertableLog<'a>> for Data<'a> {
    fn into(self) -> InsertableLog<'a> {
        InsertableLog {
            level: self.level,
            message: self.message,
            json_data: self.json_data,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}

pub async fn handler(data: Data<'_>, db: &PgConnection) -> Result<Log, failure::Error> {
    let values: InsertableLog = data.into();

    let log = diesel::insert_into(log::table)
        .values(&values)
        .get_result(db)
        .expect("Log insert failed");

    Ok(log)
}
