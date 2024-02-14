use std::env;

use crate::networking::Networking;
use crate::types::assistant::{Assistant, AssistantFile};
use crate::types::common::{ApiList, DeletionStatus, Identifiable};
use crate::types::error::OpenApiError;
use crate::types::message::Message;
use crate::types::model::Model;
use crate::types::run::Run;
use crate::types::thread::Thread;

pub struct OpenAI {
    apikey: String,
    organization_id: Option<String>,
    networking: Networking,
}

impl OpenAI {
    fn obtain_key() -> String {
        let key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
            let mut key_buff = String::new();
            println!("Please enter your OpenAI API key: ");
            std::io::stdin().read_line(&mut key_buff).unwrap();
            env::set_var("OPENAI_API_KEY", key_buff.trim());
            key_buff
        });
        key
    }

    pub fn new(organization_id: Option<String>) -> Self {
        let apikey = OpenAI::obtain_key();
        Self {
            apikey: apikey.clone(),
            organization_id: organization_id.clone(),
            networking: Networking::new(apikey, organization_id.clone()),
        }
    }

    pub fn get_networking(&self) -> &Networking {
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

    /* ASSISTANTS */
    pub fn retrieve_assistant<T: Identifiable>(
        &self,
        assistant: T,
    ) -> Result<Assistant, OpenApiError> {
        let assistant_id: String = assistant.get_identifier();
        self.networking.retrieve_assistant(assistant_id)
    }

    pub fn retrieve_assistant_file<T: Identifiable>(
        &self,
        assistant: T,
        file: T,
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

    pub fn delete_assistant_file<T: Identifiable>(
        &self,
        assistant: T,
        file: T,
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
    // TODO: Add message file support

    pub fn list_messages<T: Identifiable>(
        &self,
        thread: T,
    ) -> Result<ApiList<Message>, OpenApiError> {
        let thread_id: String = thread.get_identifier();
        self.networking.list_messages(thread_id)
    }

    pub fn retrieve_message<T: Identifiable>(
        &self,
        thread: T,
        message: T,
    ) -> Result<Message, OpenApiError> {
        let thread_id: String = thread.get_identifier();
        let message_id: String = message.get_identifier();
        self.networking.retrieve_message(thread_id, message_id)
    }

    /* RUNS */
    pub fn retrieve_run<T: Identifiable>(&self, thread: T, run: T) -> Result<Run, OpenApiError> {
        let thread_id: String = thread.get_identifier();
        let run_id: String = run.get_identifier();
        self.networking.retrieve_run(thread_id, run_id)
    }
}

#[cfg(tests)]
mod tests {}
