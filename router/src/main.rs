pub mod run;
use run::{ServerOptions, Route, HTTPVerb::*};

fn main() {
    let routes = vec![
        Route::new(GET, String::from("/"), |_req| String::from("Hello, World!")),
        Route::new(GET, String::from("/bruh"), |_req| String::from("Hello, bruh!")),
        Route::new(GET, String::from("/bruh/"), |_req| String::from("Hello, bruh with /!")),

    ];
    run::start_server(routes, ServerOptions {address: None});
}
