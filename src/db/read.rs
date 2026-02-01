use sqlx::SqlitePool;

use crate::error::DatabaseError;
use crate::prelude::{Error, Result};

pub async fn get_autofill(pool: &SqlitePool, key: &str) -> Option<String> {
    sqlx::query!("SELECT data FROM autofill where input_key = ?", key)
        .fetch_optional(pool)
        .await
        .ok()
        .and_then(|row| row.map(|r| r.data))
}
