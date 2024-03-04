use RustOpenAi::types::assistant::AssistantBuilder;
use RustOpenAi::types::message::{MessageBuilder, MessageRole};
use RustOpenAi::types::model::Model;
use RustOpenAi::types::run::RunBuilder;
use RustOpenAi::types::thread::ThreadBuilder;
use RustOpenAi::OpenAIClient;

const APIKEY: &str = "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";

fn main() {
    let organization_id = None;

    let client = OpenAIClient::new(APIKEY.into(), organization_id).unwrap();

    let models = client.list_models().unwrap();
    let model = client.load_model("gpt-4").unwrap();

    let assistants = client.list_assistants().unwrap();
    let new_assistant = AssistantBuilder::new(model).build(client.netref()).unwrap();
    let get_assistant = client.retrieve_assistant(&new_assistant).unwrap();
    let assistant_files = client.list_assistant_files(&get_assistant).unwrap();
    let del_assistant = client.delete_assistant(&get_assistant).unwrap();

    let new_thread = ThreadBuilder::new().build(client.netref()).unwrap();
    let get_thread = client.retrieve_thread(&new_thread).unwrap();
    // modify thread
    let del_thread = client.delete_thread(&new_thread).unwrap();

    let msg_thread = ThreadBuilder::new().build(client.netref()).unwrap();
    let messages = client.list_messages(&msg_thread).unwrap();
    let new_msg = MessageBuilder::new(
        &msg_thread,
        MessageRole::User,
        "Insert message here...".to_string(),
    )
    .unwrap()
    .build(client.netref())
    .unwrap();
    let get_msg = client.retrieve_message(&msg_thread, &new_msg).unwrap();
    let msg_files = client.list_message_files(&msg_thread, &get_msg).unwrap();
    let msg_file = client.retrieve_message_file(&msg_thread, &get_msg, "file_id");
    //.unwrap();
    // modify message

    //let new_run = RunBuilder::new()
    //let new_rt = RunBuilder::new_with_thread()
}
