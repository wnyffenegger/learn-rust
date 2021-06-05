use http::Method;
use http::Request;
use server::Server;

// Compiler will go find server module and pull it into scope
mod http;
mod server;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
