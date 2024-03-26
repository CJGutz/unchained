use crate::error::{Error, WebResult};

use super::{
    text_parse::{find_between, Match, remove_between}, 
    context::{ContextMap, ContextTree as Ctx, Primitive::*},
    render::render_html
};

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
    let mut open_brackets = 0;
    for character in operation.chars() {
        if character == '{' {
            open_brackets += 1;
        } else if character == '}' {
            open_brackets -= 1;
        }
    }



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
        "for" => Some(for_loop_operation),
        "if" => Some(if_operation),
        _ => Some(get_attribute_from_context_operation),
    }
}

fn unwrap_n_params<'a, const N: usize>(params: &'a Vec<String>) -> WebResult<[&'a str; N]> {
    dbg!(&params);
    let mut arr = [""; N];
    dbg!(params.len(), N);
    if params.len() != N {
        return Err(Error::InvalidParams(format!("Expected {N} parameters, got {}", params.len())));
    }
    for (i, param) in params.iter().enumerate() {
        arr[i] = param.as_str();
    }
    Ok(arr)
}


fn get_attribute_from_context_operation(call: TemplateOperationCall, context: &ContextMap) -> WebResult<String> {
    let splitted = call.name.split(".");
    let mut resulting_attribute = None;
    let mut new_context = context.clone();
    for attribute in splitted {
        resulting_attribute = match new_context.get(attribute) {
            Some(Ctx::Leaf(s)) => Some(match s {
                Str(s) => s.to_string(),
                Num(n) => n.to_string(),
                Bool(b) => b.to_string(),
            }),
            Some(Ctx::Branch(b)) => {
                new_context = *b.clone();
                continue;
            },
            _ => return Err(Error::InvalidParams("Invalid".to_string())),
        };
    }
    if resulting_attribute.is_none() {
        return Err(Error::InvalidParams("Invalid property access".to_string()));
    }
    Ok(resulting_attribute.unwrap())
}


fn if_operation(call: TemplateOperationCall, context: &ContextMap) -> WebResult<String> {
    if let Ok([first_param]) = unwrap_n_params::<1>(&call.parameters) {
        let display_content = match first_param {
            "true" => true,
            "false" => false,
            _ => {
                match context.get(first_param) {
                    Some(Ctx::Leaf(Bool(bool))) => *bool,
                    _ => return Err(Error::InvalidParams("Invalid".to_string())),
                }
            }
        };
        if display_content {
            return Ok(call.children.unwrap_or(String::new()));
        } 
        return Ok(String::new());
    }
    Err(Error::InvalidParams("Invalid".to_string()))
}

fn for_loop_operation(call: TemplateOperationCall, context: &ContextMap) -> WebResult<String> {
    let param_slice = unwrap_n_params::<3>(&call.parameters);
    dbg!(&param_slice);
    let (element, range_key) = match param_slice {
        Ok([element, "in", range]) => (element, range),
        _ => return Err(Error::InvalidParams("Invalid".to_string()))
    };
    dbg!(&range_key);
    let range = match context.get(range_key) {
        Some(Ctx::Array(arr)) => arr,
        _ => return Err(Error::InvalidParams("Invalid".to_string())),
    };
    let children = match call.children {
        Some(children) => children,
        None => return Err(Error::InvalidParams("Invalid".to_string())),
    };
    let mut new_context = context.clone();
    let mut iterated_content = String::new();
    for item in range.iter() {
        dbg!(&item);
        new_context.insert(element.to_string(), (*item).clone());
        iterated_content.push_str(render_html(children.clone(), None).unwrap().as_str());
    }
    Ok(String::new())
}
