use crate::prelude::*;
use crate::states::Transition;

#[derive(Debug, Clone)]
pub struct PausedState {
    pub reason:        String,
    pub duration_secs: u64,
    pub can_resume:    bool,
}

#[async_trait::async_trait]
impl Transition for PausedState {
    async fn execute(&self) -> Result<()> {
        warn!("Paused for {} seconds due to reason: {}", self.duration_secs, self.reason);
        // Implement logic to handle the paused state
        // For example, log the reason and wait for the specified duration
        Ok(())
    }

    async fn current_state(&self) -> &dyn Transition {
        self
    }
}
