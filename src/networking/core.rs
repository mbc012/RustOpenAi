use std::collections::HashMap;
use std::fmt::format;
use std::hash::Hash;
use std::path::PathBuf;
use std::string::ToString;

use serde_json::{Map, Value};

use crate::types::assistant::{Assistant, AssistantBuilder, AssistantFile, AssistantFileBuilder};
use crate::types::chat::{ChatCompletion, ChatCompletionBuilder};
use crate::types::common::{ApiList, DeletionStatus, Identifiable};
use crate::types::error::OpenApiError;
use crate::types::file::{File, FileBuilder};
use crate::types::message::{Message, MessageBuilder, MessageFile};
use crate::types::model::Model;
use crate::types::moderation::Moderation;
use crate::types::run::{Run, RunBuilder, RunStep};
use crate::types::thread::{Thread, ThreadBuilder};

use crate::file::FileTypes;
use crate::strip_edges;
use reqwest::blocking::multipart;
use reqwest::{blocking::Client, header::HeaderMap, Method, Url};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Networking {
    apikey: String,
    organization_id: Option<String>,
    #[serde(skip)]
    client: Client,
}

impl Networking {
    /** ---- Common Networking ---- */

    pub fn new(apikey: String, organization_id: Option<String>) -> Self {
        Self {
            apikey,
            organization_id,
            client: Client::new(),
        }
    }

    fn construct_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.apikey).parse().unwrap(),
        );
        headers.insert("OpenAI-Beta", "assistants=v1".parse().unwrap());
        if let Some(org_id) = &self.organization_id {
            headers.insert("OpenAI-Organization", org_id.parse().unwrap());
        }
        headers
    }

    fn construct_url(&self, endpoint: String) -> Result<Url, OpenApiError> {
        let base_uri: &str = "https://api.openai.com/v1/";
        Url::parse((base_uri.to_string() + endpoint.as_str()).as_str()).map_err(OpenApiError::from)
    }

    fn send_request(
        &self,
        method: Method,
        endpoint: String,
        body: Option<Value>,
        multipart_form: Option<multipart::Form>,
    ) -> Result<Value, OpenApiError> {
        let url = self.construct_url(endpoint)?;
        let mut request_builder = self
            .client
            .request(method, url)
            .headers(self.construct_headers());
        if let Some(b) = body {
            request_builder = request_builder
                .body(serde_json::to_string(&b)?)
                .header("Content-Type", "application/json")
        }
        if let Some(mf) = multipart_form {
            request_builder = request_builder.multipart(mf)
        }
        let res = request_builder.send()?;
        res.json().map_err(OpenApiError::from)
    }

    pub fn send_and_convert<T: DeserializeOwned>(
        &self,
        method: Method,
        endpoint: String,
        body: Option<Value>,
        multipart_form: Option<multipart::Form>,
    ) -> Result<T, OpenApiError> {
        self.send_request(method, endpoint.clone(), body, multipart_form)
            .and_then(|val| {
                println!("[{0}] - {1}", endpoint, serde_json::to_string(&val)?);
                Ok(val)
            })
            .and_then(|val| serde_json::from_value::<T>(val).map_err(OpenApiError::from))
    }

    /** ---- Moderation ----
     * No builder for create_moderation, uses client method
     */

    pub fn create_moderation(
        &self,
        payload: HashMap<String, String>,
    ) -> Result<Moderation, OpenApiError> {
        self.send_and_convert(
            Method::POST,
            String::from("moderations"),
            Some(serde_json::to_value(payload)?),
            None,
        )
    }
}
