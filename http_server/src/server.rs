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
                Ok((stream, addr)) => {}
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
