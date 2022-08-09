#![allow(dead_code)]
use http::Method;
use http::Request;
use server::Server;
use website_handler::WebsiteHandler;
mod http;
mod server; //order doesn't matter smart compiler
mod website_handler;
fn main() {
    let server = server::Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler);
}
