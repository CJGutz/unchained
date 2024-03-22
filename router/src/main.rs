pub mod run;
use run::{ServerOptions, Route, HTTPVerb};

fn main() {
    let routes = vec![
        Route {
            verb: HTTPVerb::GET,
            path: String::from("/"),
            content: String::from("Hei Kristine. Hvordan har du det i dag?"),

        }
    ];
    run::start_server(routes, ServerOptions {address: None});
}
