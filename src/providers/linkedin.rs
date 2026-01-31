use fantoccini::{Client, Locator};

use crate::prelude::{Error, Result};
use crate::providers::{Provider, ProviderKind, SelectorKind};
use crate::selectors::{
    JobDescriptionSelectors,
    JobListingSelectors,
    LINKEDIN_JOB_DESCRIPTION_SELECTORS,
    LINKEDIN_JOB_LISTING_SELECTORS,
    LINKEDIN_JOB_SEARCH_SELECTORS,
    LINKEDIN_LOGIN_SELECTORS,
    LoginSelectors,
};

#[derive(Debug, Clone)]
pub struct LinkedInProvider;

impl Provider for LinkedInProvider {
    fn name(&self) -> ProviderKind {
        ProviderKind::LinkedIn
    }

    fn get_login_selectors(&self, kind: SelectorKind) -> LoginSelectors {
        match kind {
            SelectorKind::Xpath => LINKEDIN_LOGIN_SELECTORS.xpath,
            SelectorKind::Css => LINKEDIN_LOGIN_SELECTORS.css,
        }
    }

    fn get_job_search_selectors(&self, kind: SelectorKind) -> crate::selectors::JobSearchSelectors {
        match kind {
            SelectorKind::Xpath => LINKEDIN_JOB_SEARCH_SELECTORS.xpath,
            SelectorKind::Css => LINKEDIN_JOB_SEARCH_SELECTORS.css,
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
