use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::impl_validation_traits;
use crate::prelude::{Validate, ValidateDirectory, ValidateFile, WithPath};

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

impl WithPath for FileLoggingConfig {
    const DIRECTORY: &'static str = crate::constants::OUTPUT_DIR;
    const FILE: &'static str = crate::constants::LOG_FILE;
}

impl Default for FileLoggingConfig {
    fn default() -> Self {
        let file_path = Self::config_file_path().expect("Failed to get log file path");
        Self {
            enabled: false,
            file_path,
        }
    }
}
