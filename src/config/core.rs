use std::path::PathBuf;

use config::Config;
use serde::{Deserialize, Serialize};

use crate::config::{DriverConfig, GeneralConfig, LoggingConfig, LoginConfig, OutputConfig, ProcessHandleExt};
use crate::impl_validation_traits;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub general: GeneralConfig,

    pub driver: DriverConfig,

    pub logging: LoggingConfig,

    pub output: OutputConfig,

    pub login: LoginConfig,

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

impl WithPath for AppConfig {
    const DIRECTORY: &'static str = crate::constants::CONFIG_DIR;
    const FILE: &'static str = crate::constants::CONFIG_FILE;
}

impl AppConfig {
    // TODO: We will need ot update the global logging level based on the config file
    // ( If the user provided one that is )

    pub fn new() -> Self {
        info!("Initializing AppConfig with default values.");
        let config_path = crate::prelude::current_path()
            .expect("Failed to get current path")
            .join(crate::constants::CONFIG_DIR)
            .join(crate::constants::CONFIG_FILE);

        let config_path_own = Self::config_file_path().expect("Failed to get app config file path");

        let config_path = if !config_path.exists() || !config_path.is_file() {
            warn!("You've provided an invalid config file path: {:?}. Using default configuration.", config_path);
            config_path_own
        } else {
            info!("Using provided config file path: {:?}", config_path);
            config_path.to_path_buf()
        };

        let c = Config::builder()
            .add_source(config::File::with_name(config_path.to_str().expect("Invalid config file path")))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .expect("Failed to build config");

        let mut app_config = c.try_deserialize().unwrap_or_else(|e| {
            warn!("Failed to deserialize config from file: {}. Using default configuration.", e);

            let config_path_c = config_path.clone();
            warn!("Writing default config to file: {:?}", config_path_c);

            let s = AppConfig {
                general:     GeneralConfig::default(),
                driver:      DriverConfig::default(),
                logging:     LoggingConfig::default(),
                output:      OutputConfig::default(),
                login:       LoginConfig::default(),
                config_path: config_path_c.clone(),
            };
            std::fs::write(
                &config_path_c,
                toml::to_string_pretty(&s).expect("Failed to serialize default config to TOML"),
            )
            .map_err(|e| {
                error!("Failed to write default config to file: {}", e);
                e
            })
            .ok();
            s
        });

        app_config.config_path = config_path;

        app_config
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        // let config_path = Self::config_file_path().expect("Failed to get app config file path");
        Self::new()
    }
}
