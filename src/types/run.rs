use crate::networking::Networking;
use crate::types::common::{Identifiable, ToolCalls, Tools, Usage};
use crate::types::error::OpenApiError;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Run {
    id: String,
    object: String,
    created_at: i64,
    thread_id: String,
    assistant_id: String,
    status: RunStatus,
    required_action: Option<RequiredAction>,
    last_error: Option<LastError>,
    expires_at: Option<i64>,
    started_at: Option<i64>,
    cancelled_at: Option<i64>,
    failed_at: Option<i64>,
    completed_at: Option<i64>,
    model: String,
    instructions: String,
    tools: Vec<Tools>,
    file_ids: Vec<String>,
    metadata: HashMap<String, String>,
    usage: Option<Usage>,
}

impl Run {
    pub fn retrieve_self(mut self, networking: &Networking) -> Result<Run, OpenApiError> {
        // TODO: Rethink name?
        // TODO: Is passing networking this way the best idea?
        networking.retrieve_run(self.thread_id.clone(), self.id.clone())
    }

    pub fn is_complete(&self) -> bool {
        match self.status {
            RunStatus::Completed => true,
            _ => false,
        }
    }
}

impl Identifiable for Run {
    fn get_identifier(&self) -> String {
        self.id.clone().to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunBuilder {
    #[serde(skip)]
    thread_id: String,
    assistant_id: String,
    model: Option<String>,
    instructions: Option<String>,
    additional_instructions: Option<String>,
    tools: Vec<()>, //TODO
    metadata: HashMap<String, String>,
}

impl RunBuilder {
    pub fn new(thread_id: String, assistant_id: String) -> Self {
        Self {
            thread_id,
            assistant_id,
            model: None,
            instructions: None,
            additional_instructions: None,
            tools: Vec::new(), // TODO
            metadata: HashMap::new(),
        }
    }

    pub fn get_thread_id(&self) -> String {
        self.thread_id.clone().to_string()
    }

    pub fn build(&self, networking: &Networking) -> Result<Run, OpenApiError> {
        networking.create_run(self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunStep {
    id: String,
    object: String,
    created_at: i64,
    assistant_id: String,
    thread_id: String,
    run_id: String,
    r#type: String, //
    status: RunStatus,
    step_details: (), //
    last_error: Option<LastError>,
    expired_at: Option<i64>,
    cancelled_at: Option<i64>,
    failed_at: Option<i64>,
    completed_at: Option<i64>,
    metadata: HashMap<String, String>,
    usage: Option<Usage>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RunType {
    MessageCreation,
    ToolCalls,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Queued,
    InProgress,
    RequiresAction,
    Cancelling,
    Cancelled,
    Failed,
    Completed,
    Expired,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LastError {
    code: LastErrorCode,
    message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LastErrorCode {
    ServerError,
    RateLimitExceeded,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequiredAction {
    r#type: RequiredActionType,
    submit_tool_outputs: SubmitToolOutputs,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubmitToolOutputs {
    tool_calls: ToolCalls,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RequiredActionType {
    SubmitToolOutput,
}