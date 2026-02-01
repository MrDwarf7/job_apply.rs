#![allow(dead_code, unused_imports)]

mod config;
mod constants;
mod db;
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
use fantoccini::elements::{self, Element, ElementRef};
use fantoccini::key::Key;
use fantoccini::wd::TimeoutConfiguration;
use fantoccini::{Client, ClientBuilder, Locator, client};
use futures::StreamExt;
use sqlx::types::chrono::{DateTime, Utc};
use tokio::fs::write;
use tokio::io::AsyncWriteExt;

// pub use self::prelude::{Error, Result, W};
use crate::config::{AppConfig, ProcessHandleExt, start_driver};
use crate::pre_fsm::wait_millis;
pub use crate::prelude::*;
use crate::providers::{Provider, ProviderKind, SelectorKind, get_provider};
use crate::state::{CloseClient, State};
use crate::states::start_state_machine;

pub const GLOBAL_LOG_LEVEL: tracing::Level = tracing::Level::INFO;

// Parsing for the JobCardData here ----

#[tokio::main]
async fn main() -> Result<()> {
    let level = LevelWrapper::from(GLOBAL_LOG_LEVEL);
    init_logger(&level).init();

    // TODO: [check] : impl. check logic (if server isn't running, check on PATH for it, run if
    // available. Otherwise download it (to local dir) and run it (via tokio's Command call).

    debug!("Logging initialized at level: {:?}", level);
    info!("Starting application...");

    let mut config = AppConfig::new().await;
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
        .expect("Provider not found");

    info!("Using provider: {:#?}", provider.name().to_string());

    // TODO: [refactor] : we need another layer of fn here to
    // be able to propogate up to which will call shutdown_app on error.
    // Doing it in main() itself is awkward as hell

    let pre_fsm_result = pre_fsm::run_all_steps(
        &state.client,
        Arc::clone(&provider),
        &state.config,
        SelectorKind::Xpath,
    )
    .await;

    match pre_fsm_result {
        Ok(_) => info!("Pre-FSM steps completed successfully."),
        Err(e) => {
            error!("Pre-FSM process encountered an error: {}", e);
            shutdown_app(state).await?;
            return Err(e);
        }
    }

    // TEST: we first test to see if we can find _all_ of them first.

    let res =
        pre_fsm::all_job_cards(&state.client, Arc::clone(&provider), SelectorKind::Xpath).await;

    match res {
        Ok(_) => info!("Job card extraction completed successfully."),
        Err(e) => {
            error!("Job card extraction encountered an error: {}", e);
            shutdown_app(state).await?;
            return Err(e);
        }
    }

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
