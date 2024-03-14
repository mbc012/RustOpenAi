use rust_open_ai::assistant::AssistantBuilder;
use rust_open_ai::file::{FileBuilder, FileTypes};
use rust_open_ai::message::{MessageBuilder, MessageRole};
use rust_open_ai::moderation::ModerationType;
use rust_open_ai::run::RunBuilder;
use rust_open_ai::thread::ThreadBuilder;
use rust_open_ai::OpenAIClient;

const APIKEY: &str = "sk-AAA";

fn main() {
    // Client
    let organization_id = None;
    let client = OpenAIClient::new(APIKEY, organization_id).unwrap();

    // Model
    let list_models = client.list_models().unwrap();
    let load_model = client.load_model("gpt-4").unwrap();

    // File
    let listed_files = client.list_files().unwrap();
    let uploaded_file = FileBuilder::new("src/bin/upload_example.pdf", FileTypes::Assistants)
        .build(client.netref())
        .unwrap();
    let get_file = client.retrieve_file(&uploaded_file).unwrap();
    //let get_content = client.retrieve_file_content(&uploaded_file).unwrap();
    let del_file = client.delete_file(&uploaded_file).unwrap();

    // Moderation
    let moderation = client
        .create_moderation("message content", ModerationType::Latest)
        .unwrap();

    // Assistant
    let assistants = client.list_assistants().unwrap();
    let new_assistant = AssistantBuilder::new(&load_model)
        .build(client.netref())
        .unwrap();
    let get_assistant = client.retrieve_assistant(&new_assistant).unwrap();
    let assistant_files = client.list_assistant_files(&get_assistant).unwrap();
    let del_assistant = client.delete_assistant(&get_assistant).unwrap();

    // Thread
    let new_thread = ThreadBuilder::new().build(client.netref()).unwrap();
    let get_thread = client.retrieve_thread(&new_thread).unwrap();
    // modify thread
    let del_thread = client.delete_thread(&new_thread).unwrap();

    // Message
    let msg_thread = ThreadBuilder::new().build(client.netref()).unwrap();
    let messages = client.list_messages(&msg_thread).unwrap();
    let new_msg = MessageBuilder::new(&msg_thread, "Insert message here...".to_string())
        .unwrap()
        .build(client.netref())
        .unwrap();
    let get_msg = client.retrieve_message(&msg_thread, &new_msg).unwrap();
    let msg_files = client.list_message_files(&msg_thread, &get_msg).unwrap();
    //let msg_file = client.retrieve_message_file(&msg_thread, &get_msg, "file_id").unwrap();
    //modify message

    // Run
    //let new_run = RunBuilder::new()
    //let new_rt = RunBuilder::new_with_thread()
}
