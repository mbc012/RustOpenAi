use crate::common::{ApiList, DeletionStatus};
use crate::error::OpenApiError;
use crate::file::{File, FileTypes};
use crate::networking::Networking;
use crate::strip_edges;
use reqwest::blocking::multipart;
use reqwest::Method;
use std::path::PathBuf;

pub trait FileActions {
    fn upload_file(&self, file: PathBuf, purpose: FileTypes) -> Result<File, OpenApiError>;
    fn list_files(&self) -> Result<ApiList<File>, OpenApiError>;
    fn retrieve_file(&self, file_id: String) -> Result<File, OpenApiError>;
    fn delete_file(&self, file_id: String) -> Result<DeletionStatus, OpenApiError>;
    fn retrieve_file_content(&self, file_id: String) -> Result<String, OpenApiError>;
}

impl FileActions for Networking {
    fn upload_file(&self, file: PathBuf, purpose: FileTypes) -> Result<File, OpenApiError> {
        let form = multipart::Form::new()
            .text("purpose", strip_edges!(serde_json::to_string(&purpose)?))
            .file("file", file.as_path())?;

        self.send_and_convert(Method::POST, String::from("files"), None, Some(form))
    }

    fn list_files(&self) -> Result<ApiList<File>, OpenApiError> {
        self.send_and_convert(Method::GET, String::from("files"), None, None)
    }

    fn retrieve_file(&self, file_id: String) -> Result<File, OpenApiError> {
        self.send_and_convert(Method::GET, format!("files/{}", file_id), None, None)
    }

    fn delete_file(&self, file_id: String) -> Result<DeletionStatus, OpenApiError> {
        self.send_and_convert(Method::DELETE, format!("files/{}", file_id), None, None)
    }

    //TODO Check the return type correct
    fn retrieve_file_content(&self, file_id: String) -> Result<String, OpenApiError> {
        self.send_and_convert(
            Method::GET,
            format!("files/{}/content", file_id),
            None,
            None,
        )
    }
}

impl FileActions for File {}
