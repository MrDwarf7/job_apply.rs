pub use linkedin::LINKEDIN_JOB_LISTING_SELECTORS;
pub use seek::SEEK_JOB_LISTING_SELECTORS;

#[derive(Debug, Clone)]
pub struct JobListingSelectors {
    pub job_card:             &'static str,
    pub easy_apply_button:    &'static str,
    pub applied_badge:        &'static str,
    pub job_closed_indicator: &'static str,
    // ... more fields as/if needed
}

pub struct JogListingSelectorAgg {
    pub css:   JobListingSelectors,
    pub xpath: JobListingSelectors,
}

mod linkedin {
    use crate::selectors::{JobListingSelectors, JogListingSelectorAgg};

    const LINKEDIN_JOB_LISTING_XPATH_SELECTORS: JobListingSelectors = JobListingSelectors {
        job_card:             "//div[contains(@class, 'job-card-container')]",
        easy_apply_button:    "//button[contains(@class, 'easy-apply-button')]",
        applied_badge:        "//span[contains(@class, 'applied-badge')]",
        job_closed_indicator: "//span[contains(@class, 'job-closed-indicator')]",
    };

    const LINKEDIN_JOB_LISTING_CSS_SELECTORS: JobListingSelectors = JobListingSelectors {
        job_card:             "div.job-card-container",
        easy_apply_button:    "button.easy-apply-button",
        applied_badge:        "span.applied-badge",
        job_closed_indicator: "span.job-closed-indicator",
    };

    pub const LINKEDIN_JOB_LISTING_SELECTORS: JogListingSelectorAgg = JogListingSelectorAgg {
        css:   LINKEDIN_JOB_LISTING_CSS_SELECTORS,
        xpath: LINKEDIN_JOB_LISTING_XPATH_SELECTORS,
    };
}

mod seek {
    use crate::selectors::{JobListingSelectors, JogListingSelectorAgg};

    const SEEK_JOB_LISTING_XPATH_SELECTORS: JobListingSelectors = JobListingSelectors {
        job_card:             "//div[contains(@class, 'job-card-container')]",
        easy_apply_button:    "//button[contains(@class, 'easy-apply-button')]",
        applied_badge:        "//span[contains(@class, 'applied-badge')]",
        job_closed_indicator: "//span[contains(@class, 'job-closed-indicator')]",
    };

    const SEEK_JOB_LISTING_CSS_SELECTORS: JobListingSelectors = JobListingSelectors {
        job_card:             "div.job-card-container",
        easy_apply_button:    "button.easy-apply-button",
        applied_badge:        "span.applied-badge",
        job_closed_indicator: "span.job-closed-indicator",
    };

    pub const SEEK_JOB_LISTING_SELECTORS: JogListingSelectorAgg = JogListingSelectorAgg {
        css:   SEEK_JOB_LISTING_CSS_SELECTORS,
        xpath: SEEK_JOB_LISTING_XPATH_SELECTORS,
    };
}
