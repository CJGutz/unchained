use std::collections::HashMap;

use crate::error::{Error, WebResult};

use super::{
    context::{ctx_str, ContextMap, ContextTree as Ctx, Primitive::*},
    render::{render_html, template},
    text_parse::{between_connected_patterns, Match},
};

const INSIDE_COMPONENT_OP_ID: &str = "inside_component_operation_identifier";

pub fn template_operation(content: &str) -> Option<Match> {
    between_connected_patterns(content, "{*", "*}")
}

#[derive(Debug)]
pub struct TemplateOperationCall {
    pub name: String,
    pub parameters: Vec<String>,
    pub children: Option<String>,
}

fn childless_templ_op_call(op_content: &str) -> Option<TemplateOperationCall> {
    let splitted = op_content.trim().split(' ').map(|s| s.to_string());
    let name = splitted.clone().take(1).collect::<String>();
    if name.is_empty() {
        return None;
    }
    Some(TemplateOperationCall {
        name,
        parameters: splitted.skip(1).collect::<Vec<String>>(),
        children: None,
    })
}

pub fn operation_params_and_children(operation: &str) -> Option<TemplateOperationCall> {
    let find = between_connected_patterns(operation, "{", "}");

    if let Some(find) = find {
        let mut operation = operation.to_string();
        operation.replace_range(find.from..find.to + 1, "");

        let op_call = childless_templ_op_call(&operation);
        return match op_call {
            None => None,
            Some(mut operation) => {
                operation.children = Some(find.content);
                return Some(operation);
            }
        };
    }
    childless_templ_op_call(operation)
}

pub type TemplateOperation = fn(TemplateOperationCall, &ContextMap) -> WebResult<String>;
/// Example template operation
/// ```
/// {* if boolean {
///  <div>Renders if true</div>
/// }
/// *}
/// ```
pub fn get_template_operation(op_name: &str, custom_operations: HashMap<&str, TemplateOperation>) -> Option<TemplateOperation> {
    match op_name {
        "get" => Some(attribute_operation),
        "for" => Some(for_loop_operation),
        "if" => Some(if_operation),
        "component" => Some(component_operation),
        "slot" => Some(slot),
        s => custom_operations.get(s).copied(),
    }
}

fn unwrap_n_params<const N: usize>(params: &Vec<String>) -> WebResult<[&str; N]> {
    let mut arr = [""; N];
    if params.len() != N {
        return Err(Error::InvalidParams(format!(
            "Expected {N} parameters, got {}",
            params.len()
        )));
    }
    for (i, param) in params.iter().enumerate() {
        arr[i] = param.as_str();
    }
    Ok(arr)
}

fn attribute_from_context(attribute: &str, context: &ContextMap) -> WebResult<String> {
    let splitted = attribute.split('.');
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
            }
            _ => {
                return Err(Error::InvalidParams(format!(
                    "Invalid attribute: {} not found in context",
                    attribute
                )))
            }
        };
    }
    if resulting_attribute.is_none() {
        return Err(Error::InvalidParams("Invalid property access".to_string()));
    }
    Ok(resulting_attribute.unwrap())
}

fn attribute_operation(call: TemplateOperationCall, context: &ContextMap) -> WebResult<String> {
    let attribute = unwrap_n_params::<1>(&call.parameters)?[0];
    attribute_from_context(attribute, context)
}

fn if_operation(call: TemplateOperationCall, context: &ContextMap) -> WebResult<String> {
    let first_param = unwrap_n_params::<1>(&call.parameters)?[0];
    let display_content = match first_param {
        "true" => true,
        "false" => false,
        _ => match context.get(first_param) {
            Some(Ctx::Leaf(Bool(bool))) => *bool,
            res => {
                return Err(Error::InvalidParams(format!(
                    "Expected value resolve to boolean, but found {:?}",
                    res.unwrap_or(&ctx_str(&format!(
                        "(no value for '{}' in context)",
                        first_param
                    )))
                )))
            }
        },
    };
    if display_content {
        return Ok(call.children.unwrap_or(String::new()));
    }
    Ok(String::new())
}

