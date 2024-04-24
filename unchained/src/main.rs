use unchained::router::Server;

fn main() {
    let routes = vec![];
    let server = Server::new(routes);
    server.listen();
}
