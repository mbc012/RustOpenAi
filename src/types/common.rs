use serde::{Deserialize, Serialize};

/// Identifiable is a trait used to get the identifier of an object.
///
/// This is implemented for ease of use of the library, it is implemented into the String and &str
/// types. Where it will return the string itself, this allows for the user to use either String,
/// &str, or any other type that implements the Identifiable trait as the identifier for the object.
/// Reducing the need to convert between types.
///
/// For the other custom types used in the library, it is implemented to return the id field of the
/// object as a string.
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
/// [Optional Format](https://platform.openai.com/docs/api-reference/assistants/listAssistantFiles)
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
