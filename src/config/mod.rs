mod core;
mod database;
mod driver;
mod driver_type;
mod general;
mod logging;
mod login;
mod output;
mod search;

pub use crate::config::core::AppConfig;
pub use crate::config::database::DatabaseConfig;
pub use crate::config::driver::{DriverConfig, start_driver};
//
pub use crate::config::general::GeneralConfig;
pub use crate::config::logging::LoggingConfig;
pub use crate::config::login::LoginConfig;
pub use crate::config::output::OutputConfig;
pub use crate::config::search::SearchConfig;
use crate::prelude::Result;

pub trait ProcessHandleExt {
    fn abort_handle(&self) -> impl std::future::Future<Output = ()> + Send;
    fn send_stop_signal(&self) -> impl Future<Output = ()>;
}

/// This trait is implemented for any config fields/sub-structs that have
/// some form of path requirement (such as database file path, output file path, etc.)
/// Some are called during initial setup to ensure paths exist, others may be called
/// later in the program lifecycle to validate generated paths.
///
/// Internally this will generally do 2 things:
///
/// 1. Check if the path exists using the existing `WithConfigPath` implementation
///     if it does not exist, set the path to the expected config path.
///
/// 2. Assert that the path exists, erroring if it does not.
///
/// This also prints debug information in debug builds to help trace path issues.
///
#[async_trait::async_trait]
pub trait ValidatePath {
    /// Called to validate and potentially correct the path(s) in the config struct.
    ///
    /// # Returns
    /// Will return `Ok(())` if the path is valid or has been corrected,
    /// otherwise will return an error if the path does not exist,
    /// or an assertion fails.
    ///
    async fn validate_path(&mut self) -> Result<()>;
}
