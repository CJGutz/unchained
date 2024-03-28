use std::collections::HashMap;

use unchained::{
    error::Error, router::{start_server, HTTPVerb::*, Response, ResponseContent, Route, ServerOptions}, templates::{
        context::{ctx_map, ctx_str, ctx_vec}, render::template
    }
};

fn main() {

    let mut context = HashMap::new();
    context.insert("list".to_string(), ctx_vec(vec![
        ctx_str("Hello"),
        ctx_str("World"),
    ]));
    context.insert("title".to_string(), ctx_map([
        ("title", ctx_str("Soek til meg pls")),
    ]));

    context.insert("front_links".to_string(), ctx_vec(vec![
        ctx_map([("href", ctx_str("/#about")), ("label", ctx_str("About me"))]),
        ctx_map([("href", ctx_str("/experience")), ("label", ctx_str("Experience"))]),
        ctx_map([("href", ctx_str("/skills")), ("label", ctx_str("Skills"))]),
    ]));


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
