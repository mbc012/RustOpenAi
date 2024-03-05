use crate::impl_ref;
use crate::networking::Networking;
use serde::{Deserialize, Serialize};

use crate::types::common::Identifiable;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Model {
    id: String,
    created: i64,
    object: String,
    owned_by: String,
}

impl Identifiable for Model {
    fn get_identifier(&self) -> String {
        self.id.clone()
    }
}
impl_ref!(Model, Identifiable);
