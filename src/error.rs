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

    #[error("No keywords set, but 'use_keywords' is true: {0}")]
    NoKeywordsSet(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),

    // fantoccini::error::CmdError
    // FantocciniCmdError(#[from] fantoccini::error::CmdError),
    #[error("Fantoccini command error: {error}")]
    FantocciniCmdError {
        error: Box<fantoccini::error::CmdError>,
    },
}

#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("Create error: {0}")]
    Create(String),

    #[error("Read error: {0}")]
    Read(String),

    #[error("Update error: {0}")]
    Update(String),

    #[error("Delete error: {0}")]
    Delete(String),
}
