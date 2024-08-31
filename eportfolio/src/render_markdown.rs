use std::collections::HashMap;

use markdown::Options;
use unchained::{error::WebResult, templates::{context::ContextMap, operations::{unwrap_n_params, TemplateOperation}, render::{load_template, RenderOptions}}};


pub fn render_md(path: &str, context: Option<ContextMap>) -> WebResult<String> {
   let closure = {
      |call, ctx, opts| {
         let file_path = unwrap_n_params::<1>(&call.parameters)?[0];
         let file_content = load_template(file_path, Some(ctx.clone()), opts)?;
         let md = markdown::to_html_with_options(
            &file_content,
            &Options::gfm()
         );
         Ok(md.unwrap_or_default())
      }
   } as TemplateOperation;

   load_template(path, context.clone(), 
      &RenderOptions {
         custom_operations: HashMap::from([
            ("md", closure),
         ])
      }
   ) 
}

pub fn md_operation() {

}
