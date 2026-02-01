use std::sync::Arc;

use fantoccini::{Client, Locator};

use crate::config::LoginConfig;
use crate::pre_fsm::{ActionPacket, detect_login_failure, wait_millis};
use crate::prelude::*;
use crate::providers::{Provider, ProviderKind, SelectorKind};
use crate::selectors::{self, LoginSelectors};
use crate::states::ActionState;

pub async fn login(
    client: &Client,
    provider: Arc<dyn Provider + Send + Sync>,
    login_config: &LoginConfig,
    selector_kind: SelectorKind, // Css, Xpath, etc.
) -> Result<()> {
    let login_url = login_config.login_url.clone();
    let selectors = provider.get_login_selectors(selector_kind);

    // used to poll the element that indicates login failure,
    // we check before exiting the function
    let (failed_tx, mut failed_rx) = tokio::sync::mpsc::channel::<String>(1);

    // we need to poll the element to check if at any point we've failed to login
    detect_login_failure(client, selectors.clone(), selector_kind, provider.clone(), failed_tx)
        .await;

    info!("Navigating to login page: {}", login_url);
    client.goto(&login_url).await.map_err(|e| {
        Error::FantocciniCmdError { error: Box::new(e) }
        // Error::Generic(format!("Failed to navigate to login URL '{}': {}", login_url, e))
    })?;

    wait_millis(500).await;

    username(ActionPacket {
        client,
        provider: Arc::clone(&provider),
        config_section: login_config,
        selector_kind,
        selectors: selectors.clone(),
    })
    .await?;

    password(&ActionPacket {
        client,
        provider: provider.clone(),
        config_section: login_config,
        selector_kind,
        selectors: selectors.clone(),
    })
    .await?;

    info!("Submitting login form");
    let login_button = provider
        .with_element(client, selector_kind, selectors.submit_button)
        .await?;

    provider
        .with_action(ActionState::Click {
            element: login_button,
        })
        .await?;

    wait_millis(200).await;

    let msg_check = failed_rx.try_recv();

    match msg_check {
        Ok(failure_message) => {
            return Err(Error::LoginFailure(failure_message));
        }
        Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {
            info!("No login failure detected, proceeding.");
        }
        Err(e) => {
            error!("Error checking for login failure: {}", e);
        }
    }
    failed_rx.close();
    drop(failed_rx);

    info!("Login process completed");

    Ok(())
}

pub async fn username(action_packet: ActionPacket<'_, LoginConfig, LoginSelectors>) -> Result<()> {
    info!("Filling in username");

    let ActionPacket {
        client,
        provider,
        config_section: login_config,
        selector_kind,
        selectors,
    } = action_packet;

    let username_input = provider
        .with_element(client, selector_kind, selectors.username_field)
        .await?;

    username_input.clear().await.map_err(|e| {
        Error::Generic(format!(
            "Failed to clear username input field before entering username: {}",
            e
        ))
    })?;

    let username_action = ActionState::InputText {
        element: &username_input,
        input:   login_config.username.clone(),
    };
    provider.with_action(username_action).await?;
    wait_millis(200).await;
    Ok(())
}

pub async fn password(action_packet: &ActionPacket<'_, LoginConfig, LoginSelectors>) -> Result<()> {
    info!("Filling in password");

    let ActionPacket {
        client,
        provider,
        config_section: login_config,
        selector_kind,
        selectors,
    } = action_packet;

    let password_input = provider
        .with_element(client, *selector_kind, selectors.password_field)
        .await?;

    password_input.clear().await.map_err(|e| {
        Error::Generic(format!(
            "Failed to clear password input field before entering password: {}",
            e
        ))
    })?;

    let password_action = ActionState::InputText {
        element: &password_input,
        input:   login_config.password.clone(),
    };

    provider.with_action(password_action).await?;
    wait_millis(200).await;
    Ok(())
}
