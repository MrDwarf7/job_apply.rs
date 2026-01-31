use fantoccini::Locator;

// use crate::portal::Portal;
use crate::prelude::Result;
use crate::states::Transition;

#[derive(Debug)]
pub enum SearchState {
    FindBy(FindBy),                    // Given a Locator, find the element
    FindAllBy(FindBy),                 // Find multiple elements by Locator
    FindJobListings(JobListingSearch), // Specialized job listing detection
}

#[async_trait::async_trait]
impl Transition for SearchState {
    async fn execute(&self) -> Result<()> {
        match self {
            SearchState::FindBy(_find_by) => {
                // Implement logic to find a single element by locator
                Ok(())
            }
            SearchState::FindAllBy(_find_by) => {
                // Implement logic to find multiple elements by locator
                Ok(())
            }
            SearchState::FindJobListings(_search) => {
                // 1. Use portal.selectors().job_listing to find all job cards
                // 2. Store element references for iteration
                // 3. Return count of found listings (0 = end of list)
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

#[derive(Debug)]
pub struct JobListingSearch {
    // pub portal:        Portal,
    pub current_index: usize,
    pub total_found:   usize,
}
