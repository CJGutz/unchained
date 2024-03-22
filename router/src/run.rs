use std::{net::{TcpListener, TcpStream}, io::{Read, Write}};

struct Response {
    content: Option<String>,
    status_code: u32,
}

fn check_routes(routes: &Vec<Route>, verb: String, path: String) -> Response {
    for route in routes {
        let route_verb = match route.verb {
            HTTPVerb::GET => "GET",
            HTTPVerb::POST => "POST",
            HTTPVerb::UPDATE => "UPDATE",
            HTTPVerb::DELETE => "DELETE",
            HTTPVerb::HEAD => "HEAD",
        };
        if route_verb == verb && route.path == path {
            return Response {content: Some(format!("Hello from {}\nContent:\n{}", route.path, route.content)), status_code: 200 };
        }
    }
    return Response { content: None, status_code: 404 }
}

fn handle_connection(stream: &mut TcpStream, routes: &Vec<Route>) {
    let buf: &mut [u8; 300] = &mut [0; 300];
    stream.read_exact(buf).unwrap();
    let content_read = String::from_utf8(buf.to_vec()).expect("invalid utf8");
    println!("Content:\n{}", content_read);
    let first_line = content_read.lines().take(1).collect::<String>();
    let (verb, path) = match first_line.split(" ").collect::<Vec<_>>()[0..3] {
        [verb, path, "HTTP/1.1"] => (verb, path),
        _ => todo!(),
    };

    println!("HTTP verb: {}", verb);
    
    
    let response = check_routes(routes, verb.to_string(), path.to_string());
    stream.write_fmt(format_args!("HTTP/1.1 {} OK\r\n\r\n{}\r\n", response.status_code, response.content.unwrap_or("".to_string()))).expect("Failed to read to stream.");

    stream.shutdown(std::net::Shutdown::Both).unwrap();
}

pub enum HTTPVerb {
    GET,
    POST,
    UPDATE,
    DELETE,
    HEAD
}

pub struct Route {
    pub verb: HTTPVerb,
    pub path: String,
    pub content: String,
}

pub struct ServerOptions {
    pub address: Option<String>,
}

const ADDRESS: &str = "localhost:8080";

pub fn start_server(routes: Vec<Route>, options: ServerOptions) {
    let address = TcpListener::bind(options.address.unwrap_or(ADDRESS.to_string())).unwrap();
    for stream in address.incoming() {
        match stream {
            Ok(mut stream) => handle_connection(&mut stream, &routes),
            Err(_) => {
                println!("Could not handle tcp connection.");
                return;
            }
        }
    }
}
