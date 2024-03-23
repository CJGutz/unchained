pub mod run;
pub mod render;
use render::template;
use run::{ServerOptions, Route, HTTPVerb::*, Response};

fn main() {
    let routes = vec![
        Route::new(GET, String::from("/"), |_req| Response::new_200(String::from("Hello, World!"))),
        Route::new(GET, String::from("/hello/"), |_req| {
            return Response::new_200(template("my_html.html".to_string(), None));
        }),
    ];
    run::start_server(routes, ServerOptions {address: None});
}
