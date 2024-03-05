use crate::moderation::{ModerationCategories, ModerationType};
use std::collections::HashMap;
use std::env;

use crate::networking::Networking;
use crate::strip_edges;
use crate::types::assistant::{Assistant, AssistantFile};
use crate::types::common::{ApiList, DeletionStatus, Identifiable};
use crate::types::error::OpenApiError;
use crate::types::file::File;
use crate::types::message::{Message, MessageFile};
use crate::types::model::Model;
use crate::types::moderation::Moderation;
use crate::types::run::{Run, RunStep};
use crate::types::thread::Thread;

/// Client for interacting with OpenAI's API.
///
/// # Arguments
/// - `networking`: A `Networking` struct that handles all networking with the API using the reqwest crate
///
/// # Examples
/// ```
/// todo!()
/// ```
///
///
/// # Panics
/// N/a
///
///
/// # Safety
/// N/a
///
///
/// # Errors
/// May return an `OpenApiError` error type.
///
pub struct OpenAIClient {
    networking: Networking,
}

impl OpenAIClient {
    /// Attempts to pull OPEN_API_KEY from ENV
    pub fn obtain_key() -> Result<String, OpenApiError> {
        env::var("OPENAI_API_KEY").map_err(|e| OpenApiError::ClientError(e.to_string()))
    }

    /// Prompts user for OPEN_API_KEY value, then sets it in the ENV
    pub fn prompt_key() -> Result<String, OpenApiError> {
        // Request user input
        let mut key_buff = String::new();
        println!("Please enter your OpenAI API key: ");
        std::io::stdin()
            .read_line(&mut key_buff)
            .map_err(|e| OpenApiError::ClientError(e.to_string()))?;

        // Set env and return key
        let trimmed_key = key_buff.trim();
        env::set_var("OPENAI_API_KEY", trimmed_key);
        Ok(trimmed_key.to_string())
    }

    /// Generate a new OpenAIClient supplying both the Apikey and Organization ID
    pub fn new(apikey: String, organization_id: Option<String>) -> Result<Self, OpenApiError> {
        Ok(Self {
            networking: Networking::new(apikey, organization_id.clone()),
        })
    }

    /// Generate a new OpenAIClient and if apikey is not found return an error
    pub fn new_with_env(organization_id: Option<String>) -> Result<Self, OpenApiError> {
        let apikey = OpenAIClient::obtain_key()?;
        Ok(Self {
            networking: Networking::new(apikey, organization_id.clone()),
        })
    }

    /// Generate a new OpenAIClient and if apikey is not found, prompt user for key and set env.
    pub fn new_with_prompt(organization_id: Option<String>) -> Result<Self, OpenApiError> {
        let mut apikey = OpenAIClient::obtain_key();
        if apikey.is_err() {
            apikey = OpenAIClient::prompt_key()
        }
        let apikey = apikey?;
        Ok(Self {
            networking: Networking::new(apikey, organization_id.clone()),
        })
    }

    /// Retrieve a client networking reference
    pub fn netref(&self) -> &Networking {
        &self.networking
    }

    /* MODELS */
    pub fn load_model<T: Identifiable>(&self, model: T) -> Result<Model, OpenApiError> {
        let model_id: String = model.get_identifier();
        self.networking.load_model(model_id)
    }

    pub fn list_models(&self) -> Result<ApiList<Model>, OpenApiError> {
        self.networking.list_models()
    }

    /* FILES */
    pub fn list_files(&self) -> Result<ApiList<File>, OpenApiError> {
        self.networking.list_files()
    }

    pub fn retrieve_file<T: Identifiable>(&self, file: T) -> Result<File, OpenApiError> {
        let file_id: String = file.get_identifier();
        self.networking.retrieve_file(file_id)
    }

    pub fn delete_file<T: Identifiable>(&self, file: T) -> Result<DeletionStatus, OpenApiError> {
        let file_id: String = file.get_identifier();
        self.networking.delete_file(file_id)
    }

    pub fn retrieve_file_content<T: Identifiable>(&self, file: T) -> Result<String, OpenApiError> {
        let file_id: String = file.get_identifier();
        self.networking.retrieve_file_content(file_id)
    }

    /* MODERATION */
    pub fn create_moderation<T: Into<String>>(
        &self,
        text: T,
        moderation_model: ModerationType,
    ) -> Result<Moderation, OpenApiError> {
        let text = text.into();
        let moderation_model = strip_edges!(serde_json::to_string(&moderation_model)?);

        let mut payload = HashMap::new();
        payload.insert(String::from("input"), text);
        payload.insert(String::from("model"), moderation_model);

        self.networking.create_moderation(payload)
    }

