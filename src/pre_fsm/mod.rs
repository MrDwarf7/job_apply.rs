mod detect_login_failure;
mod job_board;
mod keyword_search;
mod login_action;

use std::sync::Arc;

use fantoccini::{Client, Locator};

use crate::config::LoginConfig;
pub use crate::pre_fsm::detect_login_failure::detect_login_failure;
pub use crate::pre_fsm::job_board::job_board;
pub use crate::pre_fsm::keyword_search::keyword_search;
pub use crate::pre_fsm::login_action::login;
pub use crate::prelude::*;
use crate::providers::{Provider, ProviderKind, SelectorKind};
use crate::selectors::{self, LoginSelectors};
use crate::states::ActionState;

pub async fn wait_millis(dur: u64) {
    tokio::time::sleep(tokio::time::Duration::from_millis(dur)).await;
}

/// Little helper structure used
/// to bundle up parameters for action sets
pub struct ActionPacket<'a, C, S> {
    client:         &'a Client,
    provider:       Arc<dyn Provider + Send + Sync>,
    // login_config:  &'a LoginConfig,
    config_section: &'a C,
    selector_kind:  SelectorKind, // Css, Xpath, etc.
    // selectors:     LoginSelectors,
    selectors:      S,
}
