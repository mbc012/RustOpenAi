use serde::{Deserialize, Serialize};
use serde_urlencoded;
use std::collections::HashMap;

use crate::networking::Networking;
use crate::types::common::{Identifiable, Tools};
use crate::types::error::OpenApiError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Assistant {
    id: String,
    object: String,
    created_at: u64,
    name: Option<String>,
    description: Option<String>,
    model: String,
    instructions: Option<String>,
    tools: Vec<Tools>,
    file_ids: Vec<String>,
    metadata: HashMap<String, String>,
}

impl Identifiable for Assistant {
    fn get_identifier(&self) -> String {
        self.id.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantFile {
    id: String,
    object: String,
    created_at: u64,
    assistant_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantFileBuilder {
    #[serde(skip)]
    networking: Networking,
    #[serde(skip)]
    assistant_id: String,
    file_id: String,
}

impl AssistantFileBuilder {
    pub fn new(networking: Networking, assistant_id: String, file_id: String) -> Self {
        Self {
            networking,
            assistant_id,
            file_id,
        }
    }

    pub fn get_assistant_id(&self) -> String {
        self.assistant_id.clone()
    }

    pub fn get_file_id(&self) -> String {
        self.file_id.clone()
    }

    pub fn build(&self) -> Result<AssistantFile, OpenApiError> {
        self.networking.create_assistant_file(self)
    }
}

#[derive(Default, Debug, Serialize)]
pub struct AssistantBuilder {
    model: String,
    name: Option<String>,
    description: Option<String>,
    instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tools>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_ids: Option<Vec<String>>,
    metadata: Option<HashMap<String, String>>,
}

impl AssistantBuilder {
    pub fn new<T: Identifiable>(model: T) -> Self {
        let model_id: String = model.get_identifier();
        Self {
            model: model_id,
            ..AssistantBuilder::default()
        }
    }

    pub fn from(assistant: Assistant) -> Self {
        Self {
            model: assistant.model,
            name: assistant.name,
            description: assistant.description,
            instructions: assistant.instructions,
            tools: Some(assistant.tools),
            file_ids: Some(assistant.file_ids),
            metadata: Some(assistant.metadata),
        }
    }

    pub fn with_name<T: Identifiable>(mut self, name: T) -> Self {
        self.name = Some(name.get_identifier());
        self
    }

    pub fn with_description<T: Identifiable>(mut self, description: T) -> Self {
        self.description = Some(description.get_identifier());
        self
    }

    pub fn with_instructions<T: Identifiable>(mut self, instructions: T) -> Self {
        self.instructions = Some(instructions.get_identifier());
        self
    }

    pub fn with_tools(mut self, tools: Vec<Tools>) -> Self {
        self.tools = Some(tools);
        self
    }

    pub fn add_tool(mut self, tool: Tools) -> Self {
        if let Some(mut tools) = self.tools {
            tools.push(tool);
            self.tools = Some(tools);
        } else {
            self.tools = Some(vec![tool]);
        }
        self
    }

    pub fn with_file_ids(mut self, file_ids: Vec<String>) -> Self {
        self.file_ids = Some(file_ids);
        self
    }

    pub fn add_file_id<T: Identifiable>(mut self, file_id: T) -> Self {
        let file_id = file_id.get_identifier();
        if let Some(mut file_ids) = self.file_ids {
            file_ids.push(file_id);
            self.file_ids = Some(file_ids);
        } else {
            self.file_ids = Some(vec![file_id]);
        }
        self
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(&self, networking: &Networking) -> Result<Assistant, OpenApiError> {
        networking.create_assistant(self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListAssistantParams {
    limit: Option<u32>,
    order: Option<String>,
    after: Option<String>,
    before: Option<String>,
}

impl ListAssistantParams {
    pub fn new(
        limit: Option<u32>,
        order: Option<String>,
        after: Option<String>,
        before: Option<String>,
    ) -> Self {
        Self {
            limit,
            order,
            after,
            before,
        }
    }

    pub fn to_query_params(&self) -> String {
        let params = serde_urlencoded::to_string(self).unwrap();
        params
    }
}
