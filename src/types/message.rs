use crate::impl_ref;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

use crate::networking::Networking;
use crate::types::common::{Identifiable, ToolCalls};
use crate::types::error::OpenApiError;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
impl_ref!(Message, Identifiable);

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
    ///
    /// The constructor **and entry point** for the `MessageBuilder` object.
    ///
    /// # Arguments
    ///
    /// * `thread_id` - An object implementing the `Identifiable` trait. This is used to identify the thread to which the message belongs.
    /// * `content` - A `String` representing the content of the message.
    ///
    /// # Returns
    ///
    /// * `Result<Self, OpenApiError>` - Returns an instance of `MessageBuilder` if successful, or an `OpenApiError` if an error occurs.
    ///
    /// # IMPORTANT NOTE
    ///
    /// The `MessageRole` must be of variant `User`, thus no parameter for this, however this may change in future!
    ///
    // REMOVED
    // /// * `role` - A `MessageRole` enum indicating the role of the message. This can be `System`, `User`, or `Assistant`.
    pub fn new<I: Identifiable, C: Into<String>>(
        thread_id: I,
        //role: MessageRole,
        content: C,
    ) -> Result<Self, OpenApiError> {
        let thread_id = thread_id.get_identifier();
        let content = content.into();
        let role = MessageRole::User;
        // match role {
        //     MessageRole::User => {}
        //     _ => {
        //         return Err(OpenApiError::RestrictedValue(
        //             "MessageRole must be of type User".into(),
        //         ))
        //     }
        // }
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum MessageContent {
    Text(TextContent),
    ImageFile(ImageContent),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TextContent {
    r#type: String,
    text: TextValue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TextValue {
    value: String,
    annotations: Vec<Annotations>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ImageContent {
    r#type: String,
    image_file: ImageLocation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ImageLocation {
    file_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum Annotations {
    FileCitation(FileCitation),
    FilePath(FilePath),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FileCitation {
    r#type: String,
    text: String,
    file_citation: FileCitationLocation,
    start_index: u32,
    end_index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FileCitationLocation {
    file_id: String,
    quote: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FilePath {
    r#type: String,
    text: String,
    file_path: FilePathLocation,
    start_index: u32,
    end_index: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl GeneralMessage {
    pub fn get_content(&self) -> Option<String> {
        self.content.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CompletionMessage {
    SystemMessage(GeneralMessage),
    UserMessage(GeneralMessage),
    AssistantMessage(GeneralMessage),
    ToolMessage(GeneralMessage),
}

impl CompletionMessage {
    pub fn new_system(content: String, name: Option<String>) -> CompletionMessage {
        CompletionMessage::SystemMessage(GeneralMessage {
            content: Some(content),
            role: "system".into(),
            name,
            ..GeneralMessage::default()
        })
    }

    pub fn new_user(content: String, name: Option<String>) -> CompletionMessage {
        CompletionMessage::UserMessage(GeneralMessage {
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
    ) -> CompletionMessage {
        CompletionMessage::AssistantMessage(GeneralMessage {
            content,
            role: "assistant".into(),
            name,
            tool_calls,
            ..GeneralMessage::default()
        })
    }

    pub fn new_tool(content: String, tool_call_id: Option<String>) -> CompletionMessage {
        CompletionMessage::ToolMessage(GeneralMessage {
            content: Some(content),
            role: "tool".into(),
            tool_call_id,
            ..GeneralMessage::default()
        })
    }
}
