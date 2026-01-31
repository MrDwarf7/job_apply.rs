use fantoccini::{Client, Locator};
pub use linkedin::LINKEDIN_LOGIN_SELECTORS;
pub use seek::SEEK_LOGIN_SELECTORS;

use crate::Error;
use crate::prelude::Result;
use crate::providers::ByXpath;

#[derive(Debug, Clone)]
pub struct LoginSelectors {
    pub failed_attempt_indicator: &'static str,
    pub username_field:           &'static str,
    pub password_field:           &'static str,
    pub submit_button:            &'static str,
    // ... more fields as/if needed
}

// TODO: [typestate] : We will need to implement typestate pattern
// here as currently when calling say, login fn
// we can pass a selector_kind as Css,
// and if the internal functions are setup to call by_xpath,
// they'll fail.

pub struct LoginSelectorAgg {
    pub css:   LoginSelectors,
    pub xpath: LoginSelectors,
}

mod linkedin {
    use crate::selectors::{LoginSelectorAgg, LoginSelectors};

    const LINKEDIN_LOGIN_XPATH_SELECTORS: LoginSelectors = LoginSelectors {
        failed_attempt_indicator: "//div[@id='error-for-username']",
        username_field:           "//input[@id='username']",
        password_field:           "//input[@id='password']",
        submit_button:            "//button[@type='submit']",
    };

    const LINKEDIN_LOGIN_CSS_SELECTORS: LoginSelectors = LoginSelectors {
        failed_attempt_indicator: "div#error-for-username",
        username_field:           "input#username",
        password_field:           "input#password",
        submit_button:            "button[type='submit']",
    };

    pub const LINKEDIN_LOGIN_SELECTORS: LoginSelectorAgg = LoginSelectorAgg {
        css:   LINKEDIN_LOGIN_CSS_SELECTORS,
        xpath: LINKEDIN_LOGIN_XPATH_SELECTORS,
    };
}

mod seek {
    use crate::selectors::{LoginSelectorAgg, LoginSelectors};

    const SEEK_LOGIN_XPATH_SELECTORS: LoginSelectors = LoginSelectors {
        failed_attempt_indicator: "//div[@id='error-for-username']", // TODO: no idea what seek uses
        username_field:           "//input[@id='username']",
        password_field:           "//input[@id='password']",
        submit_button:            "//button[@type='submit']",
    };

    const SEEK_LOGIN_CSS_SELECTORS: LoginSelectors = LoginSelectors {
        failed_attempt_indicator: "div#error-for-username", // TODO: no idea what seek uses
        username_field:           "input#username",
        password_field:           "input#password",
        submit_button:            "button[type='submit']",
    };

    pub const SEEK_LOGIN_SELECTORS: LoginSelectorAgg = LoginSelectorAgg {
        css:   SEEK_LOGIN_CSS_SELECTORS,
        xpath: SEEK_LOGIN_XPATH_SELECTORS,
    };
}
