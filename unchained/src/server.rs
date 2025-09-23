use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    sync::Arc,
};

use crate::{
    error::{Error, WebResult},
    router::{check_routes, Request, Route},
    workers::Workers,
};

fn handle_connection(
    mut stream: TcpStream,
    routes: &Vec<Route>,
    options: &ServerOptions,
) -> WebResult<()> {
    let mut content_read = String::new();
    let mut buf_read = BufReader::new(&mut stream);
    let res = buf_read.read_line(&mut content_read);
    if res.is_err() {
        return Err(Error::Connection(
            "Could not read from stream. Invalid buffer.".to_string(),
        ));
    }

    let (verb, path) = match content_read.split(' ').collect::<Vec<_>>()[..] {
        [verb, path, _version] => (verb, path),
        _ => {
            return Err(Error::Connection(format!(
                "Unimplemented request handle for: '{}",
                content_read
            )))
        }
    };

    let mut headers = HashMap::new();
    let mut content_read = String::new();

    loop {
        content_read.clear();
        let res = buf_read.read_line(&mut content_read);
        if res.is_err() {
            return Err(Error::Connection(
                "Could not read from stream. Invalid buffer.".to_string(),
            ));
        }
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

    // HTTP/1.1 Content-Length SHOULD be included in the headers when body is present
    // See https://www.rfc-editor.org/rfc/rfc9110.html#name-content-length
    let body_len = headers
        .get("Content-Length")
        .unwrap_or(&String::from("0"))
        .parse::<u64>()
        .unwrap_or(0);
    let mut buf = vec![0; body_len as usize];
    let res = buf_read.read_exact(&mut buf);
    let body = match res {
        Ok(_) => String::from_utf8(buf).ok(),
        Err(_) => {
            return Err(Error::Connection(format!(
                "Failed to read body of content length {}.",
                body_len
            )))
        }
    };

    let request = Request {
        verb: verb.to_string(),
        path: path.to_string(),
        path_params: HashMap::new(),
        body,
        headers,
    };

    let response = check_routes(routes, request);

    let mut response_headers = response.headers.clone();
    let response_bytes = response.bytes.unwrap_or_default();
    response_headers
        .entry("Content-Length".into())
        .or_insert(response_bytes.len().to_string());
    response_headers.insert("Connection".into(), "close".into());

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
        .and_then(|_| stream.write_all(&response_bytes))
        .and_then(|_| stream.shutdown(Shutdown::Write));

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
