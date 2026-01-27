use std::fmt::Debug;

use crate::prelude::*;
use crate::states::Transition;

pub struct ErrorState {
    pub message: String,
    pub action:  Box<dyn Fn() + Send + Sync>,
}

#[async_trait::async_trait]
impl Transition for ErrorState {
    async fn execute(&self) -> Result<()> {
        error!("Error occurred: {}", self.message);
        (self.action)();
        // Implement additional error handling logic if needed
        Ok(())
    }

    async fn current_state(&self) -> &dyn Transition {
        self
    }
}

impl Debug for ErrorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ErrorState").field("message", &self.message).finish()
    }
}
