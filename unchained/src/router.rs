use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    sync::Arc,
};

use crate::{
    error::{Error, WebResult},
    workers::Workers,
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
    pub headers: HashMap<String, String>,
}

impl Response {
    pub fn new(content: Option<String>, status_code: u32) -> Response {
        Response {
            bytes: content.map(|s| s.as_bytes().to_vec()),
            status_code,
            headers: HashMap::new(),
        }
    }
    pub fn new_200(content: String) -> Response {
        Response {
            bytes: Some(content.as_bytes().to_vec()),
            status_code: 200,
            headers: HashMap::new(),
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
}

pub enum ResponseContent {
    Str(String),
    Bytes(Vec<u8>),
    FromRequest(Box<dyn Fn(Request) -> Response + Sync + Send>),
    FolderAccess,
}

pub struct Route {
    pub verb: HTTPVerb,
    pub path: String,
    pub response: ResponseContent,
    pub returned_headers: HashMap<String, String>,
}

impl Route {
    pub fn new(verb: HTTPVerb, path: &str, response: ResponseContent) -> Route {
        Route {
            verb,
            path: path.to_string(),
            response,
            returned_headers: HashMap::new(),
        }
    }

    pub fn new_with_headers(
        verb: HTTPVerb,
        path: &str,
        response: ResponseContent,
        headers: HashMap<String, String>,
    ) -> Route {
        Route {
            verb,
            path: path.to_string(),
            response,
            returned_headers: headers,
        }
    }
}

fn read_file_to_respond(file: &str) -> Response {
    let buf = PathBuf::from(file);
    let file = std::fs::read(buf).ok();
    Response {
        bytes: file.clone(),
        status_code: if file.is_some() { 200 } else { 404 },
        headers: HashMap::new(),
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

        let star_access = route.path.ends_with('*') && req_path.starts_with(route_path);
        if route_verb == request.verb && (route_path == req_path || star_access) {
            return match &route.response {
                ResponseContent::Str(s) => Response::new_200(s.to_string()),
                ResponseContent::Bytes(b) => Response {
                    bytes: Some(b.to_vec()),
                    status_code: 200,
                    headers: HashMap::new(),
                },
                ResponseContent::FromRequest(f) => f(request),
                ResponseContent::FolderAccess => read_file_to_respond(req_path),
            };
        }
    }
    Response::new(None, 404)
}

fn handle_connection(
    mut stream: TcpStream,
    routes: &Vec<Route>,
    options: &ServerOptions,
) -> WebResult<()> {
    let mut content_read = String::new();
    let mut buf_read = BufReader::new(&mut stream);
    buf_read.read_line(&mut content_read).unwrap();

    let first_line = content_read.lines().take(1).collect::<String>();
    let (verb, path) = match first_line.split(' ').collect::<Vec<_>>()[..] {
        [verb, path, _version] => (verb, path),
        _ => {
            return Err(Error::Connection(format!(
                "Unimplemented request handle for: '{}",
                first_line
            )))
        }
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

    let headers = options
        .default_headers
        .iter()
        .chain(response.headers.iter())
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<String>>()
        .join("\r\n");

    let write = stream
        .write_fmt(format_args!(
            "HTTP/1.1 {}\r\n{}\r\n\r\n",
            response.status_code, headers
        ))
        .and_then(|_a| stream.write_all(&response.bytes.unwrap_or_default()))
        .and_then(|_b| stream.write_all(b"\r\n"))
        .and_then(|_c| stream.shutdown(std::net::Shutdown::Both));

    if write.is_err() {
        return Err(Error::Connection("Could not write to stream.".to_string()));
    }
    Ok(())
}

#[derive(Clone)]
pub struct ServerOptions {
    pub address: String,
    pub threads: u32,
    pub default_headers: HashMap<String, String>,
}

const ADDRESS: &str = "0.0.0.0:8080";

pub struct Server {
    pub routes: Arc<Vec<Route>>,
    pub options: ServerOptions,
}

impl Server {
    pub fn new(routes: Vec<Route>) -> Server {
        let a: Arc<Vec<Route>> = Arc::from(routes);
        Server {
            routes: a,
            options: ServerOptions {
                address: ADDRESS.to_string(),
                threads: 4,
                default_headers: HashMap::new(),
            },
        }
    }

    pub fn set_address(&mut self, address: &str) -> &mut Self {
        self.options.address = address.to_string();
        self
    }

    pub fn set_threads(&mut self, threads: u32) -> &mut Self {
        self.options.threads = threads;
        self
    }

    pub fn add_default_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.options
            .default_headers
            .insert(key.to_string(), value.to_string());
        self
    }

    pub fn listen(&self) {
        let address = TcpListener::bind(self.options.address.clone()).unwrap();
        let workers = Workers::new(self.options.threads);
        for stream in address.incoming() {
            match stream {
                Ok(stream) => {
                    let routes = self.routes.clone();
                    let options = self.options.clone();
                    workers.post(move || handle_connection(stream, &routes, &options));
                }
                Err(_) => {
                    println!("Could not handle tcp connection.");
                }
            }
        }
    }
}
