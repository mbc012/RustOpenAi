use crate::networking::Networking;
use crate::types::common::Identifiable;
use crate::types::error::OpenApiError;

use crate::strip_edges;
use serde::{Deserialize, Serialize};
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
