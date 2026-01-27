// in-crate Error type

use std::io::Write as _;
use std::path::{Path, PathBuf};

#[allow(unused_imports)]
pub use tracing::{debug, error, info, trace, warn};

pub use crate::error::Error;

// in-crate result type
pub type Result<T> = std::result::Result<T, Error>;

// Wrapper struct
#[allow(dead_code)]
pub struct W<T>(pub T);

#[allow(dead_code)]
pub fn time<T>(t: &str, f: impl FnOnce() -> T) -> T {
    eprintln!("{t}: Starting");
    let start = std::time::Instant::now();
    let r = f();
    let elapsed = start.elapsed();
    eprintln!("{t}: Elapsed: {elapsed:?}");
    r
}

#[allow(dead_code)]
#[cfg(not(debug_assertions))]
pub fn current_path() -> Result<std::path::PathBuf> {
    std::env::current_exe()
        .map_err(|e| e.to_string())
        .map_err(Error::Generic)
}

#[allow(dead_code)]
#[cfg(debug_assertions)]
pub fn current_path() -> Result<std::path::PathBuf> {
    std::env::current_dir()
        .map_err(|e| e.to_string())
        .map_err(Error::Generic)
}

pub trait ValidateDirectory {
    /// Takes in a base directory path,
    /// performs a check to ensure it exists, creating it it doesn't,
    ///
    /// # Returns
    /// * Returns a new PathBuf representing the validated directory path.
    fn validate_directory<P: AsRef<Path>>(assumed_dir: P) -> Result<PathBuf> {
        if !assumed_dir.as_ref().exists() {
            std::fs::create_dir_all(&assumed_dir).expect("Failed to create directory");
            assert!(assumed_dir.as_ref().exists(), "Assertion failed: base_dir.as_ref().exists()");
            Ok(assumed_dir.as_ref().to_path_buf())
        } else {
            assert!(assumed_dir.as_ref().is_dir(), "Assertion failed: base_dir.as_ref().is_dir()");
            assert!(assumed_dir.as_ref().exists(), "Assertion failed: base_dir.as_ref().exists()");
            Ok(assumed_dir.as_ref().to_path_buf())
        }
    }
}

/// Validates the existence of a file within a specified directory,
/// Requires access to the `ValidateDirectory` trait to ensure the directory is valid first.
/// Then builds out the full file path and checks for its existence,
pub trait ValidateFile: ValidateDirectory {
    fn validate_file<P: AsRef<Path>>(assumed_dir: P, assumed_filename: P) -> Result<PathBuf> {
        let valid_dir = Self::validate_directory(&assumed_dir).expect("Failed to validate directory");
        if !valid_dir.join(assumed_filename.as_ref()).exists() {
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .truncate(false)
                .write(true)
                .open(valid_dir.join(assumed_filename.as_ref()))
                .expect("Failed to create file");
            file.write_all(b"").expect("Failed to write to file");

            assert!(file.metadata().is_ok(), "Assertion failed: file.metadata().is_ok()");
            assert!(file.metadata().unwrap().is_file(), "Assertion failed: file.metadata().unwrap().is_file()");
            drop(file);
        }
        Ok(valid_dir.join(assumed_filename.as_ref()))
    }
}

/// Trait to consolidate both directory and file validation.
pub trait Validate: ValidateFile {
    fn validate<P: AsRef<Path>>(assumed_dir: P, assumed_filename: P) -> Result<std::path::PathBuf> {
        Self::validate_file(assumed_dir, assumed_filename)
    }
}

/// This trait has default implementations for getting the current path
/// and appending additional path segments to it.
///
/// See the docs on `Self::from_current` for more details.
pub trait WithPath: Validate {
    const DIRECTORY: &'static str;
    const FILE: &'static str;

    fn config_file_path() -> Result<PathBuf> {
        let current = current_path().expect("Failed to get current path");
        // We make assumptions here because the validation logic handles
        // cases where the directory or file do not exist.
        let assumed_dir = current.join(Self::DIRECTORY);
        let assumed_file = assumed_dir.join(Self::FILE);

        let validated = Self::validate(assumed_dir, assumed_file);

        match validated {
            Ok(path) => Ok(path),
            Err(e) => Err(e),
        }
    }
}

pub type TracingSubscriber = tracing_subscriber::fmt::SubscriberBuilder<
    tracing_subscriber::fmt::format::DefaultFields,
    tracing_subscriber::fmt::format::Format<tracing_subscriber::fmt::format::Full>,
    // tracing_subscriber::EnvFilter,
>;

// pub fn init_logger() -> TracingSubscriber {
//     tracing_subscriber::fmt()
//         .with_level(true)
//         .with_ansi(true)
//         .with_line_number(true)
//         .with_thread_ids(true)
//     // .with_env_filter(level)
//     // .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
//     // .with_timer(tracing_subscriber::fmt::time::SystemTime)
// }

// More complex implementation that allows for custom levels and environment filters
// requires the use of the impl From<VerbosityLevel> for LevelWrapper<L, E> functionality in
// `cli.rs` file.

#[derive(Clone, Debug)]
pub struct LevelWrapper<L, E>
where
    L: Into<tracing::Level>,
    E: Into<tracing_subscriber::filter::EnvFilter>,
{
    pub level:      L,
    pub env_filter: E,
}

impl From<tracing::Level> for LevelWrapper<tracing::Level, &'static str> {
    fn from(level: tracing::Level) -> Self {
        Self {
            //
            level,
            env_filter: "my_crate=info",
        }
    }
}

pub fn init_logger<L, E>(level: &LevelWrapper<L, E>) -> TracingSubscriber
where
    L: Into<tracing::Level> + Clone,
    E: Into<tracing_subscriber::filter::EnvFilter>,
{
    let max_level: tracing::Level = level.level.clone().into();
    // let env_level: tracing_subscriber::filter::EnvFilter = level.env_filter.into();
    tracing_subscriber::fmt()
        .with_level(true)
        .with_ansi(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_max_level(max_level)
    // .with_env_filter(env_level)
    // .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
    // .with_timer(tracing_subscriber::fmt::time::SystemTime)
}

// We use the trait impl. in this crate

// #[allow(dead_code)]
// #[cfg(not(debug_assertions))]
// pub(crate) fn current_path() -> Result<std::path::PathBuf> {
//     std::env::current_exe()
//         .map_err(|e| e.to_string())
//         .map_err(Error::Generic)
// }
//
// #[allow(dead_code)]
// #[cfg(debug_assertions)]
// pub(crate) fn current_path() -> Result<std::path::PathBuf> {
//     std::env::current_dir()
//         .map_err(|e| e.to_string())
//         .map_err(Error::Generic)
// }
