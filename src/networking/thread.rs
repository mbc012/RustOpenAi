use crate::common::DeletionStatus;
use crate::error::OpenApiError;
use crate::networking::Networking;
use crate::thread::{Thread, ThreadBuilder};
use reqwest::Method;
use serde_json::{Map, Value};
use std::collections::HashMap;

pub trait ThreadActions {
    fn create_thread(&self, payload: &ThreadBuilder) -> Result<Thread, OpenApiError>;
    fn retrieve_thread(&self, thread_id: String) -> Result<Thread, OpenApiError>;
    fn modify_thread(
        &self,
        thread_id: String,
        metadata: HashMap<String, String>,
    ) -> Result<Thread, OpenApiError>;
    fn delete_thread(&self, thread_id: String) -> Result<DeletionStatus, OpenApiError>;
}

impl ThreadActions for Networking {
    fn create_thread(&self, payload: &ThreadBuilder) -> Result<Thread, OpenApiError> {
        self.send_and_convert(
            Method::POST,
            String::from("threads"),
            Some(serde_json::to_value(payload)?),
            None,
        )
    }

    fn retrieve_thread(&self, thread_id: String) -> Result<Thread, OpenApiError> {
        self.send_and_convert(Method::GET, format!("threads/{}", thread_id), None, None)
    }

    fn modify_thread(
        &self,
        thread_id: String,
        metadata: HashMap<String, String>,
    ) -> Result<Thread, OpenApiError> {
        let mut payload: Map<String, Value> = Map::new();
        payload.insert(
            "metadata".to_string(),
            Value::Object(
                metadata
                    .iter()
                    .map(|(k, v)| (k.clone(), Value::String(v.clone())))
                    .collect(),
            ),
        );
        self.send_and_convert(
            Method::PATCH,
            format!("threads/{}", thread_id),
            Some(serde_json::to_value(payload)?),
            None,
        )
    }

    fn delete_thread(&self, thread_id: String) -> Result<DeletionStatus, OpenApiError> {
        self.send_and_convert(Method::DELETE, format!("threads/{}", thread_id), None, None)
    }
}
