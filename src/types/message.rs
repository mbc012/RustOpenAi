use crate::networking::Networking;
use crate::types::common::Identifiable;
use crate::types::error::OpenApiError;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    id: String,
    object: String,
    created_at: i64,
    thread_id: String,
    role: MessageRole,
    content: Vec<MessageContent>,
    assistant_id: Option<String>,
    run_id: Option<String>,
    file_ids: Vec<String>, // TODO: Check type, could be File
    metadata: HashMap<String, String>,
}

impl Identifiable for Message {
    fn get_identifier(&self) -> String {
        self.id.clone().to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageBuilder {
    #[serde(skip)]
    thread_id: String,
    role: Option<MessageRole>, // TODO: below
    content: Option<String>,   // TODO: Check if optional is best way
    #[serde(skip_serializing_if = "Option::is_none")]
    file_ids: Option<Vec<String>>,
    metadata: Option<HashMap<String, String>>,
}
impl MessageBuilder {
    pub fn new(thread_id: String) -> Self {
        Self {
            thread_id,
            role: None,
            content: None,
            file_ids: None,
            metadata: None,
        }
    }

    pub fn new_custom(
        thread_id: String,
        role: MessageRole,
        content: String,
        file_ids: Option<Vec<String>>,
        metadata: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            thread_id,
            role: Some(role),
            content: Some(content),
            file_ids,
            metadata,
        }
    }

    pub fn with_role(mut self, role: MessageRole) -> Self {
        self.role = Some(role);
        self
    }

    pub fn with_content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn with_file_ids(mut self, file_ids: Vec<String>) -> Self {
        self.file_ids = Some(file_ids);
        self
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn get_thread_id(&self) -> String {
        self.thread_id.clone().to_string()
    }

    pub fn build(&self, networking: &Networking) -> Result<Message, OpenApiError> {
        if (self.role == None) && (self.content == None) {
            return Err(OpenApiError::new(
                "Role and content must be set".to_string(),
            ));
        }
        networking.create_message(self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageFile {
    id: String,
    object: String,
    created_at: i64,
    message_id: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum MessageContent {
    Text(TextContent),
    ImageFile(ImageContent),
}

#[derive(Serialize, Deserialize, Debug)]
struct TextContent {
    r#type: String,
    text: TextValue,
}

#[derive(Serialize, Deserialize, Debug)]
struct TextValue {
    value: String,
    annotations: Vec<Annotations>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ImageContent {
    r#type: String,
    image_file: ImageLocation,
}

#[derive(Serialize, Deserialize, Debug)]
struct ImageLocation {
    file_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Annotations {
    FileCitation(FileCitation),
    FilePath(FilePath),
}

#[derive(Serialize, Deserialize, Debug)]
struct FileCitation {
    r#type: String,
    text: String,
    file_citation: FileCitationLocation,
    start_index: u32,
    end_index: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileCitationLocation {
    file_id: String,
    quote: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FilePath {
    r#type: String,
    text: String,
    file_path: FilePathLocation,
    start_index: u32,
    end_index: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct FilePathLocation {
    file_id: String,
}
