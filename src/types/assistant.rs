use crate::impl_ref;
use serde::{Deserialize, Serialize};
use serde_urlencoded;
use std::collections::HashMap;

use crate::networking::Networking;
use crate::types::common::{Identifiable, Tools};
use crate::types::error::OpenApiError;

/// `Assistant` is a struct that represents an OpenAI assistant object. It represents an entity
/// that can be configured to respond to users’ Messages using several parameters like:
/// - Instructions: how the Assistant and model should behave or respond
/// - Model: you can specify any GPT-3.5 or GPT-4 models. The Retrieval tool requires at least gpt-3.5-turbo-1106 (newer versions are supported) or gpt-4-turbo-preview models.
/// - Tools: the API supports Code Interpreter and Retrieval that are built and hosted by OpenAI.
/// - Functions: the API allows you to define custom function signatures, with similar behavior the function calling feature.
///
/// # Fields
///
/// * `id: String` - The unique identifier of the assistant.
/// * `object: String` - The type of the object.
/// * `created_at: u64` - The timestamp of when the assistant was created.
/// * `name: Option<String>` - The name of the assistant, if provided.
/// * `description: Option<String>` - The description of the assistant, if provided.
/// * `model: String` - The model identifier that the assistant is based on.
/// * `instructions: Option<String>` - Instructions for the assistant, if provided.
/// * `tools: Vec<Tools>` - A list of tools associated with the assistant.
/// * `file_ids: Vec<String>` - A list of file identifiers associated with the assistant.
/// * `metadata: HashMap<String, String>` - A map of metadata associated with the assistant.
///
/// # Implements
///
/// `Identifiable` trait for both owned and reference values
///
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
impl_ref!(Assistant, Identifiable);

/// `AssistantFile` is a struct that represents a file associated with an OpenAI assistant object.
///
/// # Fields
///
/// * `id: String` - The unique identifier of the assistant file.
/// * `object: String` - The type of the object.
/// * `created_at: u64` - The timestamp of when the assistant file was created.
/// * `assistant_id: String` - The identifier of the assistant that this file is associated with.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantFile {
    id: String,
    object: String,
    created_at: u64,
    assistant_id: String,
}

/// `AssistantBuilder` is a struct that provides a builder pattern for creating an `Assistant`.
///
/// # Fields
///
/// * `model: String` - The model identifier that the assistant is based on.
/// * `name: Option<String>` - The name of the assistant, if provided.
/// * `description: Option<String>` - The description of the assistant, if provided.
/// * `instructions: Option<String>` - Instructions for the assistant, if provided.
/// * `tools: Option<Vec<Tools>>` - A list of tools associated with the assistant.
/// * `file_ids: Option<Vec<String>>` - A list of file identifiers associated with the assistant.
/// * `metadata: Option<HashMap<String, String>>` - A map of metadata associated with the assistant.
///
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
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<HashMap<String, String>>,
}

impl AssistantBuilder {
    pub fn new<T: Identifiable>(model: T) -> Self {
        Self {
            model: model.get_identifier(),
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

    pub fn with_name<T: Into<String>>(mut self, name: T) -> Result<Self, OpenApiError> {
        let name = name.into();
        if name.len() > 256 {
            return Err(OpenApiError::InvalidLength(name.len(), 256));
        }
        self.name = Some(name);
        Ok(self)
    }

    pub fn with_description<T: Into<String>>(
        mut self,
        description: T,
    ) -> Result<Self, OpenApiError> {
        let description = description.into();
        if description.len() > 512 {
            return Err(OpenApiError::InvalidLength(description.len(), 512));
        }
        self.description = Some(description);
        Ok(self)
    }

    pub fn with_instructions<T: Into<String>>(
        mut self,
        instructions: T,
    ) -> Result<Self, OpenApiError> {
        let instructions = instructions.into();
        if instructions.len() > 32768 {
            return Err(OpenApiError::InvalidLength(instructions.len(), 32768));
        }
        self.instructions = Some(instructions);
        Ok(self)
    }

    pub fn with_tools(mut self, tools: Vec<Tools>) -> Self {
        // Add a list tools using a Vec<Tools>
        // TODO Add check for current tool count doesnt exceed 128
        self.tools = Some(tools);
        self
    }

    pub fn add_tool(mut self, tool: Tools) -> Self {
        // Add an individual tool using a Tools
        // TODO Add check for current tool count doesn't exceed 128
        if let Some(mut tools) = self.tools {
            tools.push(tool);
            self.tools = Some(tools);
        } else {
            self.tools = Some(vec![tool]);
        }
        self
    }

    pub fn with_file_ids(mut self, file_ids: Vec<String>) -> Self {
        // Add a list of file ids using a Vec<String>
        // TODO Add check for current file id count doesn't exceed 20
        self.file_ids = Some(file_ids);
        self
    }

    pub fn add_file_id<T: Identifiable>(mut self, file_id: T) -> Self {
        // Add an individual file id using an Identifiable
        // TODO Add check for current file id count doesn't exceed 20
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
        // TODO Revisit this, not sure if we need to check metadata type
        self.metadata = Some(metadata);
        self
    }

    pub fn build(&self, networking: &Networking) -> Result<Assistant, OpenApiError> {
        networking.create_assistant(self)
    }
}

/// `AssistantFileBuilder` is a struct that provides a builder pattern for creating an `AssistantFile`.
///
/// # Fields
///
/// * `assistant_id: String` - The identifier of the assistant that this file is associated with.
/// * `file_id: String` - The unique identifier of the assistant file.
///
#[derive(Debug, Serialize)]
pub struct AssistantFileBuilder {
    #[serde(skip_serializing)]
    assistant_id: String,
    file_id: String,
}

impl AssistantFileBuilder {
    pub fn new<T: Identifiable>(assistant_id: T, file_id: T) -> Self {
        Self {
            assistant_id: assistant_id.get_identifier(),
            file_id: file_id.get_identifier(),
        }
    }

    pub fn build(&self, networking: &Networking) -> Result<AssistantFile, OpenApiError> {
        networking.create_assistant_file(self, &self.assistant_id)
    }
}
