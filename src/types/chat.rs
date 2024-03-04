use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::str::FromStr;

use crate::networking::Networking;
use crate::types::common::{Identifiable, ToolCalls, Tools, Usage};
use crate::types::error::OpenApiError;
use crate::types::message::{CompletionMessages, GeneralMessage};

#[derive(Default, Serialize, Clone, Deserialize, Debug)]
pub struct ChatBuilder {
    messages: Vec<CompletionMessages>,
    model: String,

    frequency_penalty: Option<f64>,
    logit_bias: Option<HashMap<String, i32>>, // TODO: Verify type
    logprobs: Option<bool>,
    top_logprobs: Option<i8>,
    max_tokens: Option<i32>,
    n: Option<i8>,
    presence_penalty: Option<f64>,
    response_format: Option<HashMap<String, String>>, // TODO
    seed: Option<i32>,
    stop: Vec<String>,
    stream: Option<bool>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    tools: Vec<Tools>,
    //tool_choice // TODO: Implement tool_choice
    user: Option<String>,
}

impl ChatBuilder {
    pub fn new<T: Identifiable>(model: T, messages: Vec<CompletionMessages>) -> Self {
        ChatBuilder {
            model: model.get_identifier(),
            messages,
            ..ChatBuilder::default()
        }
    }

    pub fn with_frequency_penalty(mut self, frequency_penalty: f64) -> Result<Self, OpenApiError> {
        match frequency_penalty {
            -2.0..=2.0 => {
                self.frequency_penalty = Some(frequency_penalty);
                Ok(self)
            }
            _ => Err(OpenApiError::RestrictedValue(
                "Frequency Penalty must be between -2 and 2".into(),
            )),
        }
    }

    pub fn with_logit_bias(
        mut self,
        logit_bias: HashMap<String, i32>,
    ) -> Result<Self, OpenApiError> {
        // Iterate through and check logit bias vals are valid.
        for (_k, v) in logit_bias.iter() {
            if v < &-100 || v > &100 {
                return Err(OpenApiError::RestrictedValue(
                    "Logit Bias values must be between -100 and 100".into(),
                ));
            }
        }
        self.logit_bias = Some(logit_bias);
        Ok(self)
    }

    pub fn with_logprobs(mut self, logprobs: bool) -> Result<Self, OpenApiError> {
        // Restrict models
        if self.model == "gpt-4-vision-preview" {
            return Err(OpenApiError::RestrictedValue(
                "Logprobs is not supported for gpt-4-vision-preview".into(),
            ));
        }

        self.logprobs = Some(logprobs);
        Ok(self)
    }

    pub fn with_top_logprobs(mut self, top_logprobs: i8) -> Result<Self, OpenApiError> {
        if (self.logprobs.is_none() || self.logprobs.unwrap() == false) {
            return Err(OpenApiError::RestrictedValue(
                "Top Logprobs requires logprobs to be true".into(),
            ));
        }

        self.top_logprobs = Some(top_logprobs);
        Ok(self)
    }

    pub fn with_max_tokens(mut self, max_tokens: i32) -> Result<Self, OpenApiError> {
        // 32,768 for newer models
        // 4096 for older models
        match max_tokens {
            1..=32768 => {
                self.max_tokens = Some(max_tokens);
                Ok(self)
            }
            _ => Err(OpenApiError::RestrictedValue(
                "Max Tokens must be between 1 and 32,768".into(),
            )),
        }
    }

    pub fn with_choice_count(mut self, n: i8) -> Self {
        self.n = Some(n);
        self
    }

    pub fn with_presence_penalty(mut self, presence_penalty: f64) -> Result<Self, OpenApiError> {
        match presence_penalty {
            -2.0..=2.0 => {
                self.presence_penalty = Some(presence_penalty);
                Ok(self)
            }
            _ => Err(OpenApiError::RestrictedValue(
                "Presence Penalty must be between -2.0 and 2.0".into(),
            )),
        }
    }

    pub fn with_response_format(mut self, response_format: HashMap<String, String>) -> Self {
        // TODO
        self.response_format = Some(response_format);
        self
    }

    pub fn with_seed(mut self, seed: i32) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn with_stop(mut self, stop: Vec<String>) -> Result<Self, OpenApiError> {
        match stop.len() {
            0..=4 => {
                self.stop = stop;
                Ok(self)
            }
            _ => Err(OpenApiError::InvalidLength(stop.len(), 4)),
        }
    }

    pub fn with_stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    pub fn with_temperature(mut self, temperature: f64) -> Result<Self, OpenApiError> {
        match temperature {
            0.0..=2.0 => {
                self.temperature = Some(temperature);
                Ok(self)
            }
            _ => Err(OpenApiError::RestrictedValue(
                "Temperature must be between 0.0 and 2.0".into(),
            )),
        }
    }

    pub fn with_top_p(mut self, top_p: f64) -> Result<Self, OpenApiError> {
        match top_p {
            0.0..=2.0 => {
                self.top_p = Some(top_p);
                Ok(self)
            }
            _ => Err(OpenApiError::RestrictedValue(
                "Top P must be between 0.0 and 2.0".into(),
            )),
        }
    }

    pub fn with_tools(mut self, tools: Vec<Tools>) -> Self {
        self.tools = tools;
        self
    }

    pub fn with_tool_choice(mut self, tool_choice: String) -> Self {
        todo!()
    }

    pub fn with_user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }

    pub fn build(&self, networking: &Networking) -> Result<ChatCompletion, OpenApiError> {
        networking.create_chat_completion(self)
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct ChatCompletion {
    id: String,
    choices: Vec<Choice>,
    created: i64,
    model: String,
    system_fingerprint: Option<String>,
    object: String,
    usage: Usage,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ChatCompletionChunk {
    id: String,
    choices: Vec<Choice>,
    created: i64,
    model: String,
    system_fingerprint: Option<String>,
    object: String,
}

#[derive(Clone, Deserialize, Debug)]
struct Choice {
    finish_reason: Option<String>,
    index: u32,
    logprobs: Option<LogProbChoice>,
    message: Option<GeneralMessage>, // Chat Message
    delta: Option<Delta>,
}

#[derive(Clone, Deserialize, Debug)]
struct Delta {
    content: Option<String>,
    tool_calls: Vec<ToolCalls>,
    role: String,
}

#[derive(Clone, Deserialize, Debug)]
struct LogProbChoice {
    content: LogProb,
}

#[derive(Clone, Deserialize, Debug)]
struct LogProb {
    token: String,
    logprob: f64,
    bytes: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_logprobs: Option<Vec<LogProb>>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
