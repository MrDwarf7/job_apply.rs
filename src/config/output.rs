use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::config::ValidatePath;
use crate::impl_validation_traits;
use crate::prelude::{Result, Validate, ValidateDirectory, ValidateFile, WithConfigPath};

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

impl WithConfigPath for SuccessOutputConfig {
    const DIRECTORY: &'static str = crate::constants::OUTPUT_DIR;
    const FILE: &'static str = crate::constants::SUCCESS_OUTPUT_FILE;
}

#[async_trait::async_trait]
impl ValidatePath for SuccessOutputConfig {
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
                "Success output file path does not match expected config path"
            );
        }

        assert!(
            self.file_path.exists(),
            "Success output file does not exist at path: {:?}",
            self.file_path
        );

        Ok(())
    }
}

impl Default for SuccessOutputConfig {
    fn default() -> Self {
        let file_path = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                Self::with_config_path()
                    .await
                    .expect("Failed to get SuccessOutputConfig file path")
            })
        });

        // let file_path = Self::with_config_path()
        //     .expect("Failed to get config file path for SuccessOutputConfig");
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

impl WithConfigPath for FailureOutputConfig {
    const DIRECTORY: &'static str = crate::constants::OUTPUT_DIR;
    const FILE: &'static str = crate::constants::FAILURE_OUTPUT_FILE;
}

#[async_trait::async_trait]
impl ValidatePath for FailureOutputConfig {
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
                "Failure output file path does not match expected config path"
            );
        }

        assert!(
            self.file_path.exists(),
            "Failure output file does not exist at path: {:?}",
            self.file_path
        );

        Ok(())
    }
}

impl Default for FailureOutputConfig {
    fn default() -> Self {
        let file_path = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                Self::with_config_path()
                    .await
                    .expect("Failed to get FailureOutputConfig file path")
            })
        });

        // let file_path = Self::with_config_path()
        //     .expect("Failed to get config file path for SuccessOutputConfig");

        Self {
            enabled: false,
            file_path,
        }
    }
}
