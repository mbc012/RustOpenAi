use serde_json;

/// Simplifies the implementation of a trait for a type in the case where the reference
/// implementation is the same as the owned implementation.
#[macro_export]
macro_rules! impl_ref {
    ($type:ty, $trait:ident) => {
        impl<'a> $trait for &'a $type {
            fn get_identifier(&self) -> String {
                (*self).get_identifier()
            }
        }
    };
}

/// Removes the first and last character from a string, useful for serializing enums with serde_json::to_string
#[macro_export]
macro_rules! strip_edges {
    ($s:expr) => {{
        let temp_str = $s.to_string();
        if temp_str.len() > 1 {
            temp_str[1..temp_str.len() - 1].to_string()
        } else {
            panic!("strip_edges macro used on string with length <= 1")
        }
    }};
}

/// Quick macro to pretty print a struct that impls serde Serialize
#[macro_export]
macro_rules! see_json {
    ($x:expr) => {
        match serde_json::to_string_pretty(&$x) {
            Ok(json) => println!("{}", json),
            Err(e) => println!("Error serializing to JSON: {}", e),
        }
    };
}
