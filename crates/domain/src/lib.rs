use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Internal Orchestration Error: {0}")]
    OrchestrationError(String),
    #[error("Authentication Failed: {0}")]
    AuthError(String),
    #[error("Agent Sandboxing Error")]
    SandboxError,
}

pub type Result<T> = std::result::Result<T, CoreError>;
