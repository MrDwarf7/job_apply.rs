use std::sync::Arc;

use fantoccini::{Client, Locator};

use crate::config::LoginConfig;
use crate::pre_fsm::wait_millis;
pub use crate::prelude::*;
use crate::providers::{Provider, ProviderKind, SelectorKind};
use crate::selectors::{self, LoginSelectors};
use crate::states::ActionState;

pub async fn detect_login_failure(
    client: &Client,
    selectors: LoginSelectors,
    selector_kind: SelectorKind,
    provider: Arc<dyn Provider + Send + Sync>,
    failed_tx: tokio::sync::mpsc::Sender<String>,
) {
    let client_arc = Arc::new(client.clone());

    // let total_timer = tokio::time::Instant::now();
    // let allowed_duration = tokio::time::Duration::from_secs(60);

    tokio::spawn(async move {
        let client = client_arc.clone();
        let fail_element = provider
            .with_element(&client, selector_kind, selectors.failed_attempt_indicator)
            .await;

        loop {
            if let Err(e) = &fail_element {
                error!("Error locating failed attempt indicator element: {}", e);
            } else if let Ok(elem) = &fail_element
                && elem.is_displayed().await.unwrap_or(false)
            {
                let _ = failed_tx
                    .send("Login failed detected via failed attempt indicator.".to_string())
                    .await;
                break;
            }

            if client.current_url().await.unwrap().as_str()
                != client_arc.current_url().await.unwrap().as_str()
            {
                // URL has changed, likely logged in successfully
                break;
            }

            wait_millis(1000).await;

            if failed_tx.is_closed() {
                break;
            }

            warn!("Polling for login failure...");
        }
        drop(failed_tx);
    });
}