fn for_loop_operation(call: TemplateOperationCall, context: &ContextMap) -> WebResult<String> {
    let param_slice = unwrap_n_params::<3>(&call.parameters)?;
    let (element, range_key) = match param_slice {
        [element, "in", range] => (element, range),
        _ => {
            return Err(Error::InvalidParams(format!(
                "Expected 'for element in range', but got '{:?}'",
                param_slice
            )))
        }
    };
    let range = match context.get(range_key) {
        Some(Ctx::Array(arr)) => arr,
        None => {
            return Err(Error::InvalidParams(format!(
                "Found no context value for {range_key}"
            )))
        }
        Some(res) => {
            return Err(Error::InvalidParams(format!(
                "Range has to be a context array. Got {:?}",
                res
            )))
        }
    };
    let children = match call.children {
        Some(children) => children,
        None => {
            return Err(Error::InvalidParams(
                "A for loop needs to have children to iterate.".to_string(),
            ))
        }
    };
    let mut new_context = context.clone();
    let mut iterated_content = String::new();
    for item in range.iter() {
        new_context.insert(element.to_string(), (*item).clone());
        iterated_content.push_str(
            render_html(children.clone(), Some(new_context.to_owned()))
                .unwrap()
                .as_str(),
        );
    }
    Ok(iterated_content)
}

/// Loads an html file to include in a template.
/// Use slots to add html to specific parts of the template.
/// If there are no slots, the component children, if any,
/// will use the default slot.
///
/// Context can be given to the component. If it is,
/// other context data is removed.
///
/// TODO: Update the render method to insert own operations
/// ```
/// <!-- page.html -->
/// {* component file.html data=object.attribute {
///     {* slot default {
///        <div>Default content</div>
///     } *}
///     {* slot something_else {
///        <div>Default content</div>
///     } *}
/// } *}
/// <!-- my_component.html -->
/// <div>
///     {* slot default *}
/// </div>
/// ```
///
fn component_operation(call: TemplateOperationCall, context: &ContextMap) -> WebResult<String> {
    let parameters = call.parameters;
    let file_path = parameters.get(0);
    let file_path = match file_path {
        Some(file_path) => file_path,
        None => return Err(Error::InvalidParams("File path not specified".to_string())),
    };
    if !file_path.ends_with(".html") {
        return Err(Error::InvalidParams("Invalid file path".to_string()));
    }
    let mut new_context = context.clone();
    if parameters.len() > 1 {
        new_context.clear();
        parameters
            .iter()
            .skip(1)
            .map(|p| p.split('=').collect::<Vec<_>>())
            .for_each(|p| {
                if p.len() != 2 {
                    return;
                }
                let item = attribute_from_context(p[1], context);
                new_context.insert(p[0].to_string(), Ctx::Leaf(Str(item.unwrap())));
            });
    }

    if let Some(children) = call.children {
        new_context.insert(INSIDE_COMPONENT_OP_ID.to_string(), Ctx::Leaf(Bool(true)));
        let mut content_w_slots = children.clone();
        let mut slot_operations = 0;
        while let Some(op) = template_operation(&content_w_slots) {
            if let Some(operation_call) = operation_params_and_children(&op.content) {
                if &operation_call.name == "slot" {
                    let slot_name = unwrap_n_params::<1>(&operation_call.parameters)?[0];
                    new_context.insert(
                        slot_name.to_string(),
                        Ctx::Slot(Str(operation_call.children.unwrap_or(String::new()))),
                    );
                    slot_operations += 1;
                }
            }
            content_w_slots.replace_range(op.from..op.to, "");
        }
        if slot_operations == 0 {
            new_context.insert("default".to_string(), Ctx::Slot(Str(children.to_string())));
        }
    }

    let rendered = template(file_path, Some(new_context))?;

    Ok(rendered)
}

/// Retrives html to include in context
/// This slot operation is handled when rendering
/// a component. A component operation includes the
/// slot html in the context. If a component excludes
/// a slot. It renderes nothing.
/// ```
/// <!-- page.html -->
/// {* component file.html {
///     {* slot default {
///        <div>Default content</div>
///     } *}
///     {* slot something_else {
///        <div>Default content</div>
///     } *}
/// } *}
/// <!-- my_component.html -->
/// <div>
///     {* slot default *}
/// </div>
/// ```
///
fn slot(call: TemplateOperationCall, context: &ContextMap) -> WebResult<String> {
    let slot_name = unwrap_n_params::<1>(&call.parameters)?[0];
    if let Some(Ctx::Leaf(Bool(inside_component))) = context.get(INSIDE_COMPONENT_OP_ID) {
        if !inside_component {
            return Err(Error::InvalidParams(
                "Slot function is not loaded from component".to_string(),
            ));
        }
    }
    let content_to_include = match context.get(slot_name) {
        Some(Ctx::Slot(a)) => a.to_string(),
        _ => String::new(),
    };
    Ok(content_to_include)
}
