use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::impl_validation_traits;
use crate::prelude::{Validate, ValidateDirectory, ValidateFile, WithPath};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub success: SuccessOutputConfig,
    pub failure: FailureOutputConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessOutputConfig {
    pub enabled:   bool,
    pub file_path: PathBuf,
}

impl_validation_traits!(SuccessOutputConfig);

impl WithPath for SuccessOutputConfig {
    const DIRECTORY: &'static str = crate::constants::OUTPUT_DIR;
    const FILE: &'static str = crate::constants::SUCCESS_OUTPUT_FILE;
}

impl Default for SuccessOutputConfig {
    fn default() -> Self {
        let file_path = Self::config_file_path()
            .expect("Failed to get config file path for SuccessOutputConfig");
        Self {
            enabled: false,
            file_path,
        }
    }
}

///////////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureOutputConfig {
    pub enabled:   bool,
    pub file_path: PathBuf,
}

impl_validation_traits!(FailureOutputConfig);

impl WithPath for FailureOutputConfig {
    const DIRECTORY: &'static str = crate::constants::OUTPUT_DIR;
    const FILE: &'static str = crate::constants::FAILURE_OUTPUT_FILE;
}

impl Default for FailureOutputConfig {
    fn default() -> Self {
        let file_path = Self::config_file_path()
            .expect("Failed to get config file path for SuccessOutputConfig");
        Self {
            enabled: false,
            file_path,
        }
    }
}
