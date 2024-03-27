use std::collections::HashMap;

use unchained::{
    router::{start_server, ServerOptions, Route, HTTPVerb::*, Response, ResponseContent},
    templates::{
        render::template,
        context::{ContextTree as Ctx, Primitive::*},
    }, error::Error
};

fn main() {

    let mut context = HashMap::new();
    context.insert("list".to_string(), Ctx::Array(Box::new(vec![
        Ctx::Leaf(Str("Hello".to_string())),
        Ctx::Leaf(Str("World".to_string())),
    ])));
    context.insert("title".to_string(), Ctx::Branch(Box::new(HashMap::from([
        ("title".to_string(), Ctx::Leaf(Str(String::from("Soek til meg pls")))),
    ]))));
    let start = std::time::Instant::now();
    let template = template("gutz_html_ascii.html", Some(context));
    let duration = start.elapsed();
    println!("Finished rendering after {} s", duration.as_secs_f64());

    let routes = vec![
        Route::new(GET, String::from("/"), ResponseContent::Create(Box::new(move |_req| {
            return match &template {
                Ok(template) => Response::new_200(template.to_string()),
                Err(e) => panic!("Error: {}", match e {
                    Error::InvalidParams(s) => s.to_string(),
                    Error::ParseTemplate => "What tha hell".to_string(),
                    _ => "Unknown error".to_string(),
                }),
            };
        }))),
        Route::new(GET, String::from("/images/*"), ResponseContent::None)
    ];
    start_server(routes, ServerOptions {address: None});
}
