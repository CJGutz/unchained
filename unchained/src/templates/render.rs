use std::collections::HashMap;

use crate::error::{Error, WebResult};

use super::{
    context::ContextMap,
    operations::{get_template_operation, operation_params_and_children, template_operation},
};

pub fn render_html(mut content: String, context: Option<ContextMap>) -> WebResult<String> {
    let context = &context.unwrap_or_default();

    loop {
        let result = template_operation(&content);
        if result.is_none() {
            break;
        }
        let result = result.unwrap();
        if let Some(op_call) = operation_params_and_children(&result.content) {
            if let Some(operation) = get_template_operation(&op_call.name, HashMap::new()) {
                let replacement = operation(op_call, context)?;
                content.replace_range(result.from..result.to + 1, &replacement)
            } else {
                return Err(Error::ParseTemplate(format!(
                    "No template operation specified for {}",
                    op_call.name
                )));
            }
        } else {
            return Err(Error::ParseTemplate(
                "Could not create operation from content".to_string(),
            ));
        }
    }
    Ok(content)
}

/// Render an html file
/// Use template operations `{* *}` to add
/// functionality to html with given context
pub fn template(path: &str, context: Option<ContextMap>) -> WebResult<String> {
    let content = std::fs::read_to_string(path);
    if content.is_err() {
        return Err(Error::LoadFile(format!("Could not read file {}", path)));
    }

    render_html(content.unwrap(), context)
}
