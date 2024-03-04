mod client;
pub use client::OpenAIClient;

mod macros;
mod networking;
pub mod types;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4); //TODO
    }
}
