use crate::common::ApiList;
use crate::error::OpenApiError;
use crate::message::{Message, MessageBuilder, MessageFile};
use crate::networking::Networking;
use reqwest::Method;
use std::collections::HashMap;

pub trait MessageActions {
    fn create_message(
        &self,
        payload: &MessageBuilder,
        thread_id: &String,
    ) -> Result<Message, OpenApiError>;
    fn list_messages(&self, thread_id: String) -> Result<ApiList<Message>, OpenApiError>;
    fn list_message_files(
        &self,
        thread_id: String,
        message_id: String,
    ) -> Result<ApiList<MessageFile>, OpenApiError>;
    fn retrieve_message(
        &self,
        thread_id: String,
        message_id: String,
    ) -> Result<Message, OpenApiError>;
    fn retrieve_message_file(
        &self,
        thread_id: String,
        message_id: String,
        file_id: String,
    ) -> Result<MessageFile, OpenApiError>;
    fn modify_message(
        &self,
        thread_id: String,
        message_id: String,
        metadata: HashMap<String, String>,
    ) -> Result<Message, OpenApiError>;
}

impl MessageActions for Networking {
    fn create_message(
        &self,
        payload: &MessageBuilder,
        thread_id: &String,
    ) -> Result<Message, OpenApiError> {
        self.send_and_convert(
            Method::POST,
            format!("threads/{}/messages", thread_id),
            Some(serde_json::to_value(payload)?),
            None,
        )
    }

    fn list_messages(&self, thread_id: String) -> Result<ApiList<Message>, OpenApiError> {
        self.send_and_convert(
            Method::GET,
            format!("threads/{}/messages", thread_id),
            None,
            None,
        )
    }

    fn list_message_files(
        &self,
        thread_id: String,
        message_id: String,
    ) -> Result<ApiList<MessageFile>, OpenApiError> {
        self.send_and_convert(
            Method::GET,
            format!("threads/{0}/messages/{1}/files", thread_id, message_id),
            None,
            None,
        )
    }

    fn retrieve_message(
        &self,
        thread_id: String,
        message_id: String,
    ) -> Result<Message, OpenApiError> {
        self.send_and_convert(
            Method::GET,
            format!("threads/{0}/messages/{1}", thread_id, message_id),
            None,
            None,
        )
    }

    fn retrieve_message_file(
        &self,
        thread_id: String,
        message_id: String,
        file_id: String,
    ) -> Result<MessageFile, OpenApiError> {
        self.send_and_convert(
            Method::GET,
            format!(
                "threads/{0}/messages/{1}/files/{2}",
                thread_id, message_id, file_id
            ),
            None,
            None,
        )
    }

    fn modify_message(
        &self,
        thread_id: String,
        message_id: String,
        metadata: HashMap<String, String>,
    ) -> Result<Message, OpenApiError> {
        self.send_and_convert(
            Method::POST,
            format!("threads/{0}/messages/{1}", thread_id, message_id),
            Some(serde_json::to_value(&metadata)?),
            None,
        )
    }
}
