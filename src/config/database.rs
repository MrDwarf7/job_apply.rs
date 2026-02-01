use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::config::ValidatePath;
use crate::prelude::*;
use crate::{WithConfigPath, impl_validation_traits};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub database_uri:      String,
    pub max_lifetime_secs: u64,
    pub max_connections:   u32,
    pub idle_timeout_secs: u64,
    pub min_connections:   u32,

    #[serde(skip)]
    pub database_path: PathBuf,
}

impl_validation_traits!(DatabaseConfig);

impl WithConfigPath for DatabaseConfig {
    const DIRECTORY: &'static str = crate::constants::DATABASE_DIR;
    const FILE: &'static str = crate::constants::DATABASE_FILE;
}

#[async_trait::async_trait]
impl ValidatePath for DatabaseConfig {
    async fn validate_path(&mut self) -> Result<()> {
        if !self.database_path.exists() {
            let should_be = Self::with_config_path().await?;

            #[cfg(debug_assertions)]
            {
                dbg!(&should_be);
                dbg!(&self.database_path);
            }

            self.database_path = should_be.clone();

            assert_eq!(
                should_be, self.database_path,
                "Database path does not match expected config path"
            );
        }

        assert!(
            self.database_path.exists(),
            "Database file does not exist at path: {:?}",
            self.database_path
        );

        Ok(())
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        // TEST: [ensure] : Need to actually make sure this works
        // as I would expect it to.
        // 1. Does the writing of an empty byte (as per the default impl
        //    of WithPath :: Validate :: ValidateFile ) create a valid DATABASE file for SQLite?
        // 2. Does the path creation work as expected? (We're handing a full path to the file here)
        // 3. Does SQLx accept this as a valid database URI?

        let path = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                Self::with_config_path()
                    .await
                    .expect("Failed to get app config file path")
            })
        });

        // let path =
        //     Self::with_config_path().expect("Failed to get database file path for DatabaseConfig");

        let exists = path.exists();
        info!("DatabaseConfig default init: Database file exists: {} at path: {:?}", exists, path);

        let mut database_uri = "sqlite://".to_string();
        database_uri.push_str(path.to_str().expect("Failed to convert path to str"));

        Self {
            // TODO: [impl] : make this use the same system via WithPath +
            //
            // database_uri:      "sqlite://jobs.db".to_string(),
            database_uri,
            // One hour default
            max_lifetime_secs: 3600,
            max_connections: 10,
            idle_timeout_secs: 300, // Five minutes default
            min_connections: 2,     // 2, so we can swap between them without waiting when at min.
            database_path: path,
        }
    }
}

impl From<DatabaseConfig> for sqlx::sqlite::SqlitePoolOptions {
    fn from(db_config: DatabaseConfig) -> Self {
        sqlx::sqlite::SqlitePoolOptions::new()
            .max_lifetime(tokio::time::Duration::from_secs(db_config.max_lifetime_secs))
            .max_connections(db_config.max_connections)
            .idle_timeout(tokio::time::Duration::from_secs(db_config.idle_timeout_secs))
            .min_connections(db_config.min_connections)
    }
}
