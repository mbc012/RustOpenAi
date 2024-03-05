use crate::networking::Networking;
use crate::types::common::Identifiable;
use crate::types::error::OpenApiError;

use crate::{impl_ref, strip_edges};
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
        self.id.clone()
    }
}
impl_ref!(File, Identifiable);

#[derive(Serialize, Deserialize, Clone)]
pub struct FileBuilder {
    file: PathBuf,
    purpose: FileTypes,
}

impl FileBuilder {
    pub fn new<P: Into<PathBuf>>(file: P, purpose: FileTypes) -> Self {
        let file = file.into();
        Self { file, purpose }
    }
    pub fn build(self, networking: &Networking) -> Result<File, OpenApiError> {
        networking.upload_file(self.file, self.purpose)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum FileTypes {
    FineTune,
    Assistants,
}
