use std::{collections::HashMap, fmt::Display, path::PathBuf};

#[derive(Clone, Debug)]
pub struct Request {
    pub verb: String,
    pub path: String,
    pub path_params: HashMap<String, String>,
    pub body: Option<String>,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub fn set_path_params(&mut self, params: HashMap<String, String>) -> &mut Self {
        self.path_params = params;
        self
    }
}

pub enum HTTPVerb {
    GET,
    POST,
    UPDATE,
    DELETE,
    HEAD,
}

impl Display for HTTPVerb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HTTPVerb::GET => write!(f, "GET"),
            HTTPVerb::POST => write!(f, "POST"),
            HTTPVerb::UPDATE => write!(f, "UPDATE"),
            HTTPVerb::DELETE => write!(f, "DELETE"),
            HTTPVerb::HEAD => write!(f, "HEAD"),
        }
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

/// Checks if the route matches the path.
/// Finds path params in the request.
fn compare_route_w_path_and_get_path_params(
    route: &str,
    req_path: &str,
) -> (bool, HashMap<String, String>) {
    let route_parts = route
        .trim_start_matches('/')
        .trim_end_matches('*')
        .trim_end_matches('/')
        .split('/')
        .collect::<Vec<_>>();
    let req_parts = req_path
        .trim_start_matches('/')
        .trim_end_matches('/')
        .split('/');
    let mut params = HashMap::new();
    let mut match_route = true;
    for (count, req_part) in req_parts.clone().enumerate() {
        let route_part = route_parts.get(count);
        if route_part.is_none() {
            let last_is_star = route.ends_with('*');
            match_route = last_is_star;
            break;
        }
        let route_part = *route_part.unwrap();

        if let Some(route_part) = route_part.strip_prefix(':') {
            params.insert(route_part.to_string(), req_part.to_string());
        } else if route_part != req_part {
            match_route = false;
            break;
        }
    }
    if route_parts.len() > req_parts.count() {
        match_route = false;
    }
    (match_route, params)
}

pub fn check_routes(routes: &Vec<Route>, request: Request) -> Response {
    for route in routes {
        let route_verb = route.verb.to_string();

        let (matches, params) =
            compare_route_w_path_and_get_path_params(&route.path, &request.path);
        let modified_request = request.clone().set_path_params(params).to_owned();

        if route_verb == request.verb && matches {
            return match &route.response {
                ResponseContent::Str(s) => Response::new_200(s.to_string()),
                ResponseContent::Bytes(b) => Response {
                    bytes: Some(b.to_vec()),
                    status_code: 200,
                    headers: HashMap::new(),
                },
                ResponseContent::FromRequest(f) => f(modified_request),
                ResponseContent::FolderAccess => {
                    read_file_to_respond(request.path.trim_start_matches('/'))
                }
            };
        }
    }
    Response::new(None, 404)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_equal_string_paths_matches() {
        let route_paths = vec![
            ("/path", "/path"),
            ("path", "path"),
            ("/path/", "/path/"),
            ("/path/", "path"),
            ("path", "/path"),
            ("/path", "path"),
        ];
        for (route, path) in route_paths {
            let (matches, _) = compare_route_w_path_and_get_path_params(route, path);
            assert!(matches);
        }
    }

    #[test]
    fn test_matches_w_path_params() {
        let route_paths = vec![
            ("/path/:id", "/path/123"),
            (":id/params", "more/params"),
            ("/:id/:again", "/123/once-more"),
            ("/:id/:id", "/123/once-more"),
        ];
        for (route, path) in route_paths {
            let (matches, _) = compare_route_w_path_and_get_path_params(route, path);
            assert!(matches);
        }
    }

    #[test]
    fn test_non_matching_routes_w_path_params() {
        let route_paths = vec![
            ("some-path", "other-path"),
            ("/*/some-path", "/some-path"),
            ("/:id/some-path", "/some-path"),
            ("/some-path/:id", "/some-path"),
            (":id/some-path/:id", "/some-path"),
        ];
        for (route, path) in route_paths {
            let (matches, _) = compare_route_w_path_and_get_path_params(route, path);
            assert!(!matches);
        }
    }
}
