use serde::{Deserialize, Serialize};

use crate::openai::types::Identifiable;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Model {
    id: String,
    created: i64,
    object: String,
    owned_by: String,
}

impl Model {
    pub fn new(id: String, object: Option<String>, owned_by: Option<String>) -> Self {
        Self {
            id,
            created: 0,
            object: object.unwrap_or(String::from("model")),
            owned_by: owned_by.unwrap_or(String::from("system")),
        }
    }
}

impl Identifiable for Model {
    fn get_identifier(&self) -> String {
        self.id.clone()
    }
}
