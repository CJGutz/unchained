pub mod templates;
pub mod error;
use std::collections::HashMap;

use router::templates::render::template;
use router::run::{start_server, ServerOptions, Route, HTTPVerb::*, Response};

fn main() {

    let routes = vec![
        Route::new(GET, String::from("/"), |_req| Response::new_200(String::from("Hello, World!"))),
        Route::new(GET, String::from("/hello/"),   |_req| {
            let mut context = HashMap::new();
            context.insert("title".to_string(), "My website title".to_string());
            return Response::new_200(template("my_html.html", Some(context)).unwrap());
        }),
    ];
    start_server(routes, ServerOptions {address: None});
}
