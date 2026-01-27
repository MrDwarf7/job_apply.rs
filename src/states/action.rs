use crate::prelude::Result;
use crate::states::Transition;

#[derive(Debug)]
pub enum ActionState {
    Click(String),
    InputText { locator: String, text: String },
}

#[async_trait::async_trait]
impl Transition for ActionState {
    async fn execute(&self) -> Result<()> {
        match self {
            ActionState::Click(locator) => {
                // Implement logic to click the element identified by locator
                Ok(())
            }
            ActionState::InputText { locator, text } => {
                // Implement logic to input text into the element identified by locator
                Ok(())
            }
        }
    }

    async fn current_state(&self) -> &dyn Transition {
        self
    }
}
