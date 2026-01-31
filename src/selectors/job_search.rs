pub use linkedin::LINKEDIN_JOB_SEARCH_SELECTORS;
pub use seek::SEEK_JOB_SEARCH_SELECTORS;

// *[@id="workspace"]/div/div/div[2]/div/div[1]/div/div[3]/a
// <a class="_4ed4eeb2 e91cd72d _1016a07e _0ed3419e _838ce6e7 _00f2bb2c _126ddb7a _4cb3e259 _8c20a94c _5fef3314 bca03e0a a3a4b6c0 _58a68957 d72a15d2 _1c90243c"
//  aria-disabled="false"
//  href="https://www.linkedin.com/jobs/collections/recommended/?discover=recommended&amp;discoveryOrigin=JOBS_HOME_JYMBII"
//  componentkey="bc94f4c0-e664-4fcb-80ff-d33cbd6f7afb"
//  data-view-name="jobs-home-action-top-jymbii-see-more-jobs"
//  >
//
// <span class="bd33d230 _52151404 _67ad13ed a3a4b6c0 _1016a07e _0ed3419e bca03e0a b6fe433d _06d78668 _00f2bb2c _126ddb7a _7bc17ceb _6bcf30d5 _67a9f241 _405be928 f6bd58c7 _4de1be5d _9d2f4343 _86c3c9a1 _1128dc89 _3ece5488 _66222516">
// <svg xmlns="http://www.w3.org/2000/svg" id="arrow-right-small" fill="currentColor" aria-hidden="true" data-rtl="true" data-supported-dps="16x16" viewBox="0 0 16 16" data-token-id="104" width="16" height="16" class="e4da95eb _184848aa _57781cd8 _2d8a5e42 _4532b1df ba1dd8e8" role="img" aria-label="">
// <path d="M11.45 3 15 8l-3.55 5H9l2.84-4H2V7h9.84L9 3z">
// </path>
// </svg>
// <span class="_3a77e68c _6ae86251 d7127baf _8011d5ef _872be80d _1682eb76 _61190c53 _761606f8 ba34bb39 _7dcb09d6 _2cf8aff2 _1816733c _838ce6e7 _150cba7a _679b673a">
// Show all</span>
// </span>
// </a>

#[derive(Debug, Clone)]
pub struct JobSearchSelectors {
    pub jobs_button:        &'static str,
    pub search_input:       &'static str,
    pub location_input:     &'static str,
    pub search_button:      &'static str,
    pub filter_button:      &'static str,
    pub remote_filter:      &'static str,
    pub date_posted_filter: &'static str,
    // ... more fields as/if needed
}

pub struct JobSearchSelectorAgg {
    pub css:   JobSearchSelectors,
    pub xpath: JobSearchSelectors,
}

mod linkedin {
    use crate::selectors::job_search::{JobSearchSelectorAgg, JobSearchSelectors};

