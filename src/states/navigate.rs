// use crate::portal::Portal;
use crate::prelude::Result;
use crate::states::Transition;

#[derive(Debug)]
pub enum NavigateState {
    ToUrl { attempt: u8, url: String },
    ToElement(ValidElements),
    // ToJobSearch { portal: Portal, keywords: String },
}

#[async_trait::async_trait]
impl Transition for NavigateState {
    async fn execute(&self) -> Result<()> {
        match self {
            NavigateState::ToUrl { attempt: _, url: _ } => {
                // Implement logic to navigate to the specified URL
                Ok(())
            }
            NavigateState::ToElement(_element) => {
                // Implement logic to navigate to the specified element
                Ok(())
            } // NavigateState::ToJobSearch { portal: _, keywords: _ } => {
              // Navigate to portal's search URL, enter keywords, submit, wait for listings
              // 1. client.goto(portal.search_url())
              // 2. wait for search input element (portal.selectors().search_input)
              // 3. clear input, type keywords
              // 4. submit search (press Enter)
              // 5. wait for job listings element (portal.selectors().job_listing)
              //     Ok(())
              // }
        }
    }

    async fn current_state(&self) -> &dyn Transition {
        self
    }
}

#[derive(Debug)]
pub enum ValidElements {
    Login(LoginElements),
    Job(JobElements),
    Application(ApplicationElements),
}

#[derive(Debug)]
pub enum LoginElements {
    LoginUsername,
    LoginPassword,
}

#[derive(Debug)]
pub enum JobElements {
    SearchInput,
    Listings,
    Description(DescriptionElements),
}

#[derive(Debug)]
pub enum DescriptionElements {
    ApplyButton,
    Submit,
}

#[derive(Debug)]
pub enum ApplicationElements {
    BulletPoint,
    TextField,
    NumberField,
    TickBox,
    ResumeUpload,

    // This gets special treatment due to having a config option to toggle it
    FollowCompanyTickBox,
}
