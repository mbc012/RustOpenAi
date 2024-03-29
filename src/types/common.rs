use crate::impl_ref;
use serde::{Deserialize, Serialize};

/// `Identifiable` is a trait that provides a method for retrieving the identifier field of an object.
///
/// This trait is designed to simplify the usage of the library by allowing the user to use either `String`,
/// `&String`, `&str`, or any other type that implements the `Identifiable` trait as the identifier for the object.
/// This reduces the need for type conversions.
///
/// For custom types used in the library, `Identifiable` is implemented to return the `id` field of the
/// object as a string (for both owned and reference values). This allows for easy access to the identifier of custom objects.
///
/// # Methods
///
/// `get_identifier(&self) -> String`
///
/// Returns a string representing the identifier of the object. For `String`, `&String` and `&str`, it returns the string itself.
/// For other types, it returns the `id` field of the object as a string.
///
pub trait Identifiable {
    /// Returns a string representing the identifier of the object.
    fn get_identifier(&self) -> String;
}

impl Identifiable for String {
    fn get_identifier(&self) -> String {
        self.clone()
    }
}
impl_ref!(String, Identifiable);

impl Identifiable for &str {
    fn get_identifier(&self) -> String {
        self.to_string()
    }
}

/// ApiList is a generic struct used to represent a list of objects returned from the OpenAI API.
///
/// It can be expressed in different forms with the parameters of first_id, last_id, and has_more
/// being optional.
///
/// See examples:
/// [Optional Format](https://platform.openai.com/docs/api-reference/assistants/listAssistantFiles),
/// [Full Format](https://platform.openai.com/docs/api-reference/models/list)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiList<T> {
    object: String,
    data: Vec<T>,
    first_id: Option<String>,
    last_id: Option<String>,
    has_more: Option<bool>,
}

impl<T> ApiList<T> {
    /// Returns a reference to the data field of the ApiList.
    pub fn get_data_vec(&self) -> &Vec<T> {
        return &self.data;
    }
}

// TODO: ADD DOCUMENTATION FOR BELOW

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Usage {
    completion_tokens: u32,
    prompt_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tools {
    r#type: ToolTypes,
    #[serde(skip_serializing_if = "Option::is_none")]
    function: Option<ToolFunction>,
}

impl Tools {
    pub fn code_interpreter() -> Self {
        Self {
            r#type: ToolTypes::CodeInterpreter,
            function: None,
        }
    }

    pub fn retrieval() -> Self {
        Self {
            r#type: ToolTypes::Retrieval,
            function: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ToolTypes {
    CodeInterpreter,
    Retrieval,
    Function,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolFunction {
    description: String,
    name: String,
    // TODO: Implement parameters into ToolFunctions
    //#[serde(skip_serializing_if = "Option::is_none")]
    //parameters: Option<>
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct ToolCalls {
    index: Option<u32>,
    id: String,
    r#type: String,
    function: ToolCallsFunction,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct ToolCallsFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletionStatus {
    id: String,
    object: String,
    deleted: bool,
}
