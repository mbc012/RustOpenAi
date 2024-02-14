use crate::openai::macros;
use crate::openai::networking::Networking;
use crate::openai::types::{Identifiable, OpenApiError};
use crate::strip_edges;
use reqwest::blocking::multipart;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct File {
    id: String,
    bytes: u64,
    created_at: i64,
    filename: String,
    object: String,
    purpose: String,
}

impl Identifiable for File {
    fn get_identifier(&self) -> String {
        self.id.clone().to_string()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileBuilder {
    file: PathBuf,
    purpose: Option<FileTypes>,
}

impl FileBuilder {
    pub fn new(file: PathBuf) -> Self {
        Self {
            file,
            purpose: None,
        }
    }

    pub fn with_purpose(mut self, purpose: FileTypes) -> Self {
        self.purpose = Some(purpose);
        self
    }

    pub fn build(&self, networking: &Networking) -> Result<File, OpenApiError> {
        if (self.file == None) && (self.purpose == None) {
            return Err(OpenApiError::new(
                "File and purpose must be set".to_string(),
            ));
        }
        networking.upload_file(self)
    }

    pub fn purpose_str(&self) -> String {
        strip_edges!(serde_json::to_string(&self.purpose).unwrap()).to_string()
    }

    pub fn get_path_buff(&self) -> PathBuf {
        self.file.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FileTypes {
    FineTune,
    Assistants,
}
