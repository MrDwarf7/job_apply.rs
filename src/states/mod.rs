mod action;
mod error_state;
mod navigate;
mod paused;
mod search;
mod validate;

use std::sync::Arc;

use fantoccini::Locator;

use crate::prelude::*;
pub use crate::states::action::ActionState;
use crate::states::error_state::ErrorState;
use crate::states::navigate::NavigateState;
use crate::states::paused::PausedState;
use crate::states::search::{FindBy, JobListingSearch, SearchState};
use crate::states::validate::{InvalidReason, JobValidation, ValidateState, ValidationResult};

#[async_trait::async_trait]
pub trait Transition {
    async fn execute(&self) -> Result<()>;

    async fn current_state(&self) -> &dyn Transition;
}

#[derive(Debug, Clone)]
pub enum States<'a> {
    /// Searching for a specific T ( element, text, etc.)
    Search(SearchState),
    //
    /// Navigating to a specific T ( url, text, etc.)
    Navigate(NavigateState),

    /// Taking an action (click, input text, etc.)
    Action(ActionState<'a>),

    /// Validating a job (Easy Apply available, not already applied, etc.)
    Validate(ValidateState),

    /// The maximum number of iterations has been reached
    MaxIterationsReached,

    /// Requires user intervention to continue
    Paused(PausedState),

    /// Something has gone catastrophically wrong
    /// from which we cannot simply ask the user to resume
    Error, // This originall had (ErrorState) but it's not Clone so
}

#[async_trait::async_trait]
impl Transition for States<'_> {
    async fn execute(&self) -> Result<()> {
        self.execute().await
    }

    async fn current_state(&self) -> &dyn Transition {
        self.current_state().await
    }
}

impl States<'_> {
    async fn execute(&self) -> Result<()> {
        let fut = Box::pin(async move {
            match self {
                States::Search(state) => state.execute().await,
                States::Navigate(state) => state.execute().await,
                States::Action(state) => state.execute().await,
                States::Validate(state) => state.execute().await,
                States::MaxIterationsReached => {
                    todo!()
                }
                States::Paused(state) => state.execute().await,
                States::Error => {
                    todo!()
                    // let error_state = ErrorState {
                    //     message: "An unrecoverable error has occurred.".to_string(),
                    //     action:  Box::new(|| {
                    //         error!("Executing error handling action.");
                    //     }),
                    // };
                    // error_state.execute().await
                }
            }
        });

        return fut.await;
    }

    async fn current_state(&self) -> &dyn Transition {
        Box::pin(async move {
            (match self {
                States::Search(state) => state.current_state(),
                States::Navigate(state) => state.current_state(),
                States::Action(state) => state.current_state(),
                States::Validate(state) => state.current_state(),
                States::MaxIterationsReached => {
                    todo!()
                }
                States::Paused(state) => state.current_state(),
                States::Error => {
                    todo!()
                }
            })
            .await
        })
        .await
    }
}

// This funciton signature is open to changes as we work through early development states
//
pub async fn start_state_machine<S>(
    starting_url: &S,
    max_iterations: usize, // max_attempts: N, // later we will impl .some form of exponential backoff here
) -> Result<()>
where
    S: AsRef<str> + Send + Sync + Clone + 'static,
{
    let starting_state = States::Navigate(NavigateState::ToUrl {
        attempt: 0,
        url:     starting_url.as_ref().to_string(),
    });

    starting_state.execute().await?;

    for idx in 0..max_iterations {
        let starting_state = starting_state.clone();

        let handle = tokio::spawn(async move {
            let _ = starting_state.execute().await;
        })
        .await;

        match handle {
            Ok(_) => (),
            Err(e) => {
                // TODO: [state] : We need a way to programmatically update our
                // state to say we found an error, so on the following loop iteration
                // we can handle it

                error!("Task join error: {}", e);
                // return Err(Error::from(e)); // for now we can crash/fail
            }
        }

        if idx + 1 >= max_iterations {
            warn!("Maximum iterations ({}) reached, stopping state machine.", max_iterations);
            break;
        }

        // tokio::try_join!(handle)?; // Wait for the state machine to complete
    }

    Ok(())
}
