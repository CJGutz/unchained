use std::collections::HashMap;

use router::{
    run::{start_server, ServerOptions, Route, HTTPVerb::*, Response},
    templates::{
        render::template,
        context::{ContextTree as Ctx, Primitive::*},
    }, error::Error
};

fn main() {

    let routes = vec![
        Route::new(GET, String::from("/hello/"),   |_req| {
            let mut context = HashMap::new();
            context.insert("list".to_string(), Ctx::Array(Box::new(vec![
                Ctx::Leaf(Str("Hello".to_string())),
                Ctx::Leaf(Str("World".to_string())),
            ])));
            context.insert("title".to_string(), Ctx::Branch(Box::new(HashMap::from([
                ("title".to_string(), Ctx::Leaf(Str(String::from("Søk til meg pls")))),
            ]))));
            let template = template("gutz_html.html", Some(context));
            return match template {
                Ok(template) => Response::new_200(template),
                Err(e) => panic!("Error: {}", match e {
                    Error::InvalidParams(s) => s,
                    Error::ParseTemplate => "What tha hell".to_string(),
                    _ => "Unknown error".to_string(),
                }),
            };
        }),
    ];
    start_server(routes, ServerOptions {address: None});
}
