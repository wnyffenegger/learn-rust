use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Read;
use std::net::TcpListener;

// By default this becomes the 'server' module
// so it needs to be addressed as server::Server
// or included with use or something

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
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
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(err) => {
                                    println!("Failed to translate buffer into string {}", err);
                                }
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
