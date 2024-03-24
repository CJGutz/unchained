use std::collections::HashMap;

use router::{
    run::{start_server, ServerOptions, Route, HTTPVerb::*, Response},
    templates::{
        render::template,
        context::{ContextTree as Ctx, Primitive::*},
    }
};

fn main() {

    let routes = vec![
        Route::new(GET, String::from("/hello/"),   |_req| {
            let mut context = HashMap::new();
            context.insert("list".to_string(), Ctx::Array(Box::new(vec![
                Ctx::Leaf(Str("Hello".to_string())),
                Ctx::Leaf(Str("World".to_string())),
            ])));
            return Response::new_200(template("my_html.html", Some(context)).unwrap());
        }),
    ];
    start_server(routes, ServerOptions {address: None});
}
