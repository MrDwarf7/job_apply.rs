#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error handler: {0}")]
    Generic(String),

    //
    #[error("Driver not found: {driver}")]
    DriverNotFound {
        driver: String,
        source: which::Error,
    },

    #[error("Failed to start driver: {driver}")]
    DriverStartError {
        driver: String,
        source: std::io::Error,
    },

    #[error("State machine error: {0}")]
    StateMachineError(String),

    #[error("Tokio task join error")]
    TokioJoinError(#[from] tokio::task::JoinError),

    #[error("Login failure: {0}")]
    LoginFailure(String),
}
