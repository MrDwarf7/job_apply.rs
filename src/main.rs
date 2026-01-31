#![allow(dead_code, unused_imports)]

mod config;
mod constants;
mod error;
mod macros;
mod pre_fsm;
mod prelude;
mod providers;
mod selectors;
mod state;
mod states;

use std::collections::HashMap;
use std::io::Write as StdWrite;
use std::sync::{Arc, Mutex};

use fantoccini::actions::{
    InputSource,
    KeyAction,
    KeyActions,
    MOUSE_BUTTON_LEFT,
    MouseActions,
    NullAction,
    NullActions,
    PointerAction,
};
use fantoccini::elements::{Element, ElementRef};
use fantoccini::key::Key;
use fantoccini::wd::TimeoutConfiguration;
use fantoccini::{Client, ClientBuilder, Locator, client};
use futures::StreamExt;
use tokio::fs::write;
use tokio::io::AsyncWriteExt;

// pub use self::prelude::{Error, Result, W};
use crate::config::{AppConfig, ProcessHandleExt, start_driver};
use crate::pre_fsm::wait_millis;
pub use crate::prelude::*;
use crate::providers::{ProviderKind, SelectorKind, get_provider};
use crate::state::{CloseClient, State};
use crate::states::start_state_machine;

// use tokio::io::AsyncWriteExt;

pub const GLOBAL_LOG_LEVEL: tracing::Level = tracing::Level::INFO;

pub struct Location {
    state:   String, // New South Wales
    country: String, // Australia
}

// pub enum EmploymentType {
//     FullTime,
//     PartTime,
//     Contract,
//     Temporary,
//     Internship,
//     Volunteer,
//     Other,
// }

pub enum WorkKind {
    OnSite,
    Remote,
    Hybrid,
}

pub struct DateInfo {
    // We'd want to actually strip the 'Posted on' part, and use a proper DT type
    full_date: String, // Posted on January 19, 2026, 10:43 AM
    relative:  String, // 1 week ago
}

pub struct JobCardData {
    // there's also random dot things in some of the data
    // ·
    // whatever that is... we need to strip those out
    idx:                   usize,
    card_title:            String, // invisible // Full Stack Engineer (Verified job)
    job_title:             String, // Full Stack Engineer
    company_name:          String, // Nityo Infotech
    location:              Location, // { state: New South Wales, contry:  Australia }
    work_type:             WorkKind, // (Hybrid)
    is_actively_reviewing: bool,   // Actively reviewing applicants
    already_viewed:        bool,   // Viewed
    posted_date:           DateInfo, //  Posted on January 19, 2026, 10:43 AM
    has_easy_apply:        bool,   // the icon for it //Easy Apply
}

// Parsing for the JobCardData here ----

