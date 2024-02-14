use serde;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::str::FromStr;

use crate::openai::networking::Networking;
use crate::openai::types::models::Model;
use crate::openai::types::{Identifiable, MessageRole, OpenApiError, ToolCalls, Usage};

// TODO: Add derives

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct UnixEpoch(i64);

impl UnixEpoch {
    pub fn new(epoch: i64) -> Result<Self, String> {
        if UnixEpoch::is_valid(epoch) {
            Ok(UnixEpoch(epoch))
        } else {
            Err("Invalid epoch supplied".to_string())
        }
    }

    fn is_valid(epoch: i64) -> bool {
        if epoch < 0 {
            return false;
        }

        true
    }

    pub fn value(&self) -> i64 {
        return self.0;
    }
}

#[derive(Clone, Default, Debug)]
pub struct MessageList {
    messages: Vec<Message>,
}

impl MessageList {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn push(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    pub fn to_string(&self) -> String {
        String::from("[{}]")
    }
}

impl Serialize for MessageList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.messages.len())).unwrap();
        for item in &self.messages {
            seq.serialize_element(item).unwrap();
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for MessageList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(MessageListVisitor)
    }
}

struct MessageListVisitor;

impl<'de> Visitor<'de> for MessageListVisitor {
    type Value = MessageList;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a sequence of messages")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<MessageList, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut messages = if let Some(size) = seq.size_hint() {
            Vec::with_capacity(size)
        } else {
            Vec::new()
        };

        while let Some(msg) = seq.next_element()? {
            messages.push(msg);
        }

        Ok(MessageList { messages })
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Message {
    content: String,
    role: MessageRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<ToolCalls>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

impl Message {
    pub fn new(
        content: String,
        role: MessageRole,
        name: Option<String>,
        tool_calls: Option<Vec<ToolCalls>>,
        tool_call_id: Option<String>,
    ) -> Self {
        Self {
            content,
            role,
            name,
            tool_calls,
            tool_call_id,
        }
    }

    pub fn new_user(content: String, name: Option<String>) -> Self {
        Self {
            content,
            role: MessageRole::User,
            name,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn new_system(content: String, name: Option<String>) -> Self {
        Self {
            content,
            role: MessageRole::System,
            name,
            tool_calls: None,
            tool_call_id: None,
        }
    }
}

#[derive(Default, Serialize, Clone, Deserialize, Debug)]
pub struct ChatBuilder {
    networking: Networking,
    messages: MessageList,
    model: String, // Model ID, not Model obj (Model.get_id)

    frequency_penalty: Option<f64>,
    logit_bias: Option<HashMap<String, i32>>, // TODO: Verify type
    logprobs: Option<bool>,
    top_logprobs: Option<i8>,
    max_tokens: Option<i32>,
    n: Option<i8>,
    presence_penalty: Option<f64>,
    response_format: Option<HashMap<String, String>>,
    seed: Option<i32>,
    //stop: // TODO: Implement stop feature
    stream: Option<bool>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    //tools: Vec<>, // TODO: Implement tools feature
    //tool_choice // TODO: Implement tool_choice
    user: Option<String>,
}

impl ChatBuilder {
    pub fn new<T: Identifiable>(model: T, messages: MessageList) -> Self {
        ChatBuilder {
            model: model.get_identifier(),
            messages,
            ..ChatBuilder::default()
        }
    }

    pub fn to_hashmap(&self) -> HashMap<String, String> {
        serde_json::to_value(&self)
            .unwrap()
            .as_object()
            .unwrap()
            .into_iter()
            .filter(|(k, v)| !v.is_null())
            .map(|(k, v)| (k.clone(), v.clone().to_string()))
            .collect()
    }

    pub fn print_hashmap(&self) {
        for (k, v) in self.to_hashmap().iter() {
            println!("{0}, {1}", k, v)
        }
    }

    pub fn build(&self) -> Result<ChatCompletion, OpenApiError> {
        self.networking.create_chat_completion(self)
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct ChatCompletion {
    id: String,
    choices: Vec<Choices>,
    created: i64,
    model: String,
    system_fingerprint: Option<String>,
    object: String,
    usage: Usage,
}

impl ChatCompletion {
    pub fn ping(&self) -> bool {
        true
    }
}

#[derive(Clone, Deserialize, Debug)]
struct Choices {
    finish_reason: String,
    index: usize,
    message: Message, // Chat Message
    logprobs: Option<WeightedLogProb>,
}

#[derive(Clone, Deserialize, Debug)]
struct WeightedLogProb {
    token: String,
    logprob: u32,
    bytes: Option<Vec<u8>>,
    top_logprobs: Vec<LogProb>,
}

#[derive(Clone, Deserialize, Debug)]
struct LogProb {
    token: String,
    logprob: u32,
    bytes: Option<Vec<u8>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_role() {
        assert_eq!(MessageRole::from_str("user").unwrap(), MessageRole::User)
    }

    #[test]
    fn test_message_role_from_str() {
        let s: String = "system".to_string();
        let mr: MessageRole = s.parse().unwrap();
        assert_eq!(mr, MessageRole::System);
    }

    #[test]
    #[should_panic]
    fn test_bad_message_role() {
        let s: String = "ssystem".to_string();
        let mr: MessageRole = s.parse().unwrap();
        assert_eq!(mr, MessageRole::System)
    }
}