    /* ASSISTANTS */
    pub fn retrieve_assistant<T: Identifiable>(
        &self,
        assistant: T,
    ) -> Result<Assistant, OpenApiError> {
        let assistant_id: String = assistant.get_identifier();
        self.networking.retrieve_assistant(assistant_id)
    }

    pub fn retrieve_assistant_file<A: Identifiable, F: Identifiable>(
        &self,
        assistant: A,
        file: F,
    ) -> Result<AssistantFile, OpenApiError> {
        let assistant_id: String = assistant.get_identifier();
        let file_id: String = file.get_identifier();
        self.networking
            .retrieve_assistant_file(assistant_id, file_id)
    }

    pub fn list_assistant_files<T: Identifiable>(
        &self,
        assistant: T,
    ) -> Result<ApiList<AssistantFile>, OpenApiError> {
        let assistant_id: String = assistant.get_identifier();
        self.networking.list_assistant_files(assistant_id)
    }

    pub fn list_assistants(&self) -> Result<ApiList<Assistant>, OpenApiError> {
        self.networking.list_assistants(None) //TODO Check params of list assistant
    }

    pub fn delete_assistant<T: Identifiable>(
        &self,
        assistant: T,
    ) -> Result<DeletionStatus, OpenApiError> {
        let assistant_id: String = assistant.get_identifier();
        self.networking.delete_assistant(assistant_id)
    }

    pub fn delete_assistant_file<A: Identifiable, F: Identifiable>(
        &self,
        assistant: A,
        file: F,
    ) -> Result<DeletionStatus, OpenApiError> {
        let assistant_id: String = assistant.get_identifier();
        let file_id: String = file.get_identifier();
        self.networking.delete_assistant_file(assistant_id, file_id)
    }

    /* THREADS */

    pub fn retrieve_thread<T: Identifiable>(&self, thread: T) -> Result<Thread, OpenApiError> {
        let thread_id: String = thread.get_identifier();
        self.networking.retrieve_thread(thread_id)
    }

    pub fn delete_thread<T: Identifiable>(
        &self,
        thread: T,
    ) -> Result<DeletionStatus, OpenApiError> {
        let thread_id: String = thread.get_identifier();
        self.networking.delete_thread(thread_id)
    }

    /* MESSAGES */
    pub fn list_message_files<T: Identifiable, M: Identifiable>(
        &self,
        thread: T,
        message: M,
    ) -> Result<ApiList<MessageFile>, OpenApiError> {
        let thread_id: String = thread.get_identifier();
        let message_id: String = message.get_identifier();
        self.networking.list_message_files(thread_id, message_id)
    }

    pub fn list_messages<T: Identifiable>(
        &self,
        thread: T,
    ) -> Result<ApiList<Message>, OpenApiError> {
        let thread_id: String = thread.get_identifier();
        self.networking.list_messages(thread_id)
    }

    pub fn retrieve_message<T: Identifiable, M: Identifiable>(
        &self,
        thread: T,
        message: M,
    ) -> Result<Message, OpenApiError> {
        let thread_id: String = thread.get_identifier();
        let message_id: String = message.get_identifier();
        self.networking.retrieve_message(thread_id, message_id)
    }

    pub fn retrieve_message_file<T: Identifiable, M: Identifiable, F: Identifiable>(
        &self,
        thread_id: T,
        message_id: M,
        file_id: F,
    ) -> Result<MessageFile, OpenApiError> {
        let thread_id = thread_id.get_identifier();
        let message_id = message_id.get_identifier();
        let file_id = file_id.get_identifier();
        self.networking
            .retrieve_message_file(thread_id, message_id, file_id)
    }

    /* RUNS */
    pub fn retrieve_run<T: Identifiable, R: Identifiable>(
        &self,
        thread: T,
        run: R,
    ) -> Result<Run, OpenApiError> {
        let thread_id: String = thread.get_identifier();
        let run_id: String = run.get_identifier();
        self.networking.retrieve_run(thread_id, run_id)
    }

    pub fn retrieve_run_step<T: Identifiable, R: Identifiable, S: Identifiable>(
        &self,
        thread_id: T,
        run_id: R,
        step_id: S,
    ) -> Result<RunStep, OpenApiError> {
        let thread_id: String = thread_id.get_identifier();
        let run_id: String = run_id.get_identifier();
        let step_id: String = step_id.get_identifier();
        self.networking
            .retrieve_run_step(thread_id, run_id, step_id)
    }

    pub fn cancel_run<T: Identifiable, R: Identifiable>(
        &self,
        thread_id: T,
        run_id: R,
    ) -> Result<Run, OpenApiError> {
        let thread_id: String = thread_id.get_identifier();
        let run_id: String = run_id.get_identifier();
        self.networking.cancel_run(thread_id, run_id)
    }
}

#[cfg(tests)]
mod tests {}
