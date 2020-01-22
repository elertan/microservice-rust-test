use diesel::{PgConnection, RunQueryDsl};

use crate::models::log::{InsertableLog, Log};
use crate::schema::log;

pub async fn handler(data: InsertableLog<'_>, db: &PgConnection) -> Result<Log, failure::Error> {
    let log = diesel::insert_into(log::table)
        .values(&data)
        .get_result(db)
        .expect("Log insert failed");

    Ok(log)
}
