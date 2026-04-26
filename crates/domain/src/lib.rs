use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Internal Error: {0}")]
    InternalError(String),

    #[error("Serialization Error: {0}")]
    SerializationError(String),

    #[error("Orchestration Error: {0}")]
    OrchestrationError(String),

    #[error("Authentication Failed: {0}")]
    AuthError(String),

    #[error("Agent Sandboxing Error: {0}")]
    SandboxError(String),
}

pub type Result<T> = std::result::Result<T, CoreError>;
