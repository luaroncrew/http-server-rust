#![allow(dead_code)]

use server::Server;
use http::Method;
use http::request;
use std::env;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {
    let public_path = env::var("PUBLIC_PATH").unwrap();
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}
