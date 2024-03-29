use unchained::router::{start_server, ServerOptions};


fn main() {

    let routes = vec![];
    start_server(routes, ServerOptions {address: None});
}
