mod detect_login_failure;
mod job_board;
mod job_card;
mod keyword_search;
mod login_action;

use std::sync::Arc;

use fantoccini::{Client, Locator};

use crate::config::{AppConfig, LoginConfig};
pub use crate::pre_fsm::detect_login_failure::detect_login_failure;
pub use crate::pre_fsm::job_board::job_board;
pub use crate::pre_fsm::job_card::all_job_cards;
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

// MUST call shutdown_app on error propagation
// shutdown_app(state).await?;
//
pub async fn run_all_steps(
    client: &Client,
    provider: Arc<dyn Provider + Send + Sync>,
    config: &AppConfig,
    selector_kind: SelectorKind, // Css, Xpath, etc.
) -> Result<()> {
    //

    let login_result = login(client, Arc::clone(&provider), &config.login, selector_kind)
        .await
        .map_err(|e| {
            error!("Login failed: {}", e);
            e
        });

    match login_result {
        Ok(_) => info!("Login successful."),
        Err(e) => {
            error!("Login process encountered an error: {}", e);
            return Err(e);
        }
    }

    let job_board_result = job_board(client, Arc::clone(&provider), selector_kind).await;

    match job_board_result {
        Ok(_) => info!("Navigated to job board successfully."),
        Err(e) => {
            error!("Job board navigation encountered an error: {}", e);
            return Err(e);
        }
    }

    // If we want to use keywords,
    // then we have to filter down by them first to get
    // access to the other filters
    if config.search.use_keywords {
        if config.search.keywords.is_empty() {
            warn!("No keywords specified in config, but 'use_keywords' is set to true.");
            return Err(Error::NoKeywordsSet(
                "No keywords specified in config, but 'use_keywords' is set to true.".to_string(),
            ));
        }

        assert!(!config.search.keywords.is_empty(), "Check for keyword length failed!");

        // TODO: [impl] :
        // impl the search box selection & input here to filter
        // the job listings based on keywords from config
        let keyword_result =
            keyword_search(client, Arc::clone(&provider), &config.search, SelectorKind::Xpath);
        match keyword_result.await {
            Ok(_) => info!("Keyword search applied successfully."),
            Err(e) => {
                error!("Keyword search encountered an error: {}", e);
                return Err(e);
            }
        }
    } else {
        // TODO: [impl] : use the 'show all' button to list all jobs for linkedin

        // No keywords?
        // Then we can click the show all button ( for linkedin at least)
    }

    Ok(())
}
