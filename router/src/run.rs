use std::{net::{TcpListener, TcpStream}, io::{BufRead, Write, BufReader}, collections::HashMap, path::PathBuf};

pub struct Request {
    pub verb: String,
    pub path: String,
    pub body: Option<String>,
    pub headers: HashMap<String, String>,
}

pub struct Response {
    pub bytes: Option<Vec<u8>>,
    pub status_code: u32,
}

impl Response {
    pub fn new(content: Option<String>, status_code: u32) -> Response {
        Response {
            bytes: content.map(|s| s.as_bytes().to_vec()),
            status_code,
        }
    }
    pub fn new_200(content: String) -> Response {
        Response {
            bytes: Some(content.as_bytes().to_vec()),
            status_code: 200,
        }
    }
}

pub enum ResponseContent {
    Str(String),
    Bytes(Vec<u8>),
    Create(Box<dyn Fn(Request) -> Response>),
    None,
}

pub struct Route {
    pub verb: HTTPVerb,
    pub path: String,
    pub response: ResponseContent,
}

impl Route {
    pub fn new(verb: HTTPVerb, path: String, response: ResponseContent) -> Route {
        Route {
            verb,
            path,
            response
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
        if route.path.ends_with("*") && request.path.contains(route.path.trim_end_matches("*")) {
            let buf = PathBuf::from(&request.path.trim_start_matches("/"));
            let file = std::fs::read(buf).ok();
            return Response {
                bytes: file.clone(),
                status_code: if file.is_some() { 200 } else { 404 },
            };
        } else if route_verb == request.verb && route.path.trim_end_matches("/") == request.path.trim_end_matches("/") {
            return match &route.response {
                ResponseContent::Str(s) => Response::new_200(s.to_string()),
                ResponseContent::Bytes(b) => Response {
                    bytes: Some(b.to_vec()),
                    status_code: 200,
                },
                ResponseContent::Create(f) => f(request),
                ResponseContent::None => Response::new(None, 200),
            };
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
        [verb, path, _version] => (verb, path),
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

    stream.write_fmt(format_args!("HTTP/1.1 {} 200\r\n\r\n", response.status_code)).expect("Failed to write to stream.");

    stream.write_all(&response.bytes.unwrap_or(vec![])).unwrap();
    stream.write_all(b"\r\n").unwrap();

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