#[tokio::main]
async fn main() -> Result<()> {
    let level = LevelWrapper::from(GLOBAL_LOG_LEVEL);
    init_logger(&level).init();

    // TODO: [check] : impl. check logic (if server isn't running, check on PATH for it, run if
    // available. Otherwise download it (to local dir) and run it (via tokio's Command call).

    debug!("Logging initialized at level: {:?}", level);
    info!("Starting application...");

    let mut config = AppConfig::new();
    dbg!(&config);

    let client = start_driver(&mut config.driver)
        .await
        .unwrap_or_else(|e| panic!("Failed to start driver: {}", e));
    info!("Driver process handle: {:?}", config.driver.driver_process);

    let state = State::new(config, client);

    let provider = get_provider(ProviderKind::from(state.config.login.provider.as_str())) // replace with config value later
        .ok_or_else(|| {
            panic!("Failed to get provider");
        })
        .unwrap();

    info!("Using provider: {:#?}", provider.name().to_string());

    // TODO: [refactor] : we need another layer of fn here to
    // be able to propogate up to which will call shutdown_app on error.
    // Doing it in main() itself is awkward as hell

    let login_result = pre_fsm::login(
        &state.client,
        Arc::clone(&provider),
        &state.config.login,
        SelectorKind::Xpath,
    )
    .await
    .map_err(|e| {
        error!("Login failed: {}", e);
        e
    });

    match login_result {
        Ok(_) => info!("Login successful."),
        Err(e) => {
            error!("Login process encountered an error: {}", e);
            shutdown_app(state).await?;
            return Err(e);
        }
    }

    let job_board_result =
        pre_fsm::job_board(&state.client, Arc::clone(&provider), SelectorKind::Xpath).await;

    match job_board_result {
        Ok(_) => info!("Navigated to job board successfully."),
        Err(e) => {
            error!("Job board navigation encountered an error: {}", e);
            shutdown_app(state).await?;
            return Err(e);
        }
    }

    // If we want to use keywords,
    // then we have to filter down by them first to get
    // access to the other filters
    if state.config.search.use_keywords {
        if state.config.search.keywords.is_empty() {
            warn!("No keywords specified in config, but 'use_keywords' is set to true.");
            return Err(Error::NoKeywordsSet(
                "No keywords specified in config, but 'use_keywords' is set to true.".to_string(),
            ));
        }

        assert!(!state.config.search.keywords.is_empty(), "Check for keyword length failed!");

        // TODO: [impl] :
        // impl the search box selection & input here to filter
        // the job listings based on keywords from config
        let keyword_result = pre_fsm::keyword_search(
            &state.client,
            Arc::clone(&provider),
            &state.config.search,
            SelectorKind::Xpath,
        );
        match keyword_result.await {
            Ok(_) => info!("Keyword search applied successfully."),
            Err(e) => {
                error!("Keyword search encountered an error: {}", e);
                shutdown_app(state).await?;
                return Err(e);
            }
        }
    } else {
        // TODO: [impl] : use the 'show all' button to list all jobs for linkedin

        // No keywords?
        // Then we can click the show all button ( for linkedin at least)
    }

    // TEST: we first test to see if we can find _all_ of them first.

    let job_card_selectors = provider.get_job_listing_selectors(SelectorKind::Xpath);
    wait_millis(1500).await;

    let all_job_cards = provider
        .with_elements(&state.client, SelectorKind::Xpath, job_card_selectors.job_card)
        .await;

    all_job_cards
        .map(|cards| {
            info!("Found {} job cards on the page.", cards.len());

            tokio::spawn(async move {
                if !cards.is_empty() {
                    // TODO: Move the collection & parsing into a struct ( JobCardData )
                    for (i, card) in cards.iter().enumerate() {
                        let tag_name = card.tag_name().await.unwrap_or_default();
                        let card_text = card.text().await.unwrap_or_default();
                        info!(
                            "Job Card {}: Tag Name: {:?},\n Text Snippet: {}\n\n",
                            i + 1,
                            tag_name,
                            card_text
                        );

                        // /// Output Example:
                        // Job Card 1: Tag Name: "div",
                        // Text Snippet: Full Stack Engineer (Verified job)
                        // Full Stack Engineer
                        // Nityo Infotech
                        // New South Wales, Australia (Hybrid)
                        // Actively reviewing applicants
                        // Viewed
                        //  ·
                        // Posted on January 19, 2026, 10:43 AM
                        // 1 week ago
                        //  ·
                        //  Easy Apply
                    }
                } else {
                    warn!("No job cards found on the page.");
                }
            })
        })
        .map_err(|e| {
            error!("Error finding job cards: {}", e);
            e
        })?;

    /////////////////////////

    // We need to handle getting sent a sign-in link thing

    // If login successful,
    // We need to move to the relevant site section for
    // job board/pages/browsing/etc.
    // Set any relevant filters from config (TODO)
    //
    // then start the main FSM

    // In state machine ->
    // Search for job listings ->
    // Identify ones that match criteria ->
    // Select and click apply ->
    //      go through application process FSM ->
    //      hit submit ---->
    //
    // Next Job -> loop

    shutdown_app(state).await?;

    Ok(())
}

/// Runs various destruction/cleanup tasks,
/// then takes direct ownerhsip of the app_config to drop it.
pub async fn shutdown_app<S: ProcessHandleExt + CloseClient + 'static>(state: S) -> Result<()> {
    // We may need to go back to handing through just AppConfig + Client separately,
    // due to the fact that client.close() takes `self` not `&self`.

    // TEST: while we're developing, just wait a few seconds to read logs and output etc.
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    debug!("Shutting down application...");

    // state.close_client().await?;
    state.send_stop_signal().await;
    state.abort_handle().await;

    let _ = state;

    info!("Application shutdown complete.");

    Ok(())
}
