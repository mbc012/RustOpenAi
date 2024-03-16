//! Library for interfacing with the OpenAI API.
//!

///
/// lib.rs is used to re-export the contents of this crate, in a format that is easy to import.
///
/// Sections
/// - root - Client object used for interacting with the OpenAI API
/// - builders - Builder objects used for creating new objects
/// -
///
mod client;
pub use client::OpenAIClient;

mod macros;
mod networking;
mod types;

pub use types::assistant;
pub use types::chat;
pub use types::common;
pub use types::error;
pub use types::file;
pub use types::message;
pub use types::model;
pub use types::moderation;
pub use types::run;
pub use types::thread;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4); //TODO
    }
}
