pub mod run;
pub mod render;
use std::collections::HashMap;

use render::template;
use run::{ServerOptions, Route, HTTPVerb::*, Response};

fn main() {

    let routes = vec![
        Route::new(GET, String::from("/"), |_req| Response::new_200(String::from("Hello, World!"))),
        Route::new(GET, String::from("/hello/"),   |_req| {
            let mut context = HashMap::new();
            context.insert("title".to_string(), "My website title".to_string());
            return Response::new_200(template("my_html.html".to_string(), Some(context)));
        }),
    ];
    run::start_server(routes, ServerOptions {address: None});
}
