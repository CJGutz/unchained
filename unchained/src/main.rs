use unchained_web::server::Server;

fn main() {
    let routes = vec![];
    let server = Server::new(routes);
    server.listen();
}
