use std::collections::HashMap;
use std::fmt::format;
use std::hash::Hash;
use std::string::ToString;

use serde_json::{Map, Value};

use crate::types::assistant::{Assistant, AssistantBuilder, AssistantFile, AssistantFileBuilder};
use crate::types::chat::{ChatBuilder, ChatCompletion};
use crate::types::common::{ApiList, DeletionStatus};
use crate::types::error::OpenApiError;
use crate::types::file::{File, FileBuilder};
use crate::types::message::{Message, MessageBuilder, MessageFile};
use crate::types::model::Model;
use crate::types::moderation::Moderation;
use crate::types::run::{Run, RunBuilder, RunStep};
use crate::types::thread::{Thread, ThreadBuilder};

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

    fn send_and_convert<T: DeserializeOwned>(
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

    /** ---- Chat Completion ---- */

    pub fn create_chat_completion(
        &self,
        payload: &ChatBuilder,
    ) -> Result<ChatCompletion, OpenApiError> {
        self.send_and_convert(
            Method::POST,
            String::from("chat/completions"),
            Some(serde_json::to_value(payload)?),
            None,
        )
    }

    /** ---- File ---- */

    pub fn upload_file(&self, payload: &FileBuilder) -> Result<File, OpenApiError> {
        let form = multipart::Form::new()
            .text("purpose", payload.purpose_str())
            .file("file", payload.get_path_buff().as_path())?;

        self.send_and_convert(Method::POST, String::from("files"), None, Some(form))
    }

    pub fn list_files(&self) -> Result<Vec<File>, OpenApiError> {
        self.send_and_convert(Method::GET, String::from("files"), None, None)
    }

    pub fn retrieve_file(&self, file_id: String) -> Result<File, OpenApiError> {
        self.send_and_convert(Method::GET, format!("files/{}", file_id), None, None)
    }

    pub fn delete_file(&self, file_id: String) -> Result<DeletionStatus, OpenApiError> {
        self.send_and_convert(Method::DELETE, format!("files/{}", file_id), None, None)
    }

    pub fn retrieve_file_content(&self, file_id: String) -> Result<File, OpenApiError> {
        self.send_and_convert(
            Method::GET,
            format!("files/{}/content", file_id),
            None,
            None,
        )
    }

    /** ---- Model ---- */

    pub fn list_models(&self) -> Result<ApiList<Model>, OpenApiError> {
        self.send_and_convert(Method::GET, String::from("models"), None, None)
    }

    pub fn load_model(&self, model_id: String) -> Result<Model, OpenApiError> {
        self.send_and_convert(Method::GET, format!("models/{}", model_id), None, None)
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
            String::from("content/moderation"),
            Some(serde_json::to_value(payload)?),
            None,
        )
    }

    /** ---- Assistant + Assistant files ---- */

    pub fn create_assistant(&self, payload: &AssistantBuilder) -> Result<Assistant, OpenApiError> {
        self.send_and_convert(
            Method::POST,
            String::from("assistants"),
            Some(serde_json::to_value(payload)?),
            None,
        )
    }

    pub fn create_assistant_file(
        &self,
        payload: &AssistantFileBuilder,
        assistant_id: &String,
    ) -> Result<AssistantFile, OpenApiError> {
        self.send_and_convert(
            Method::POST,
            format!("assistants/{}/files", assistant_id),
            Some(serde_json::to_value(payload)?),
            None,
        )
    }

    pub fn list_assistants(
        &self,
        params: Option<&String>,
    ) -> Result<ApiList<Assistant>, OpenApiError> {
        // Add param support
        self.send_and_convert(Method::GET, String::from("assistants"), None, None)
    }

    pub fn list_assistant_files(
        &self,
        assistant_id: String,
    ) -> Result<ApiList<AssistantFile>, OpenApiError> {
        self.send_and_convert(
            Method::GET,
            format!("assistants/{}/files", assistant_id),
            None,
            None,
        )
    }

    pub fn retrieve_assistant(&self, assistant_id: String) -> Result<Assistant, OpenApiError> {
        self.send_and_convert(
            Method::GET,
            format!("assistants/{}", assistant_id),
            None,
            None,
        )
    }

    pub fn retrieve_assistant_file(
        &self,
        assistant_id: String,
        file_id: String,
    ) -> Result<AssistantFile, OpenApiError> {
        self.send_and_convert(
            Method::GET,
            format!("assistants/{}/files/{}", assistant_id, file_id),
            None,
            None,
        )
    }

    pub fn modify_assistant(
        &self,
        assistant_id: String,
        payload: &AssistantBuilder,
    ) -> Result<Assistant, OpenApiError> {
        self.send_and_convert(
            Method::PATCH,
            format!("assistants/{}", assistant_id),
            Some(serde_json::to_value(payload)?),
            None,
        )
    }

    pub fn delete_assistant(&self, assistant_id: String) -> Result<DeletionStatus, OpenApiError> {
        self.send_and_convert(
            Method::DELETE,
            format!("assistants/{}", assistant_id),
            None,
            None,
        )
    }

    pub fn delete_assistant_file(
        &self,
        assistant_id: String,
        file_id: String,
    ) -> Result<DeletionStatus, OpenApiError> {
        self.send_and_convert(
            Method::DELETE,
            format!("assistants/{}/files/{}", assistant_id, file_id),
            None,
            None,
        )
    }

    /** ---- Threads ---- */

    pub fn create_thread(&self, payload: &ThreadBuilder) -> Result<Thread, OpenApiError> {
        self.send_and_convert(
            Method::POST,
            String::from("threads"),
            Some(serde_json::to_value(payload)?),
            None,
        )
    }

    pub fn retrieve_thread(&self, thread_id: String) -> Result<Thread, OpenApiError> {
        self.send_and_convert(Method::GET, format!("threads/{}", thread_id), None, None)
    }

    pub fn modify_thread(
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

    pub fn delete_thread(&self, thread_id: String) -> Result<DeletionStatus, OpenApiError> {
        self.send_and_convert(Method::DELETE, format!("threads/{}", thread_id), None, None)
    }

    /** ---- Messages ---- */

    pub fn create_message(
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

    pub fn list_messages(&self, thread_id: String) -> Result<ApiList<Message>, OpenApiError> {
        self.send_and_convert(
            Method::GET,
            format!("threads/{}/messages", thread_id),
            None,
            None,
        )
    }

    pub fn list_message_files(
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

    pub fn retrieve_message(
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

    pub fn retrieve_message_file(
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

    pub fn modify_message(
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

    /** ---- Runs ---- */

    pub fn create_run<S: Serialize>(
        &self,
        payload: &S,
        thread_id: &Option<String>,
    ) -> Result<Run, OpenApiError> {
        let endpoint: String = match thread_id {
            Some(tid) => format!("threads/{}/runs", tid),
            None => String::from("threads/runs"),
        };
        self.send_and_convert(
            Method::POST,
            endpoint,
            Some(serde_json::to_value(payload)?),
            None,
        )
    }

    pub fn retrieve_run(&self, thread_id: String, run_id: String) -> Result<Run, OpenApiError> {
        self.send_and_convert(
            Method::GET,
            format!("threads/{0}/runs/{1}", thread_id, run_id),
            None,
            None,
        )
    }

    pub fn retrieve_run_step(
        &self,
        thread_id: String,
        run_id: String,
        step_id: String,
    ) -> Result<RunStep, OpenApiError> {
        self.send_and_convert(
            Method::GET,
            format!("threads/{0}/runs/{1}/steps/{2}", thread_id, run_id, step_id),
            None,
            None,
        )
    }

    pub fn list_runs(&self, thread_id: String) -> Result<ApiList<Run>, OpenApiError> {
        self.send_and_convert(
            Method::GET,
            format!("threads/{0}/runs", thread_id),
            None,
            None,
        )
    }

    pub fn list_run_steps(
        &self,
        thread_id: String,
        run_id: String,
    ) -> Result<ApiList<RunStep>, OpenApiError> {
        self.send_and_convert(
            Method::GET,
            format!("threads/{0}/runs/{1}/steps", thread_id, run_id),
            None,
            None,
        )
    }
}
