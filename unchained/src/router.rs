use std::{
    collections::HashMap, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, path::PathBuf
};

pub struct Request {
    pub verb: String,
    pub path: String,
    pub body: Option<String>,
    pub headers: HashMap<String, String>,
}

pub enum HTTPVerb {
    GET,
    POST,
    UPDATE,
    DELETE,
    HEAD,
}

impl ToString for HTTPVerb {
    fn to_string(&self) -> String {
        match self {
            HTTPVerb::GET => "GET",
            HTTPVerb::POST => "POST",
            HTTPVerb::UPDATE => "UPDATE",
            HTTPVerb::DELETE => "DELETE",
            HTTPVerb::HEAD => "HEAD",
        }
        .to_string()
    }
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
    FromRequest(Box<dyn Fn(Request) -> Response>),
    FolderAccess,
}

pub struct Route {
    pub verb: HTTPVerb,
    pub path: String,
    pub response: ResponseContent,
}

impl Route {
    pub fn new(verb: HTTPVerb, path: &str, response: ResponseContent) -> Route {
        Route {
            verb,
            path: path.to_string(),
            response,
        }
    }
}

fn check_routes(routes: &Vec<Route>, request: Request) -> Response {
    for route in routes {
        let route_verb = route.verb.to_string();
        let route_path = route
            .path
            .trim_end_matches('/')
            .trim_end_matches('*')
            .trim_start_matches('/');
        let req_path = request.path.trim_end_matches('/').trim_start_matches('/');

        if route.path.ends_with('*')
            && req_path.starts_with(route_path)
            && matches!(route.response, ResponseContent::FolderAccess)
        {
            let buf = PathBuf::from(req_path);
            let file = std::fs::read(buf).ok();
            return Response {
                bytes: file.clone(),
                status_code: if file.is_some() { 200 } else { 404 },
            };
        } else if route_verb == request.verb && route_path == req_path {
            match &route.response {
                ResponseContent::Str(s) => return Response::new_200(s.to_string()),
                ResponseContent::Bytes(b) => {
                    return Response {
                        bytes: Some(b.to_vec()),
                        status_code: 200,
                    }
                }
                ResponseContent::FromRequest(f) => return f(request),
                ResponseContent::FolderAccess => (),
            };
        }
    }
    Response::new(None, 404)
}

fn handle_connection(mut stream: TcpStream, routes: &Vec<Route>) {
    let mut content_read = String::new();
    let mut buf_read = BufReader::new(&mut stream);
    buf_read.read_line(&mut content_read).unwrap();

    let first_line = content_read.lines().take(1).collect::<String>();
    let (verb, path) = match first_line.split(' ').collect::<Vec<_>>()[..] {
        [verb, path, _version] => (verb, path),
        _ => panic!("Unimplemented request handle for: {}", first_line)
    };

    let mut headers = HashMap::new();

    loop {
        let mut content_read = String::new();
        buf_read.read_line(&mut content_read).unwrap();
        match content_read
            .trim()
            .split(": ")
            .collect::<Vec<_>>()
            .as_slice()
        {
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

    stream
        .write_fmt(format_args!("HTTP/1.1 {} \r\n\r\n", response.status_code))
        .expect("Failed to write to stream.");

    stream.write_all(&response.bytes.unwrap_or(vec![])).unwrap();
    stream.write_all(b"\r\n").unwrap();

    stream.shutdown(std::net::Shutdown::Both).unwrap();
}

pub struct ServerOptions {
    pub address: String,
}

const ADDRESS: &str = "localhost:8080";

pub struct Server {
    pub routes: Vec<Route>,
    pub options: ServerOptions,
}

impl Server {
    pub fn new(routes: Vec<Route>) -> Server {
        Server { routes, options: ServerOptions { address: ADDRESS.to_string() } }
    }
    pub fn set_address(&mut self, address: &str) {
        self.options.address = address.to_string();
    }
    pub fn listen(&self) {
        let address = TcpListener::bind(self.options.address.clone()).unwrap();
        for stream in address.incoming() {
            match stream {
                Ok(stream) => handle_connection(stream, &self.routes),
                Err(_) => {
                    println!("Could not handle tcp connection.");
                    return;
                }
            }
        }
    }
}

