use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub unfollow_companies: bool,

    /// When it has a value we use that, otherwise
    /// when `None`, will be set to usize::MAX
    pub maximum_iterations: Option<u8>,
    //

    // Impl. later
    // pub headless_mode:    bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            unfollow_companies: true,
            maximum_iterations: Some(255),
        }
    }
}
