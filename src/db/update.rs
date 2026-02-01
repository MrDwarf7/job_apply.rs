use sqlx::SqlitePool;

use crate::db::data::StatusType;
use crate::error::DatabaseError;
use crate::prelude::{Error, Result};

pub async fn update_job_status(
    pool: &SqlitePool,
    job_id: i64,
    new_status: StatusType,
) -> Result<()> {
    let status: &str = new_status.into();
    sqlx::query!(
        r#"
        UPDATE jobs 
        SET status_type_fk = ? 
        WHERE id = ?
        "#,
        status,
        job_id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        Error::DatabaseError(DatabaseError::Update(format!(
            "Failed to update job status for job_id {}: {}",
            job_id, e
        )))
    })
    .map(|_| ())
}

pub async fn update_autofill(pool: &SqlitePool, key: &str, data: &str) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT OR REPLACE INTO autofill ( input_key, data )
        VALUES (?, ?)
        "#,
        key,
        data
    )
    .execute(pool)
    .await
    .map_err(|e| {
        Error::DatabaseError(DatabaseError::Update(format!(
            "Failed to update autofill for key {}: {}",
            key, e
        )))
    })
    .map(|_| ())
}
