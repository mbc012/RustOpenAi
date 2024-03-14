use crate::networking::Networking;
use crate::types::common::{Identifiable, ToolCalls, Tools, Usage};
use crate::types::error::OpenApiError;

use crate::impl_ref;
use crate::message::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;

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

    pub fn retrieve_first_message(&self, networking: &Networking) -> Result<Message, OpenApiError> {
        let messages = networking.list_messages(self.thread_id.clone()).unwrap();
        if messages.get_data_vec().len() < 1 {
            // TODO: Change to OperationalError when completed
            return Err(OpenApiError::ClientError("Run contains no messages".into()));
        }
        let message = messages.get_data_vec().first().unwrap();
        Ok(message.clone())
    }

    pub fn retrieve_status(&self) -> RunStatus {
        self.status.clone()
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
impl_ref!(Run, Identifiable);

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RunBuilder {
    #[serde(skip)]
    thread_id: Option<String>,
    assistant_id: String,
    model: Option<String>,
    instructions: Option<String>,
    additional_instructions: Option<String>,
    tools: Vec<()>, //TODO
    metadata: HashMap<String, String>,
}

impl RunBuilder {
    pub fn new<T: Identifiable, A: Identifiable>(thread_id: T, assistant_id: A) -> Self {
        let thread_id: String = thread_id.get_identifier();
        let assistant_id: String = assistant_id.get_identifier();
        Self {
            thread_id: Some(thread_id),
            assistant_id,
            ..Self::default()
        }
    }

    pub fn new_with_thread<A: Identifiable>(assistant_id: A) -> Self {
        let assistant_id: String = assistant_id.get_identifier();
        Self {
            assistant_id,
            ..Self::default()
        }
    }

    pub fn with_model<M: Identifiable>(mut self, model: M) -> Self {
        let model: String = model.get_identifier();
        self.model = Some(model);
        self
    }

    pub fn with_instructions<I: Into<String>>(mut self, instructions: I) -> Self {
        let instructions = instructions.into();
        self.instructions = Some(instructions);
        self
    }

    pub fn with_additional_instructions<AI: Into<String>>(
        mut self,
        additional_instructions: AI,
    ) -> Self {
        let additional_instructions = additional_instructions.into();
        self.additional_instructions = Some(additional_instructions);
        self
    }

    pub fn build(&self, networking: &Networking) -> Result<Run, OpenApiError> {
        networking.create_run(self, &self.thread_id)
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

/** ---- Run Tests ---- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_builder() {
        //let run = RunBuilder::new_with_thread("asst_SJVM5rueqSA5KWXbOsvR2EO5".into());
        todo!()
    }
}
