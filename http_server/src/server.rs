use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::TcpListener;

// By default this becomes the 'server' module
// so it needs to be addressed as server::Server
// or included with use or something

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BAD_REQUEST, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Server is listening on {} ", &self.addr);

        // Bind returns a recoverable error
        // but we want to convert it to an unrecoverable error
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            // Want to recover from a connection if it fails
            let res = listener.accept();

            match listener.accept() {
                // Read function takes mutable reference to self so we must declare it as mutable
                // fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
                Ok((mut stream, addr)) => {
                    // Must specify a value for every byte in the array which
                    // prevents an overread
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            // Need to convert buffer to actual text to read to screen
                            // we don't care if they're invalid right now
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(err) => handler.handle_bad_request(&err),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
