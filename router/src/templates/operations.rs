use crate::{templates::text_parse::{find_between, Match, remove_between}, error::{Error, WebResult}};

use super::context::{ContextMap, ContextTree as Ctx, Primitive::*};

pub fn template_operation(content: &str) -> Option<Match> {
    find_between(content, "{*", "*}")
}


#[derive(Debug)]
pub struct TemplateOperationCall {
    pub name: String,
    pub parameters: Vec<String>,
    pub children: Option<String>,
}

fn childless_templ_op_call(op_content: &str) -> Option<TemplateOperationCall> {
    let splitted = op_content.trim().split(" ").map(|s| s.to_string());
    let name = splitted.clone().take(1).collect::<String>();
    if name.is_empty() { return None }
    return Some(TemplateOperationCall {
        name,
        parameters: splitted.skip(1).collect::<Vec<String>>(),
        children: None,
    })
}

pub fn operation_params_and_children(operation: &str) -> Option<TemplateOperationCall> {
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


fn if_operation(call: TemplateOperationCall, context: &ContextMap) -> WebResult<String> {
    if let Some(first_param) = call.parameters.first() {
        let display_content = match first_param.as_str() {
            "true" => true,
            "false" => false,
            _ => {
                match context.get(first_param) {
                    Some(Ctx::Leaf(Bool(bool))) => *bool,
                    _ => return Err(Error::InvalidParams),
                }
            }
        };
        if display_content {
            return Ok(call.children.unwrap_or(String::new()));
        } 
        return Ok(String::new());
    }
    Err(Error::InvalidParams)
}

type TemplateOperation = fn(TemplateOperationCall, &ContextMap) -> WebResult<String>;
/// Example template operation
/// ```
/// {{ if boolean {
///  <div>Renders if true</div>
/// }
/// }}
/// ```
pub fn get_template_operation(op_name: &str) -> Option<TemplateOperation> {
    match op_name {
        "loop" => None,
        "if" => Some(if_operation),
        _ => None,
    }
}
