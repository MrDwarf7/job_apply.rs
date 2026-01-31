// use crate::portal::Portal;
use crate::prelude::Result;
use crate::states::Transition;

#[derive(Debug, Clone)]
pub enum ValidateState {
    CheckJob(JobValidation),
}

#[derive(Debug, Clone)]
pub struct JobValidation {
    // pub portal: Portal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationResult {
    ValidJob,
    InvalidJob(InvalidReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvalidReason {
    AlreadyApplied,
    NoEasyApply,
    JobClosed,
}

#[async_trait::async_trait]
impl Transition for ValidateState {
    async fn execute(&self) -> Result<()> {
        match self {
            ValidateState::CheckJob(_validation) => {
                // 1. Check for Easy Apply button (portal.selectors().easy_apply_button)
                // 2. Check for "Applied" badge indicating already applied
                // 3. Check for job closed/expired indicators
                // 4. Return ValidationResult::ValidJob or InvalidJob with reason
                Ok(())
            }
        }
    }

    async fn current_state(&self) -> &dyn Transition {
        self
    }
}
