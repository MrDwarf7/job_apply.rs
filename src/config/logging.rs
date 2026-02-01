use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::config::ValidatePath;
use crate::impl_validation_traits;
use crate::prelude::{Result, Validate, ValidateDirectory, ValidateFile, WithConfigPath};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub log_level:     LoggingLevel,
    pub debug_logging: FileLoggingConfig,
}

#[rustfmt::skip]
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd, strum::Display, strum::EnumString)]
pub enum LoggingLevel {
    #[serde(rename = "trace")]
    #[strum(serialize = "trace")]
    Trace,

    #[serde(rename = "debug")]
    #[strum(serialize = "debug")]
    Debug,

    #[serde(rename = "info")]
    #[strum(serialize = "info")]
    #[default]
    Info,

    #[serde(rename = "warn")]
    #[strum(serialize = "warn")]
    Warn,

    #[serde(rename = "error")]
    #[strum(serialize = "error")]
    Error,
}

impl From<LoggingLevel> for tracing::Level {
    fn from(level: LoggingLevel) -> Self {
        match level {
            LoggingLevel::Trace => tracing::Level::TRACE,
            LoggingLevel::Debug => tracing::Level::DEBUG,
            LoggingLevel::Info => tracing::Level::INFO,
            LoggingLevel::Warn => tracing::Level::WARN,
            LoggingLevel::Error => tracing::Level::ERROR,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileLoggingConfig {
    pub enabled:   bool,
    pub file_path: PathBuf,
}

impl FileLoggingConfig {
    pub fn new<P: AsRef<Path>>(enabled: bool, file_path: P) -> Self {
        Self {
            enabled,
            file_path: file_path.as_ref().to_path_buf(),
        }
    }
}

impl_validation_traits!(FileLoggingConfig);

impl WithConfigPath for FileLoggingConfig {
    const DIRECTORY: &'static str = crate::constants::OUTPUT_DIR;
    const FILE: &'static str = crate::constants::LOG_FILE;
}

#[async_trait::async_trait]
impl ValidatePath for FileLoggingConfig {
    async fn validate_path(&mut self) -> Result<()> {
        if !self.file_path.exists() {
            let should_be = Self::with_config_path().await?;

            #[cfg(debug_assertions)]
            {
                dbg!(&should_be);
                dbg!(&self.file_path);
            }

            self.file_path = should_be.clone();

            assert_eq!(
                should_be, self.file_path,
                "Log file path does not match expected config path"
            );
        }
        assert!(self.file_path.exists(), "Log file does not exist at path: {:?}", self.file_path);
        Ok(())
    }
}

impl Default for FileLoggingConfig {
    fn default() -> Self {
        let file_path = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                Self::with_config_path()
                    .await
                    .expect("Failed to get FileLoggingConfig log file path")
            })
        });

        // let file_path = Self::with_config_path().expect("Failed to get log file path");

        Self {
            enabled: false,
            file_path,
        }
    }
}
