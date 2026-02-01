use std::sync::Arc;

use fantoccini::Client;

use crate::pre_fsm::wait_millis;
use crate::prelude::*;
use crate::providers::{Provider, SelectorKind};

// TODO: [same_fn_1] : See associated comment

pub async fn all_job_cards(
    client: &Client,
    provider: Arc<dyn Provider + Send + Sync>,
    selector_kind: SelectorKind, // Css, Xpath, etc.
) -> Result<()> {
    //

    let job_card_selectors = provider.get_job_listing_selectors(selector_kind);
    wait_millis(1500).await;

    let all_job_cards = provider
        .with_elements(client, selector_kind, job_card_selectors.job_card)
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

    Ok(())
}
