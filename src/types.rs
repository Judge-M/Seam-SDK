use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

pub type WorkerId = Uuid;
pub type TicketId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: TicketId,
    pub objective: String,
    #[serde(default)]
    pub context: Vec<ContextItem>,
    #[serde(default)]
    pub constraints: Vec<String>,
    pub deliverable: String,
    #[serde(default)]
    pub local_authority: LocalAuthority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextItem {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalAuthority {
    pub read_workspace: bool,
    pub write_workspace: bool,
    pub execute_local: bool,
}

impl Default for LocalAuthority {
    fn default() -> Self {
        Self {
            read_workspace: true,
            write_workspace: false,
            execute_local: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerStatus {
    Completed,
    Blocked,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerReport {
    pub worker_id: WorkerId,
    pub ticket_id: TicketId,
    pub status: WorkerStatus,
    pub summary: String,
    #[serde(default)]
    pub artifacts: Vec<ArtifactRef>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactRef {
    pub name: String,
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalRequest {
    pub worker_id: WorkerId,
    pub ticket_id: TicketId,
    pub intent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalResult {
    pub data: Value,
}
