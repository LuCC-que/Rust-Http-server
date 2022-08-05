use http::Method;
use http::Request;

use server::Server;
mod http;
mod server; //order doesn't matter smart compiler

fn main() {
    let server = server::Server::new("127.0.0.1:8080".to_string());
    server.run();
}
