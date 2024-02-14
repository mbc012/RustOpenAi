use serde_json;

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

// TODO: Check working
#[macro_export]
macro_rules! see_json {
    ($x:expr) => {
        match serde_json::to_string_pretty(&$x) {
            Ok(json) => println!("{}", json),
            Err(e) => println!("Error serializing to JSON: {}", e),
        }
    };
}
