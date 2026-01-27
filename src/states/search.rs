use fantoccini::Locator;

use crate::prelude::Result;
use crate::states::Transition;

#[derive(Debug)]
pub enum SearchState {
    FindBy(FindBy),    // Given a Locator, find the element
    FindAllBy(FindBy), // Find multiple elements by Locator
}

#[async_trait::async_trait]
impl Transition for SearchState {
    async fn execute(&self) -> Result<()> {
        match self {
            SearchState::FindBy(find_by) => {
                // Implement logic to find a single element by locator
                Ok(())
            }
            SearchState::FindAllBy(find_by) => {
                // Implement logic to find multiple elements by locator
                Ok(())
            }
        }
    }

    async fn current_state(&self) -> &dyn Transition {
        self
    }
}

#[derive(Debug)]
pub struct FindBy {
    pub locator: Locator<'static>,
}
