use crate::chat::{ChatCompletion, ChatCompletionBuilder};
use crate::error::OpenApiError;
use crate::networking::Networking;
use reqwest::Method;

pub trait ChatCompletionActions {
    fn create_chat_completion(
        &self,
        payload: &ChatCompletionBuilder,
    ) -> Result<ChatCompletion, OpenApiError>;
}

impl ChatCompletionActions for Networking {
    fn create_chat_completion(
        &self,
        payload: &ChatCompletionBuilder,
    ) -> Result<ChatCompletion, OpenApiError> {
        self.send_and_convert(
            Method::POST,
            String::from("chat/completions"),
            Some(serde_json::to_value(payload)?),
            None,
        )
    }
}
