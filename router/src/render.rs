use std::collections::HashMap;

pub fn template(path: String, context: Option<HashMap<String, String>>) -> String {

    let content = std::fs::read_to_string(path).expect("Could not read file");
    return content;
}
