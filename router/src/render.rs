use std::collections::HashMap;

pub struct Match {
    pub from: usize,
    pub to: usize,
    pub content: String,
}


/// Finds the first match of from and to in the content.
/// Example:
/// ```
/// let found = find_between("content that | contains patterns | but", "|", "|");
///
/// assert!(found.is_some());
/// let found = found.unwrap();
/// assert_eq!(found.content, " contains patterns ");
/// assert_eq!(found.from, 13);
/// assert_eq!(found.to, 33);
/// ```
fn find_between(content: &str, from: &str, to: &str) -> Option<Match> {
    let from_index = match content.find(from) {
        Some(index) => index,
        None => return None,
    }; 
    let after_from = &content[(from_index+from.len())..]; 
    let to_index = match after_from.find(to) {
        Some(index) => index,
        None => return None,
    };  
    let content_inside = after_from[..to_index].to_string(); 

    Some(Match {
        from: from_index,
        to: to_index + from_index + from.len() + to.len() - 1,
        content: content_inside
    })
}

/// Removes everything inclusively between the first occurrences
/// of `from` and `to` and returns it exclusive of the patterns.
/// Example:
/// ```
/// let content = String::from("A string with [a pattern] found");
/// let res = remove_between(content, "[", "]");
/// assert!(res.is_some());
/// let (changed_content, inside_pattern) = res.unwrap();
/// assert_eq!(changed_content, "A string with  found".to_string());
/// assert_eq!(inside_pattern, "a pattern".to_string());
/// ```
fn remove_between(content: &str, from: &str, to: &str) -> Option<(String, String)> {
    let find = find_between(content, from, to);
    if find.is_none() { return None }
    let find = find.unwrap();

    let mut content = content.to_string();
    content.replace_range(find.from..find.to+1, "");
    return Some((content, find.content));
}


pub struct TemplateOperationCall {
    pub name: String,
    pub parameters: Vec<String>,
    pub children: Option<String>,
}
fn childless_templ_op_call(op_content: &str) -> Option<TemplateOperationCall> {
    let splitted = op_content.split(" ").map(|s| s.to_string());
    let name = splitted.clone().take(1).collect::<String>();
    if name.is_empty() { return None }
    return Some(TemplateOperationCall {
        name,
        parameters: splitted.skip(1).collect::<Vec<String>>(),
        children: None,
    })
}
fn operation_params_and_children(operation: &str) -> Option<TemplateOperationCall> {
    if let Some((removed_children, children)) = remove_between(operation, "{", "}") {
        let op_call = childless_templ_op_call(&removed_children);
        return match op_call {
            None => None,
            Some(mut operation) => {
                operation.children = Some(children);
                return Some(operation);
            }
        };
    }
    return childless_templ_op_call(operation);
         
}


fn if_operation(call: TemplateOperationCall, context: &HashMap<String, String>) -> Result<String, ()> {
    if let Some(first_param) = call.parameters.first() {
        let display_content = match first_param.as_str() {
            "true" => true,
            "false" => false,
            _ => {
                let attribute = context.get(first_param);
                attribute.is_some() && attribute.unwrap().as_str() == "true" // TODO: Store context
                // with primitive values
            }
        };
        if display_content {
            return Ok(call.children.unwrap_or(String::new()));
        } 
        return Ok(String::new());
    }
    Err(())
}

type TemplateOperation = fn(TemplateOperationCall, &HashMap<String, String>) -> Result<String, ()>;
/// Example template operation
/// ```
/// {{ if boolean {
///  <div>Renders if true</div>
/// }
/// }}
/// ```
fn get_template_operation(op_name: &str) -> Option<TemplateOperation> {
    match op_name {
        "loop" => None,
        "if" => None,
        _ => None,
    }
}

/// Render an html file
/// Use template operations `{{ }}` to add 
/// functionality to html with given context
/// ```
/// if let Some(to_replace_with) = context.get(&m.content.trim().to_owned()) {
///     content.replace_range(m.from..m.to+1, to_replace_with); 
/// }
/// ```
pub fn template(path: &str, context: Option<HashMap<String, String>>) -> Result<String, ()> {

    let content = std::fs::read_to_string(path);
    if content.is_err() { return Err(()) }
    let mut content = content.unwrap();
    let context = &context.unwrap_or_default();

    loop {
        let result = find_between(&content, "{{", "}}");
        if result.is_none() { break; }
        let result = result.unwrap();
        if let Some(op_call) = operation_params_and_children(&result.content) {
            if let Some(operation) = get_template_operation(&op_call.name) {
                let replacement = operation(op_call, context);
                if replacement.is_err() { return Err(()) }
                content.replace_range(result.from..result.from+1, &replacement.unwrap())
            }
        }
    }
    return Ok(content);
}

#[cfg(test)]
mod tests {
    use crate::render::{find_between, remove_between};
    #[test]
    fn test_get_between_in_one_line_match_w_equal_patterns() {
        let found = find_between("content that | contains patterns |", "|", "|");
        assert!(found.is_some());
        assert_eq!(found.unwrap().content, " contains patterns ");
    }

    #[test]
    fn test_get_between_in_two_lines_match_w_equal_patterns() {
        let found = find_between("content that | contains patterns | but this |is \n also content to| get", "|", "|");

        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.content, " contains patterns ");
        assert_eq!(found.from, 13);
        assert_eq!(found.to, 33);
    }

    #[test]
    fn test_get_between_w_multi_char_pattern() {
        let found = find_between("content that || contains patterns || but this ||is \n also content to|| get", "||", "||");

        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.content, " contains patterns ");
        assert_eq!(found.from, 13);
        assert_eq!(found.to, 35);
    }

    #[test]
    fn test_assymmetric_pattern() { 
        let found = find_between("content that contains patterns but this {{is \n also content to}} get", "{{", "}}");

        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.content, "is \n also content to");
        assert_eq!(found.from, 40);
        assert_eq!(found.to, 63);
    }

    #[test]
    fn test_no_pattern_found() {
        let found = find_between("content with no pattern", "|", "}");

        assert!(found.is_none());
    }

    #[test]
    fn test_no_to_pattern_found() {
        let found = find_between("content with | no pattern", "|", "}");

        assert!(found.is_none());
    }

    #[test]
    fn test_remove_single_asymmetric_pattern() {
        let res = remove_between("A string with [a pattern] found", "[", "]");
        assert!(res.is_some());
        let (changed_content, inside_pattern) = res.unwrap();
        assert_eq!(changed_content, "A string with  found".to_string());
        assert_eq!(inside_pattern, "a pattern".to_string());
    }

}