    const LINKEDIN_JOB_SEARCH_XPATH_SELECTORS: JobSearchSelectors = JobSearchSelectors {
        // <a class="_3f219769 _6d3479b6 a3a4b6c0 _1016a07e _0ed3419e _6d9316ce _6fea649d abfb09f7 _06d78668 _00f2bb2c _8ecc8ecb f01d568d _65e46c10 fdb4c3c7 _58a68957 c383c9f7 _126ddb7a _4fa48274" href="https://www.linkedin.com/jobs/" data-view-name="navigation-jobs" aria-label="Jobs, 0 new notifications"><span class="_294f22da _58a68957"><svg xmlns="http://www.w3.org/2000/svg" id="job-medium" fill="currentColor" data-supported-dps="24x24" viewBox="0 0 24 24" data-token-id="183" width="24" height="24" class="e4da95eb _184848aa _79489f42 _35a5673e" role="img" aria-label="" aria-hidden="true" style="width: 24px; min-width: 24px; height: 24px; min-height: 24px;"><path d="M17 6V5a3 3 0 0 0-3-3h-4a3 3 0 0 0-3 3v1H2v4a3 3 0 0 0 3 3h14a3 3 0 0 0 3-3V6zM9 5a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v1H9zm10 9a4 4 0 0 0 3-1.38V17a3 3 0 0 1-3 3H5a3 3 0 0 1-3-3v-4.38A4 4 0 0 0 5 14z"></path></svg></span><span class="_6d9316ce _6fea649d"><span class="_59445045 _8518075b _3390a91a eacbff74 a5b43d99 _330a58e2 _48ac6412 _838ce6e7 _7bc17ceb _6bcf30d5 _6d9316ce _06d78668 _4532b1df">Jobs</span></span></a>
        jobs_button: "//a[@data-view-name='navigation-jobs']",

        // <input
        // class="a11e0090 bc3d8983 _22cc9796 _77bf8873 _9df4e0d3 _9566f9ea a2778aeb _96c5067d d72a15d2 _998ef67f _2e0af555 _1c19bb15 f174b912 dfde759c _12209c27 _36e07d1b _4fc770fd d1569c3c _3b55c1a6 f66d22d2 _4253bb7d _819b1156 b557317c _91ff6da3 _0726064e _43c3c72e _3486c43d df567e4e _5d6bc843 _2c542d84 fdb4c3c7"
        // id=":r1:"
        // data-testid="typeahead-input"
        // autocomplete="off"
        // dir="auto"
        // aria-autocomplete="list"
        // placeholder="Describe the job you want"
        // componentkey="jobSearchBox"
        // value=""
        // style="border-color: var(--fe0a3c3e, var(--e4bbf257));"
        // >
        search_input:       "//input[@componentkey='jobSearchBox']",
        location_input:     "//input[@aria-label='Location']",
        search_button:      "//button[@aria-label='Search']",
        filter_button:      "//button[contains(@class, 'filter-button')]",
        remote_filter:      "//label[contains(., 'Remote')]//input[@type='checkbox']",
        date_posted_filter: "//label[contains(., 'Past 24 hours')]//input[@type='checkbox']",
    };

    const LINKEDIN_JOB_SEARCH_CSS_SELECTORS: JobSearchSelectors = JobSearchSelectors {
        jobs_button:        "a[data-view-name='navigation-jobs']",
        search_input:       "input[aria-label='Search jobs']",
        location_input:     "input[aria-label='Location']",
        search_button:      "button[aria-label='Search']",
        filter_button:      "button.filter-button",
        remote_filter:      "label:contains('Remote') input[type='checkbox']",
        date_posted_filter: "label:contains('Past 24 hours') input[type='checkbox']",
    };

    pub const LINKEDIN_JOB_SEARCH_SELECTORS: JobSearchSelectorAgg = JobSearchSelectorAgg {
        css:   LINKEDIN_JOB_SEARCH_CSS_SELECTORS,
        xpath: LINKEDIN_JOB_SEARCH_XPATH_SELECTORS,
    };
}

mod seek {
    use crate::selectors::job_search::{JobSearchSelectorAgg, JobSearchSelectors};

    const SEEK_JOB_SEARCH_XPATH_SELECTORS: JobSearchSelectors = JobSearchSelectors {
        jobs_button:        "//a[@aria-label='Jobs']",
        search_input:       "//input[@id='search-field']",
        location_input:     "//input[@id='location-field']",
        search_button:      "//button[@id='search-button']",
        filter_button:      "//button[contains(@class, 'filter-button')]",
        remote_filter:      "//label[contains(., 'Remote')]//input[@type='checkbox']",
        date_posted_filter: "//label[contains(., 'Past 24 hours')]//input[@type='checkbox']",
    };

    const SEEK_JOB_SEARCH_CSS_SELECTORS: JobSearchSelectors = JobSearchSelectors {
        jobs_button:        "a[aria-label='Jobs']",
        search_input:       "input#search-field",
        location_input:     "input#location-field",
        search_button:      "button#search-button",
        filter_button:      "button.filter-button",
        remote_filter:      "label:contains('Remote') input[type='checkbox']",
        date_posted_filter: "label:contains('Past 24 hours') input[type='checkbox']",
    };

    pub const SEEK_JOB_SEARCH_SELECTORS: JobSearchSelectorAgg = JobSearchSelectorAgg {
        css:   SEEK_JOB_SEARCH_CSS_SELECTORS,
        xpath: SEEK_JOB_SEARCH_XPATH_SELECTORS,
    };
}
