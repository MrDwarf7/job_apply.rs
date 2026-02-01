use sqlx::SqlitePool;

use crate::db::data::{JobCardData, WorkType};
use crate::error::DatabaseError;
use crate::prelude::{Error, Result};

pub type JobCreationRowId = i64;

pub async fn create_job(pool: &SqlitePool, data: &JobCardData) -> JobCreationRowId {
    let idx = data.idx as i64;
    let work_type = match data.work_type {
        WorkType::OnSite => "on-site",
        WorkType::Remote => "remote",
        WorkType::Hybrid => "hybrid",
    };

    let is_actively_reviewing = data.is_actively_reviewing as i64;
    let already_viewed = data.already_viewed as i64;
    let has_easy_apply = data.has_easy_apply as i64;

    sqlx::query!(
        r#"
        INSERT INTO jobs ( idx, card_title, job_title, company_name, state, country,
            work_type_fk, is_actively_reviewing, 
            already_viewed, full_date, 
            relative_date, has_easy_apply, 
            status_type_fk)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'pending')
        "#,
        idx,
        data.card_title,
        data.job_title,
        data.company_name,
        data.location.state,
        data.location.country,
        work_type,
        is_actively_reviewing,
        already_viewed,
        data.posted_date.full_date,
        data.posted_date.relative,
        has_easy_apply
    )
    .execute(pool)
    .await
    .map_err(|e| {
        Error::DatabaseError(DatabaseError::Create(format!(
            "Failed to insert job idx {}: {}",
            data.idx, e
        )))
    })
    .expect("Failed to insert job into database")
    .last_insert_rowid()
}

pub type AutofillCreationRowId = i64;

pub async fn create_autofill(pool: &SqlitePool, key: &str, data: &str) -> AutofillCreationRowId {
    sqlx::query!(
        r#"
        INSERT INTO autofill ( input_key, data )
        VALUES (?, ?)
        "#,
        key,
        data
    )
    .execute(pool)
    .await
    .map_err(|e| {
        Error::DatabaseError(DatabaseError::Create(format!(
            "Failed to insert autofill data for key {}: {}",
            key, e
        )))
    })
    .expect("Failed to insert autofill data into database")
    .last_insert_rowid()
}
