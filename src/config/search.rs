use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub use_keywords: bool,
    pub keywords:     Vec<String>,
    pub location:     Option<String>,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            use_keywords: false,
            keywords:     vec![],
            location:     None,
        }
    }
}
