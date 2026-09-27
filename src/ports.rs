use async_trait::async_trait;
use thiserror::Error;

use crate::{ExternalRequest, ExternalResult, Ticket, WorkerAction, WorkerId};

#[derive(Debug, Clone)]
pub struct ReasoningRequest {
    pub worker_id: WorkerId,
    pub ticket: Ticket,
    pub recent_observations: Vec<String>,
}

#[derive(Debug, Error)]
pub enum ReasoningError {
    #[error("invalid action: {0}")]
    InvalidAction(String),
    #[error("reasoner unavailable")]
    Unavailable,
}

#[async_trait]
pub trait Reasoner: Send + Sync {
    async fn next_action(&self, request: ReasoningRequest) -> Result<WorkerAction, ReasoningError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ExternalError {
    #[error("external operation denied")]
    Denied,
    #[error("external operation unavailable")]
    Unavailable,
    #[error("external operation ambiguous")]
    Ambiguous,
    #[error("external operation invalid")]
    Invalid,
}

impl ExternalError {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Denied => "EXTERNAL_DENIED",
            Self::Unavailable => "EXTERNAL_UNAVAILABLE",
            Self::Ambiguous => "EXTERNAL_AMBIGUOUS",
            Self::Invalid => "EXTERNAL_INVALID",
        }
    }
}

#[async_trait]
pub trait ExternalRuntime: Send + Sync {
    async fn invoke(&self, request: ExternalRequest) -> Result<ExternalResult, ExternalError>;
}
