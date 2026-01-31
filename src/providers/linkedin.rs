use fantoccini::{Client, Locator};

use crate::prelude::{Error, Result};
use crate::providers::{ByCss, ByXpath, Provider, SelectorKind};
use crate::selectors::{
    JobDescriptionSelectors,
    JobListingSelectors,
    LINKEDIN_JOB_DESCRIPTION_SELECTORS,
    LINKEDIN_JOB_LISTING_SELECTORS,
    LINKEDIN_LOGIN_SELECTORS,
    LoginSelectors,
};

#[derive(Debug, Clone)]
pub struct LinkedInProvider;

#[async_trait::async_trait]
impl ByXpath for LinkedInProvider {
    async fn by_xpath(client: &Client, selector: &str) -> Result<fantoccini::elements::Element> {
        let element = client
            .wait()
            .for_element(Locator::XPath(selector))
            .await
            .map_err(|e| {
                Error::Generic(format!("Element not found for XPath '{}': {}", selector, e))
            })?;
        Ok(element)
    }
}

#[async_trait::async_trait]
impl ByCss for LinkedInProvider {
    async fn by_css(client: &Client, selector: &str) -> Result<fantoccini::elements::Element> {
        let element = client
            .wait()
            .for_element(Locator::Css(selector))
            .await
            .map_err(|e| {
                Error::Generic(format!("Element not found for CSS selector '{}': {}", selector, e))
            })?;
        Ok(element)
    }
}

impl Provider for LinkedInProvider {
    fn name(&self) -> &'static str {
        "linkedin"
    }

    fn get_login_selectors(&self, kind: SelectorKind) -> LoginSelectors {
        match kind {
            SelectorKind::Xpath => LINKEDIN_LOGIN_SELECTORS.xpath,
            SelectorKind::Css => LINKEDIN_LOGIN_SELECTORS.css,
        }
    }

    fn get_job_listing_selectors(&self, kind: SelectorKind) -> JobListingSelectors {
        match kind {
            SelectorKind::Xpath => LINKEDIN_JOB_LISTING_SELECTORS.xpath,
            SelectorKind::Css => LINKEDIN_JOB_LISTING_SELECTORS.css,
        }
    }

    fn get_job_description_selectors(&self, kind: SelectorKind) -> JobDescriptionSelectors {
        match kind {
            SelectorKind::Xpath => LINKEDIN_JOB_DESCRIPTION_SELECTORS.xpath,
            SelectorKind::Css => LINKEDIN_JOB_DESCRIPTION_SELECTORS.css,
        }
    }
}

impl From<&str> for LinkedInProvider {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "linkedin" => LinkedInProvider,
            _ => {
                unimplemented!(
                    "You've called LinkedInProvider::from with an unsupported provider name."
                )
            }
        }
    }
}

impl From<String> for LinkedInProvider {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "linkedin" => LinkedInProvider,
            _ => {
                unimplemented!(
                    "You've called LinkedInProvider::from with an unsupported provider name."
                )
            }
        }
    }
}
