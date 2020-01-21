table! {
    use diesel::sql_types::*;
    use crate::models::log::Log_level;

    log (id) {
        id -> Int4,
        level -> Log_level,
        message -> Varchar,
        json_data -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
