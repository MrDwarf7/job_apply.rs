use std::sync::Arc;

use fantoccini::{Client, Locator};

use crate::config::SearchConfig;
use crate::pre_fsm::{ActionPacket, wait_millis};
pub use crate::prelude::*;
use crate::providers::{Provider, ProviderKind, SelectorKind};
use crate::selectors::JobSearchSelectors;
use crate::states::ActionState;

pub async fn keyword_search(
    client: &Client,
    provider: Arc<dyn Provider + Send + Sync>,
    search_config: &SearchConfig,
    selector_kind: SelectorKind, // Css, Xpath, etc.
) -> Result<()> {
    //
    info!(
        "Starting keyword search for provider: {:?} with keywords: {:?}",
        provider.name(),
        search_config.keywords
    );

    let selectors = provider.get_job_search_selectors(selector_kind);
    wait_millis(300).await;

    let search_element = provider
        .with_element(client, selector_kind, selectors.search_input)
        .await?;

    info!("Entering keywords into search input field.");

    let search_action = ActionState::InputText {
        element: &search_element,
        input:   search_config.keywords.clone().join(" "),
    };
    provider.with_action(search_action).await?;
    let enter_key = ActionState::InputKey {
        element: &search_element,
        key:     fantoccini::key::Key::Enter,
    };
    provider.with_action(enter_key).await?;

    info!("Submitting keyword search form.");

    wait_millis(200).await;

    // apply filters
    //
    let action_packet = ActionPacket {
        client,
        provider: provider.clone(),
        config_section: search_config,
        selector_kind,
        selectors: selectors.clone(),
    };
    match provider.name() {
        ProviderKind::LinkedIn => {
            linkedin_filters(&action_packet).await?;
            // linkedin_location(&action_packet).await;
        }
        ProviderKind::Seek => {
            seek_filters().await;
            seek_location().await;
        }
    }

    info!("Keyword search completed for provider: {:?}", provider.name());

    Ok(())
}

// TODO: [impl] : implement the provider-specific keyword search filters and location functions

pub async fn linkedin_filters(
    action_packet: &ActionPacket<'_, SearchConfig, JobSearchSelectors>,
) -> Result<()> {
    let ActionPacket {
        client,
        provider,
        config_section: search_config,
        selector_kind,
        selectors,
    } = action_packet;

    info!("Applying LinkedIn keyword search filters for keywords: {:?}", search_config.keywords);

    let easy_apply_button =
        "//*[@id=\"root\"]/div[2]/div[2]/div[2]/div/div/div/div/div/div/div[2]/div[7]/div/div";

    info!("Locating 'Easy Apply' button element.");
    let easy_apply_button = provider
        .with_element(client, *selector_kind, easy_apply_button)
        .await?;

    let easy_apply_action = ActionState::Click {
        element: easy_apply_button,
    };

    info!("Clicking 'Easy Apply' button to apply filter.");
    provider.with_action(easy_apply_action).await?;
    wait_millis(500).await;
    info!("Easy Apply filter applied successfully.");

    // // easy apply button
    //
    // //*[@id="root"]/div[2]/div[2]/div[2]/div/div/div/div/div/div/div[2]/div[7]/div/div
    // let easy_apply_button = "button:contains('Easy Apply')";

    Ok(())
}

pub async fn linkedin_location(action_packet: &ActionPacket<'_, SearchConfig, JobSearchSelectors>) {
    //
}

pub async fn seek_filters() {
    unimplemented!("Seek provider keyword search filters not implemented yet.")
}

pub async fn seek_location() {
    unimplemented!("Seek provider keyword search location not implemented yet.")
}
