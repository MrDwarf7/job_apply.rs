use std::sync::Arc;

use fantoccini::{Client, Locator};

use crate::pre_fsm::wait_millis;
pub use crate::prelude::*;
use crate::providers::{Provider, ProviderKind, SelectorKind};
use crate::states::ActionState;

pub async fn job_board(
    client: &Client,
    provider: Arc<dyn Provider + Send + Sync>,
    selector_kind: SelectorKind, // Css, Xpath, etc.
) -> Result<()> {
    info!("Navigating to job board page for provider: {:?}", provider.name());
    let selectors = provider.get_job_search_selectors(selector_kind);

    wait_millis(1500).await;

    let jobs_element = provider
        .with_element(client, selector_kind, selectors.jobs_button)
        .await;

    info!("Found jobs button element, proceeding to click it.");
    let jobs_element = match jobs_element {
        Ok(elem) => elem,
        Err(e) => return Err(Error::Generic(format!("Failed to find jobs button element: {}", e))),
    };

    info!("Clicking jobs button element to navigate to job board.");
    let jobs_button_action = ActionState::Click {
        element: jobs_element,
    };

    info!("Performing click action on jobs button.");
    provider.with_action(jobs_button_action).await?;

    wait_millis(200).await;

    info!("Verifying navigation to job board page.");
    match provider.name() {
        ProviderKind::LinkedIn => {
            // LinkedIn has a different flow for job board navigation
            info!("Using LinkedIn-specific job board navigation.");
            let jobs_url_linkedin = "https://www.linkedin.com/jobs/";
            if client
                .current_url()
                .await
                .map_err(|e| Error::Generic(format!("Failed to get current URL: {}", e)))?
                .as_str()
                == jobs_url_linkedin
            {
                info!("Already on LinkedIn job board page.");
                return Ok(());
            }

            warn!(
                "LinkedIn job board navigation via button click may not have worked as expected. \
                 Navigating directly to job board URL."
            );

            client.goto(jobs_url_linkedin).await.map_err(|e| {
                Error::Generic(format!(
                    "Failed to navigate to job board URL '{}': {}",
                    jobs_url_linkedin, e
                ))
            })?;
        }
        ProviderKind::Seek => {
            unimplemented!("Seek provider INITIAL job board navigation not implemented yet.")
        }
    }

    info!("Navigated to job board page successfully.");

    Ok(())
}
