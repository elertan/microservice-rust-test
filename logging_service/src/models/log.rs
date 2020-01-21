use chrono::prelude::*;
use diesel_derive_enum::DbEnum;
use serde_derive::*;

use crate::schema::log;

#[derive(Debug, Serialize, Deserialize, DbEnum, PartialEq)]
#[serde(rename_all = "camelCase")]
#[DieselType = "Log_level"]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Identifiable, PartialEq)]
#[serde(rename_all = "camelCase")]
#[table_name = "log"]
pub struct Log {
    pub id: i32,
    pub level: LogLevel,
    pub message: String,
    pub json_data: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
