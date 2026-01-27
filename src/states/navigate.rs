use crate::prelude::Result;
use crate::states::Transition;

#[derive(Debug)]
pub enum NavigateState {
    ToUrl { attempt: u8, url: String },
    ToElement(ValidElements),
    // ToTab(ActionState), // etc...
}

#[async_trait::async_trait]
impl Transition for NavigateState {
    async fn execute(&self) -> Result<()> {
        match self {
            NavigateState::ToUrl { attempt, url } => {
                // Implement logic to navigate to the specified URL
                Ok(())
            }
            NavigateState::ToElement(element) => {
                // Implement logic to navigate to the specified element
                Ok(())
            }
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
