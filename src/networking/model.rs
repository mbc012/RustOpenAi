use crate::common::ApiList;
use crate::error::OpenApiError;
use crate::model::Model;
use crate::networking::Networking;
use reqwest::Method;

pub trait ModelActions {
    fn list_models(&self) -> Result<ApiList<Model>, OpenApiError>;
    fn load_model(&self, model_id: String) -> Result<Model, OpenApiError>;
}

impl ModelActions for Networking {
    fn list_models(&self) -> Result<ApiList<Model>, OpenApiError> {
        self.send_and_convert(Method::GET, String::from("models"), None, None)
    }

    fn load_model(&self, model_id: String) -> Result<Model, OpenApiError> {
        self.send_and_convert(Method::GET, format!("models/{}", model_id), None, None)
    }
}
