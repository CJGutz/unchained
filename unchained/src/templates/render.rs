use std::collections::HashMap;

use crate::error::{Error, WebResult};

use super::{
    context::ContextMap,
    operations::{get_template_operation, operation_params_and_children, template_operation, TemplateOperation},
};

#[derive(Clone)]
pub struct RenderOptions<'a> {
    pub custom_operations: HashMap<&'a str, TemplateOperation>
}

impl RenderOptions<'_> {
    pub fn empty() -> Self {
        RenderOptions {
            custom_operations: HashMap::new()
        }
    }
}

/// Turn string with template operations into html
/// Context gives template operations access to data
pub fn render_html(mut content: String, context: Option<ContextMap>, options: &RenderOptions) -> WebResult<String> {
    let context = &context.unwrap_or_default();
    let mut min_look_index = 0;

    loop {
        let result = template_operation(&content[min_look_index..]);
        if result.is_none() {
            break;
        }
        let mut result = result.unwrap();
        result.push(min_look_index);
        min_look_index = result.from;

        if let Some(op_call) = operation_params_and_children(&result.content) {
            if let Some(operation) = get_template_operation(&op_call.name, options.custom_operations.clone()) {
                let replacement = operation(op_call, context, options)?;
                content.replace_range(result.from..=result.to, &replacement)
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

/// Render an html file from file
/// Use template operations `{* *}` to add
/// functionality to html with given context
pub fn load_template(path: &str, context: Option<ContextMap>, options: &RenderOptions) -> WebResult<String> {
    let content = std::fs::read_to_string(path);
    if content.is_err() {
        return Err(Error::LoadFile(format!("Could not read file {}", path)));
    }

    render_html(content.unwrap(), context, options)
}
