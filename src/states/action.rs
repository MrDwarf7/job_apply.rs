use fantoccini::elements::Element;

use crate::prelude::Result;
use crate::states::Transition;

#[derive(Debug)]
pub enum ActionState<'a> {
    Click {
        element: Element,
    },
    InputText {
        element: &'a Element,
        input:   String,
    },
}

#[async_trait::async_trait]
impl<'a> Transition for ActionState<'a> {
    async fn execute(&self) -> Result<()> {
        match self {
            ActionState::Click { element } => {
                // Implement logic to click the element identified by locator
                Ok(())
            }
            ActionState::InputText { element, input } => {
                // Implement logic to input text into the element identified by locator
                Ok(())
            }
        }
    }

    async fn current_state(&self) -> &dyn Transition {
        self
    }
}
