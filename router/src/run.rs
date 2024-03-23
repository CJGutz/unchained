use std::{net::{TcpListener, TcpStream}, io::{BufRead, Write, BufReader}, collections::HashMap};

pub struct Request {
    pub verb: String,
    pub path: String,
    pub body: Option<String>,
    pub headers: HashMap<String, String>,
}

pub struct Response {
    pub content: Option<String>,
    pub status_code: u32,
}

impl Response {
    pub fn new(content: Option<String>, status_code: u32) -> Response {
        Response {
            content,
            status_code,
        }
    }
    pub fn new_200(content: String) -> Response {
        Response {
            content: Some(content),
            status_code: 200,
        }
    }
}

pub struct Route {
    pub verb: HTTPVerb,
    pub path: String,
    pub get_response: fn(Request) -> Response,
}

impl Route {
    pub fn new(verb: HTTPVerb, path: String, get_response: fn(Request) -> Response) -> Route {
        Route {
            verb,
            path,
            get_response,
        }
    }
}

fn check_routes(routes: &Vec<Route>, request: Request) -> Response {
    for route in routes {
        let route_verb = match route.verb {
            HTTPVerb::GET => "GET",
            HTTPVerb::POST => "POST",
            HTTPVerb::UPDATE => "UPDATE",
            HTTPVerb::DELETE => "DELETE",
            HTTPVerb::HEAD => "HEAD",
        };
        if route_verb == request.verb && route.path.trim_end_matches("/") == request.path.trim_end_matches("/") {
            return (route.get_response)(request);
        }
    }
    return Response::new(None, 404);
}

fn handle_connection(mut stream: TcpStream, routes: &Vec<Route>) {

    let mut content_read = String::new();
    let mut buf_read = BufReader::new(&mut stream);
    buf_read.read_line(&mut content_read).unwrap();

    let first_line = content_read.lines().take(1).collect::<String>();
    let (verb, path) = match first_line.split(" ").collect::<Vec<_>>()[0..3] {
        [verb, path, "HTTP/1.1"] => (verb, path),
        _ => todo!(),
    };

    let mut headers = HashMap::new();

    loop {
        let mut content_read = String::new();
        buf_read.read_line(&mut content_read).unwrap();
        match content_read.trim().split(": ").collect::<Vec<_>>().as_slice() {
            [k, v] => headers.insert(k.to_string(), v.to_string()),
            _ => None,
        };
        if content_read.trim().is_empty() {
            break;
        }
    }

    let request = Request {
        verb: verb.to_string(),
        path: path.to_string(),
        body: None,
        headers,
    };

    let response = check_routes(routes, request);

    stream.write_fmt(format_args!("HTTP/1.1 {} OK\r\n\r\n{}\r\n", response.status_code, response.content.unwrap_or("".to_string()))).expect("Failed to write to stream.");

    stream.shutdown(std::net::Shutdown::Both).unwrap();
}

pub enum HTTPVerb {
    GET,
    POST,
    UPDATE,
    DELETE,
    HEAD
}


pub struct ServerOptions {
    pub address: Option<String>,
}

const ADDRESS: &str = "localhost:8080";

pub fn start_server(routes: Vec<Route>, options: ServerOptions) {
    let address = TcpListener::bind(options.address.unwrap_or(ADDRESS.to_string())).unwrap();
    for stream in address.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream, &routes),
            Err(_) => {
                println!("Could not handle tcp connection.");
                return;
            }
        }
    }
}
