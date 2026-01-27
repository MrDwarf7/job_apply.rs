use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use which::which;

use crate::prelude::{Error, Result};

#[derive(
    Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Serialize, Deserialize, strum::Display, strum::EnumString,
)]
pub enum DriverType {
    #[serde(rename = "chrome")]
    Chrome,
    #[serde(rename = "chromium")]
    #[strum(serialize = "chromedriver")]
    Chromium,
    // Firefox,
    // Safari,
    // Edge,
}

pub fn find_driver(driver: DriverType) -> Result<PathBuf> {
    let driver_name = match driver {
        DriverType::Chrome | DriverType::Chromium => "chromedriver",
        // Other's here later
    };
    #[cfg(target_os = "windows")]
    let driver_name = format!("{}.exe", driver_name);

    #[cfg(not(target_os = "windows"))]
    let loc = which(driver_name);

    if let Err(e) = loc {
        return Err(Error::DriverNotFound {
            driver: driver_name.to_string(),
            source: e,
        });
    }

    Ok(loc.unwrap())
}
