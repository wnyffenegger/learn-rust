use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    println!("Reading contents at {}", file_path);
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory traversal attack: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::OK, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::OK, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(contents) => {
                        println!("Grabbing {}", path);
                        Response::new(StatusCode::OK, Some(contents))
                    }
                    None => Response::new(StatusCode::NOT_FOUND, None),
                },
            },
            _ => Response::new(StatusCode::NOT_FOUND, None),
        }
    }
}
