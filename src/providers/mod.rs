use std::fmt::{Debug, Display};
use std::sync::Arc;

use crate::prelude::Result;
use crate::selectors::{
    JobDescriptionSelectors,
    JobListingSelectors,
    JobSearchSelectors,
    LoginSelectors,
};
use crate::states::ActionState;

mod linkedin;
mod seek;

use fantoccini::Client;
use fantoccini::elements::Element;
pub use linkedin::LinkedInProvider;
pub use seek::SeekProvider;

// #[async_trait::async_trait]
// pub trait ByXpath {
//     async fn by_xpath(client: &Client, selector: &str) -> Result<fantoccini::elements::Element>;
// }
//
// #[async_trait::async_trait]
// pub trait ByCss {
//     async fn by_css(client: &Client, selector: &str) -> Result<fantoccini::elements::Element>;
// }

#[async_trait::async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> ProviderKind;

    /// Allows for attempting different selector strategies in a preferred order.
    fn preferred_selector_order(&self) -> Vec<SelectorKind> {
        vec![SelectorKind::Xpath, SelectorKind::Css]
    }

    // We want to return a function based on the kind,
    // so it'll be the child type's trait function
    // fn selector_by_kind(&self, kind: SelectorKind);

    // TODO: [refactor] : will need to move these to a diff trait later,
    // then implement on each + bind Provider to the new trait
    //
    // We can also expand this set to use things like the goto url as well

    async fn with_element(
        &self,
        client: &Client,
        kind: SelectorKind,
        selector: &str,
    ) -> Result<Element> {
        let locator = match kind {
            SelectorKind::Css => fantoccini::Locator::Css(selector),
            SelectorKind::Xpath => fantoccini::Locator::XPath(selector),
        };
        client.wait().for_element(locator).await.map_err(|e| {
            crate::prelude::Error::Generic(format!(
                "Element not found for selector '{}': {}",
                selector, e
            ))
        })
    }

    async fn with_elements(
        &self,
        client: &Client,
        kind: SelectorKind,
        selector: &str,
    ) -> Result<Vec<Element>> {
        let locator = match kind {
            SelectorKind::Css => fantoccini::Locator::Css(selector),
            SelectorKind::Xpath => fantoccini::Locator::XPath(selector),
        };
        client.find_all(locator).await.map_err(|e| {
            crate::prelude::Error::Generic(format!(
                "Elements not found for selector '{}': {}",
                selector, e
            ))
        })
    }

    async fn with_action<'a>(&self, action: ActionState<'a>) -> Result<()> {
        match action {
            ActionState::Click { element } => {
                element.click().await.map_err(|e| {
                    crate::prelude::Error::Generic(format!("Failed to click element: {}", e))
                })
            }
            ActionState::InputText { element, input } => {
                element.send_keys(&input).await.map_err(|e| {
                    crate::prelude::Error::Generic(format!(
                        "Failed to send keys '{}': {}",
                        input, e
                    ))
                })
            }
            ActionState::InputKey { element, key } => {
                element.send_keys(&key).await.map_err(|e| {
                    crate::prelude::Error::Generic(format!("Failed to send key '{:?}': {}", key, e))
                })
            }
        }
    }

    // // We have a function for each 'set' of selectors (Each stage)

    fn get_login_selectors(&self, kind: SelectorKind) -> LoginSelectors;

    fn get_job_search_selectors(&self, kind: SelectorKind) -> JobSearchSelectors;

    fn get_job_listing_selectors(&self, kind: SelectorKind) -> JobListingSelectors;

    fn get_job_description_selectors(&self, kind: SelectorKind) -> JobDescriptionSelectors;

    // TODO: [impl] : implement the other selectors needed to complete the flow

    // fn get_application_selectors(&self) -> application::ApplicationSelectors;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectorKind {
    Css,
    Xpath,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderKind {
    LinkedIn,
    Seek,
    // ... others
}

impl Display for ProviderKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ProviderKind::LinkedIn => "LinkedIn",
            ProviderKind::Seek => "Seek",
            // ... others
        };
        write!(f, "{}", s)
    }
}

impl From<&str> for ProviderKind {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "linkedin" => ProviderKind::LinkedIn,
            "seek" => ProviderKind::Seek,
            _ => unimplemented!("Unsupported provider kind: {}", s),
        }
    }
}

impl From<String> for ProviderKind {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "linkedin" => ProviderKind::LinkedIn,
            "seek" => ProviderKind::Seek,
            _ => unimplemented!("Unsupported provider kind: {}", s),
        }
    }
}

// TODO: [trait] : convert to a trait type (so either "linkedin" or ProviderKind can be passed in)

#[allow(unreachable_patterns)]
pub fn get_provider(kind: ProviderKind) -> Option<Arc<dyn Provider + Send + Sync>> {
    match kind {
        ProviderKind::LinkedIn => Some(Arc::new(LinkedInProvider)),
        ProviderKind::Seek => Some(Arc::new(SeekProvider)),
        // ... others
        _ => None,
    }
}
