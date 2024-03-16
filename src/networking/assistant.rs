use crate::assistant::{Assistant, AssistantBuilder, AssistantFile, AssistantFileBuilder};
use crate::common::{ApiList, DeletionStatus};
use crate::error::OpenApiError;
use crate::networking::Networking;
use reqwest::Method;

pub trait AssistantActions {
    fn create_assistant(&self, payload: &AssistantBuilder) -> Result<Assistant, OpenApiError>;
    fn create_assistant_file(
        &self,
        payload: &AssistantFileBuilder,
        assistant_id: &String,
    ) -> Result<AssistantFile, OpenApiError>;
    fn list_assistants(&self, params: Option<&String>) -> Result<ApiList<Assistant>, OpenApiError>;
    fn list_assistant_files(
        &self,
        assistant_id: String,
    ) -> Result<ApiList<AssistantFile>, OpenApiError>;
    fn retrieve_assistant(&self, assistant_id: String) -> Result<Assistant, OpenApiError>;
    fn retrieve_assistant_file(
        &self,
        assistant_id: String,
        file_id: String,
    ) -> Result<AssistantFile, OpenApiError>;
    fn modify_assistant(
        &self,
        assistant_id: String,
        payload: &AssistantBuilder,
    ) -> Result<Assistant, OpenApiError>;
    fn delete_assistant(&self, assistant_id: String) -> Result<DeletionStatus, OpenApiError>;
    fn delete_assistant_file(
        &self,
        assistant_id: String,
        file_id: String,
    ) -> Result<DeletionStatus, OpenApiError>;
}

impl AssistantActions for Networking {
    fn create_assistant(&self, payload: &AssistantBuilder) -> Result<Assistant, OpenApiError> {
        self.send_and_convert(
            Method::POST,
            String::from("assistants"),
            Some(serde_json::to_value(payload)?),
            None,
        )
    }

    fn create_assistant_file(
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

    fn list_assistants(&self, params: Option<&String>) -> Result<ApiList<Assistant>, OpenApiError> {
        // Add param support
        self.send_and_convert(Method::GET, String::from("assistants"), None, None)
    }

    fn list_assistant_files(
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

    fn retrieve_assistant(&self, assistant_id: String) -> Result<Assistant, OpenApiError> {
        self.send_and_convert(
            Method::GET,
            format!("assistants/{}", assistant_id),
            None,
            None,
        )
    }

    fn retrieve_assistant_file(
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

    fn modify_assistant(
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

    fn delete_assistant(&self, assistant_id: String) -> Result<DeletionStatus, OpenApiError> {
        self.send_and_convert(
            Method::DELETE,
            format!("assistants/{}", assistant_id),
            None,
            None,
        )
    }

    fn delete_assistant_file(
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
}
