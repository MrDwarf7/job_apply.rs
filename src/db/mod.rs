mod create;
mod data;
mod delete;
mod read;
mod update;

use std::sync::Arc;

use fantoccini::Client;
use fantoccini::elements::Element;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{SqlitePool, migrate};

use crate::config::DatabaseConfig;
pub use crate::db::create::{create_autofill, create_job};
use crate::db::data::JobCardData;
pub use crate::db::read::get_autofill;
pub use crate::db::update::{update_autofill, update_job_status};
use crate::prelude::Result;
use crate::providers::{Provider, SelectorKind};

// pub use crate::db::read::{get_job_by_id, get_jobs_by_status, get_all_jobs};
// pub use crate::db::delete::delete_job_by_id;

pub(crate) async fn setup_db(db_config: DatabaseConfig) -> SqlitePool {
    let db_uri = db_config.database_uri.clone(); // clone before move occurs

    let sqlite_options: SqlitePoolOptions = db_config.into();

    let pool = sqlite_options
        .connect(db_uri.as_str())
        .await
        .expect("Failed to create SQLite connection pool");

    migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    pool
}

// TODO: [refactor] : Probably better suited for the selectors/ module tbh?

async fn extract_job_card(
    //
    client: &Client,
    element: &Element,
    idx: usize,
) -> Result<JobCardData> {
    let card_title = element
        .text()
        .await
        .map_err(|e| {
            crate::prelude::Error::Generic(format!(
                "Failed to extract card title text for job card idx {}: {}",
                idx, e
            ))
        })?
        .replace('·', "")
        .trim()
        .to_string();
    // let job_title = element.

    todo!()
}

// TODO: [same_fn_1] : See associated comment

async fn get_job_cards(
    client: &Client,
    provider: Arc<dyn Provider + Send + Sync>,
    selector_kind: SelectorKind,
) -> Vec<JobCardData> {
    //
    let job_card_selectors = provider.get_job_listing_selectors(selector_kind);
    let job_cards = provider
        .with_elements(client, selector_kind, job_card_selectors.job_card)
        .await
        .unwrap_or_default();
    let mut cards = vec![];

    for (idx, elem) in job_cards.iter().enumerate() {
        cards.push(extract_job_card(client, elem, idx).await.unwrap());
    }

    cards
}
