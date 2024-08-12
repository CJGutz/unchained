use unchained_web::router::Server;

fn main() {
    let routes = vec![];
    let server = Server::new(routes);
    server.listen();
}
