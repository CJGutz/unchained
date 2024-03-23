use std::collections::HashMap;

pub fn template(path: String, context: Option<HashMap<String, String>>) -> String {

    let mut content = std::fs::read_to_string(path).expect("Could not read file");
    content = content.lines().map(|line| {
        let mut new_line = String::from(line);
        if new_line.contains("[- ") {
            if let Some(context) = &context {
                for (key, value) in context {
                    new_line = new_line.replace(&format!("[- {} -]", key), value); 
                }
            }
        }
        new_line
    }).collect::<Vec<String>>().join("\n");
    return content;
}
