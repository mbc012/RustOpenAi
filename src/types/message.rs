use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

use crate::networking::Networking;
use crate::types::common::{Identifiable, ToolCalls};
use crate::types::error::OpenApiError;

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
    file_ids: Vec<String>,
    metadata: HashMap<String, String>,
}

impl Identifiable for Message {
    fn get_identifier(&self) -> String {
        self.id.clone()
    }
}

impl<'a> Identifiable for &'a Message {
    fn get_identifier(&self) -> String {
        self.id.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageBuilder {
    #[serde(skip)]
    thread_id: String,
    role: MessageRole,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_ids: Option<Vec<String>>,
    metadata: Option<HashMap<String, String>>,
}
impl MessageBuilder {
    pub fn new<I: Identifiable>(
        thread_id: I,
        role: MessageRole,
        content: String,
    ) -> Result<Self, OpenApiError> {
        let thread_id = thread_id.get_identifier();
        match role {
            MessageRole::User => {}
            _ => {
                return Err(OpenApiError::RestrictedValue(
                    "MessageRole must be of type User".into(),
                ))
            }
        }
        Ok(Self {
            thread_id,
            role,
            content,
            file_ids: None,
            metadata: None,
        })
    }

    pub fn with_file_ids(mut self, file_ids: Vec<String>) -> Self {
        self.file_ids = Some(file_ids);
        self
    }

    pub fn add_file_id<I: Identifiable>(mut self, file_id: I) -> Self {
        let file_id = file_id.get_identifier();
        match &mut self.file_ids {
            Some(file_ids) => file_ids.push(file_id),
            None => self.file_ids = Some(vec![file_id]),
        }
        self
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(&self, networking: &Networking) -> Result<Message, OpenApiError> {
        networking.create_message(self, &self.thread_id)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageFile {
    id: String,
    object: String,
    created_at: i64,
    message_id: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    #[default]
    User,
    Assistant,
}

impl From<&str> for MessageRole {
    fn from(s: &str) -> Self {
        match s {
            "system" => MessageRole::System,
            "user" => MessageRole::User,
            "assistant" => MessageRole::Assistant,
            _ => MessageRole::User,
        }
    }
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

/** ---- General Message ----
Used for representing the various message objects

*/

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct GeneralMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    role: MessageRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<ToolCalls>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CompletionMessages {
    SystemMessage(GeneralMessage),
    UserMessage(GeneralMessage),
    AssistantMessage(GeneralMessage),
    ToolMessage(GeneralMessage),
}

impl CompletionMessages {
    pub fn new_system(content: String, name: Option<String>) -> CompletionMessages {
        CompletionMessages::SystemMessage(GeneralMessage {
            content: Some(content),
            role: "system".into(),
            name,
            ..GeneralMessage::default()
        })
    }

    pub fn new_user(content: String, name: Option<String>) -> CompletionMessages {
        CompletionMessages::UserMessage(GeneralMessage {
            content: Some(content),
            role: "user".into(),
            name,
            ..GeneralMessage::default()
        })
    }

    pub fn new_assistant(
        content: Option<String>,
        name: Option<String>,
        tool_calls: Option<Vec<ToolCalls>>,
    ) -> CompletionMessages {
        CompletionMessages::AssistantMessage(GeneralMessage {
            content,
            role: "assistant".into(),
            name,
            tool_calls,
            ..GeneralMessage::default()
        })
    }

    pub fn new_tool(content: String, tool_call_id: Option<String>) -> CompletionMessages {
        CompletionMessages::ToolMessage(GeneralMessage {
            content: Some(content),
            role: "tool".into(),
            tool_call_id,
            ..GeneralMessage::default()
        })
    }
}
