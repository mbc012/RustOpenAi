use crate::networking::Networking;
use crate::types::chat::MessageList; // TODO FIXME - MessageList needs to be changed to use ApiList
use crate::types::common::Identifiable;
use crate::types::error::OpenApiError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Thread {
    id: String,
    object: String,
    created_at: i64,
    metadata: HashMap<String, String>,
}

impl Identifiable for Thread {
    fn get_identifier(&self) -> String {
        self.id.clone().to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThreadBuilder {
    #[serde(skip)]
    networking: Networking,
    messages: Option<MessageList>,
    metadata: Option<HashMap<String, String>>,
}

impl ThreadBuilder {
    pub fn new_empty(networking: Networking) -> Self {
        Self {
            networking,
            messages: None,
            metadata: None,
        }
    }

    pub fn new(
        networking: Networking,
        messages: Option<MessageList>,
        metadata: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            networking,
            messages,
            metadata,
        }
    }

    pub fn with_messages(mut self, messages: MessageList) -> Self {
        self.messages = Some(messages);
        self
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> Result<Thread, OpenApiError> {
        self.networking.create_thread(self.clone())
    }
}
