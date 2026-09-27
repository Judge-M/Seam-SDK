use std::sync::Arc;

use async_trait::async_trait;
use seam_sdk::{
    ExternalError, ExternalRequest, ExternalResult, ExternalRuntime, FsProcessWorkbench, Reasoner,
    ReasoningError, ReasoningRequest, Ticket, WorkerAction, WorkerRuntime,
};
use serde_json::json;
use uuid::Uuid;

struct FixtureReasoner;

#[async_trait]
impl Reasoner for FixtureReasoner {
    async fn next_action(&self, request: ReasoningRequest) -> Result<WorkerAction, ReasoningError> {
        if request.recent_observations.is_empty() {
            Ok(WorkerAction::External {
                intent: "Inspect the fixture issue".into(),
            })
        } else {
            Ok(WorkerAction::Finish {
                summary: "Fixture issue inspected".into(),
                artifacts: vec![],
            })
        }
    }
}

struct FixtureExternal;

#[async_trait]
impl ExternalRuntime for FixtureExternal {
    async fn invoke(&self, request: ExternalRequest) -> Result<ExternalResult, ExternalError> {
        assert_eq!(request.intent, "Inspect the fixture issue");
        Ok(ExternalResult {
            data: json!({"issue": "fixture"}),
        })
    }
}

#[tokio::main]
async fn main() {
    let runtime = WorkerRuntime::new(
        Arc::new(FixtureReasoner),
        Arc::new(FixtureExternal),
        Arc::new(FsProcessWorkbench::new(".", false, false)),
    );
    let report = runtime
        .run(
            Uuid::new_v4(),
            Ticket {
                id: Uuid::new_v4(),
                objective: "Inspect one issue".into(),
                context: vec![],
                constraints: vec![],
                deliverable: "summary".into(),
                local_authority: Default::default(),
            },
        )
        .await
        .expect("worker runtime");
    println!("{}", report.summary);
}
