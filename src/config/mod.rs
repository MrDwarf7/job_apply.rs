mod core;
mod driver;
mod driver_type;
mod general;
mod logging;
mod login;
mod output;

pub use crate::config::core::AppConfig;
pub use crate::config::driver::{DriverConfig, start_driver};
//
pub use crate::config::general::GeneralConfig;
pub use crate::config::logging::LoggingConfig;
pub use crate::config::login::LoginConfig;
pub use crate::config::output::OutputConfig;

pub trait ProcessHandleExt {
    fn abort_handle(&self) -> impl std::future::Future<Output = ()> + Send;
    fn send_stop_signal(&self) -> impl Future<Output = ()>;
}
