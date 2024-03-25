use crate::error::{WebResult, Error};

use super::{
    context::ContextMap,
    operations::{template_operation, get_template_operation, operation_params_and_children}
};

pub fn render_html(mut content: String, context: Option<ContextMap>) -> WebResult<String> {
    let context = &context.unwrap_or_default();

    loop {
        let result = template_operation(&content);
        if result.is_none() { break; }
        let result = result.unwrap();
        if let Some(op_call) = operation_params_and_children(&result.content) {
            if let Some(operation) = get_template_operation(&op_call.name) {
                let replacement = operation(op_call, context);
                if replacement.is_err() { return Err(replacement.unwrap_err()) }
                content.replace_range(result.from..result.to+1, &replacement.unwrap())
            } else {
                return Err(Error::ParseTemplate)
            }
        } else {
            return Err(Error::ParseTemplate)
        }
    }
    return Ok(content);
}


/// Render an html file
/// Use template operations `{* *}` to add 
/// functionality to html with given context
pub fn template(path: &str, context: Option<ContextMap>) -> WebResult<String> {

    let content = std::fs::read_to_string(path);
    if content.is_err() { return Err(Error::LoadFile) }

    render_html(content.unwrap(), context)
}

