use std::path::PathBuf;

use config::Config;
use serde::{Deserialize, Serialize};

use crate::config::database::DatabaseConfig;
use crate::config::{
    DriverConfig,
    GeneralConfig,
    LoggingConfig,
    LoginConfig,
    OutputConfig,
    ProcessHandleExt,
    SearchConfig,
    ValidatePath as _,
};
use crate::impl_validation_traits;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub general: GeneralConfig,

    pub database: DatabaseConfig,

    pub driver: DriverConfig,

    pub logging: LoggingConfig,

    pub output: OutputConfig,

    pub login: LoginConfig,

    #[serde(default)]
    pub search: SearchConfig,

    #[serde(skip)]
    pub config_path: PathBuf,
}

impl ProcessHandleExt for AppConfig {
    async fn abort_handle(&self) {
        Box::pin(async move {
            if let Some(driver_process) = &self.driver.driver_process {
                driver_process.abort_handle().await;
            } else {
                warn!("No driver process handle to abort.");
            }
        })
        .await;
    }

    async fn send_stop_signal(&self) {
        Box::pin(async move {
            if let Some(driver_process) = &self.driver.driver_process {
                driver_process.send_stop_signal().await;
            } else {
                warn!("No driver process handle to send stop signal to.");
            }
        })
        .await;
    }
}

impl_validation_traits!(AppConfig);

impl WithConfigPath for AppConfig {
    const DIRECTORY: &'static str = crate::constants::CONFIG_DIR;
    const FILE: &'static str = crate::constants::CONFIG_FILE;
}

impl AppConfig {
    // TODO: We will need ot update the global logging level based on the config file
    // ( If the user provided one that is )

    pub async fn new() -> Self {
        info!("Initializing AppConfig with default values.");
        let config_path = crate::prelude::current_path()
            .expect("Failed to get current path")
            .join(crate::constants::CONFIG_DIR)
            .join(crate::constants::CONFIG_FILE);

        let config_path_own = Self::with_config_path()
            .await
            .expect("Failed to get app config file path");

        let config_path = if !config_path.exists() || !config_path.is_file() {
            warn!(
                "You've provided an invalid config file path: {:?}. Using default configuration.",
                config_path
            );
            config_path_own
        } else {
            info!("Using provided config file path: {:?}", config_path);
            config_path.to_path_buf()
        };

        let c = Config::builder()
            .add_source(config::File::with_name(
                config_path.to_str().expect("Invalid config file path"),
            ))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .expect("Failed to build config");

        // BUG: [logical_ordering] :
        // If the config file exists, we
        // never check the paths embedded in the actual config file itself.
        // This means that as long as there is a valid 'enough' config file,
        // we have the potential to skip database creation,
        // even if the database itself is missing.

        let mut app_config: AppConfig = match c.try_deserialize() {
            Ok(cfg) => cfg,
            Err(e) => {
                warn!("Failed to deserialize config from file: {e}. Using default configuration.");
                fallback_deserialize(config_path.clone()).await
            }
        };

        app_config.config_path = config_path;

        app_config.database.validate_path().await
            .unwrap_or_else(|e| {
                warn!("Database file validation failed: {e}. A new database has been created at the path provided in the config file: {}.",
                    app_config.database.database_path.display());
            });

        app_config
    }
}

async fn fallback_deserialize(config_path: PathBuf) -> AppConfig {
    // let config_path_c = config_path.clone();
    warn!("Writing default config to file: {config_path:?}");

    let s = AppConfig {
        config_path: config_path.clone(),
        ..Default::default()
    };

    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            tokio::fs::write(
                // &config_path_c,
                &config_path,
                toml::to_string_pretty(&s).expect("Failed to serialize default config to TOML"),
            )
            .await
            .map_err(|e| {
                error!("Failed to write default config to file: {e}");
                e
            })
            .ok();
        })
    });
    s
}

impl Default for AppConfig {
    fn default() -> Self {
        // let runtime = tokio::runtime::Builder::new_current_thread()
        //     .enable_all()
        //     .build()
        //     .expect("AppConfig :: Default :: Failed to create Tokio runtime");

        let path = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                Self::with_config_path()
                    .await
                    .expect("Failed to get app config file path")
            })
        });

        let config_path = path;
        // Self::with_config_path().expect("Failed to get app config file path");
        Self {
            general: GeneralConfig::default(),
            database: DatabaseConfig::default(),
            driver: DriverConfig::default(),
            logging: LoggingConfig::default(),
            output: OutputConfig::default(),
            login: LoginConfig::default(),
            search: SearchConfig::default(),
            config_path,
        }
    }
}
