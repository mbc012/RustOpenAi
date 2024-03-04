use crate::networking::Networking;
use crate::types::common::{ApiList, Identifiable};
use crate::types::error::OpenApiError;
use crate::types::message::GeneralMessage;
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
        self.id.clone()
    }
}

impl<'a> Identifiable for &'a Thread {
    fn get_identifier(&self) -> String {
        self.id.clone()
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ThreadBuilder {
    messages: Option<Vec<GeneralMessage>>,
    metadata: Option<HashMap<String, String>>,
}

impl ThreadBuilder {
    pub fn new() -> Self {
        ThreadBuilder::default()
    }

    pub fn with_messages(mut self, messages: Vec<GeneralMessage>) -> Self {
        self.messages = Some(messages);
        self
    }

    pub fn add_message(mut self, message: GeneralMessage) -> Self {
        match &mut self.messages {
            Some(messages) => messages.push(message),
            None => self.messages = Some(vec![message]),
        }
        self
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(&self, networking: &Networking) -> Result<Thread, OpenApiError> {
        networking.create_thread(self)
    }
}
