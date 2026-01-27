#![allow(dead_code, unused_imports)]

mod config;
mod constants;
mod error;
mod macros;
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
pub use crate::prelude::*;
use crate::providers::{ProviderKind, SelectorKind, get_provider};
use crate::state::{CloseClient, State};
use crate::states::start_state_machine;

// use tokio::io::AsyncWriteExt;

pub const GLOBAL_LOG_LEVEL: tracing::Level = tracing::Level::DEBUG;

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

    info!("Using provider: {}", provider.name());

    // TODO: [refactor] : we need another layer of fn here to
    // be able to propogate up to which will call shutdown_app on error.
    // Doing it in main() itself is awkward as hell

    pre_fsm::login(&state.client, provider, &state.config.login, SelectorKind::Xpath)
        .await
        .map_err(|e| {
            error!("Login failed: {}", e);
            e
        })?;

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
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    debug!("Shutting down application...");

    // state.close_client().await?;
    state.send_stop_signal().await;
    state.abort_handle().await;

    let _ = state;

    info!("Application shutdown complete.");

    Ok(())
}
