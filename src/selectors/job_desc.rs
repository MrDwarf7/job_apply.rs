pub use linkedin::LINKEDIN_JOB_DESCRIPTION_SELECTORS;
pub use seek::SEEK_JOB_DESCRIPTION_SELECTORS;

#[derive(Debug, Clone)]
pub struct JobDescriptionSelectors {
    pub job_title:       &'static str,
    pub company_name:    &'static str,
    pub location:        &'static str,
    pub job_description: &'static str,
    pub apply_button:    &'static str,
    pub save_button:     &'static str,
}

pub struct JobDescriptionSelectorAgg {
    pub css:   JobDescriptionSelectors,
    pub xpath: JobDescriptionSelectors,
}

mod linkedin {
    use crate::selectors::{JobDescriptionSelectorAgg, JobDescriptionSelectors};

    const LINKEDIN_JOB_DESCRIPTION_XPATH_SELECTORS: JobDescriptionSelectors =
        JobDescriptionSelectors {
            job_title:       "//h1[contains(@class, 'topcard__title')]",
            company_name:    "//a[contains(@class, 'topcard__org-name-link')]",
            location:        "//span[contains(@class, 'topcard__flavor--bullet')]",
            job_description: "//div[contains(@class, 'description__text')]",
            apply_button:    "//button[contains(@class, 'apply-button')]",
            save_button:     "//button[contains(@class, 'save-button')]",
        };

    const LINKEDIN_JOB_DESCRIPTION_CSS_SELECTORS: JobDescriptionSelectors =
        JobDescriptionSelectors {
            job_title:       "h1.topcard__title",
            company_name:    "a.topcard__org-name-link",
            location:        "span.topcard__flavor--bullet",
            job_description: "div.description__text",
            apply_button:    "button.apply-button",
            save_button:     "button.save-button",
        };

    pub const LINKEDIN_JOB_DESCRIPTION_SELECTORS: JobDescriptionSelectorAgg =
        JobDescriptionSelectorAgg {
            css:   LINKEDIN_JOB_DESCRIPTION_CSS_SELECTORS,
            xpath: LINKEDIN_JOB_DESCRIPTION_XPATH_SELECTORS,
        };
}

mod seek {
    use crate::selectors::{JobDescriptionSelectorAgg, JobDescriptionSelectors};

    const SEEK_JOB_DESCRIPTION_XPATH_SELECTORS: JobDescriptionSelectors = JobDescriptionSelectors {
        job_title:       "//h1[contains(@class, 'job-title')]",
        company_name:    "//a[contains(@class, 'company-name')]",
        location:        "//span[contains(@class, 'job-location')]",
        job_description: "//div[contains(@class, 'job-description')]",
        apply_button:    "//button[contains(@class, 'apply-now-button')]",
        save_button:     "//button[contains(@class, 'save-job-button')]",
    };

    const SEEK_JOB_DESCRIPTION_CSS_SELECTORS: JobDescriptionSelectors = JobDescriptionSelectors {
        job_title:       "h1.job-title",
        company_name:    "a.company-name",
        location:        "span.job-location",
        job_description: "div.job-description",
        apply_button:    "button.apply-now-button",
        save_button:     "button.save-job-button",
    };

    pub const SEEK_JOB_DESCRIPTION_SELECTORS: JobDescriptionSelectorAgg =
        JobDescriptionSelectorAgg {
            css:   SEEK_JOB_DESCRIPTION_CSS_SELECTORS,
            xpath: SEEK_JOB_DESCRIPTION_XPATH_SELECTORS,
        };
}
