use server::Server;
use http::method::Method;
mod server;
mod http;

fn main() {
    let http_server = Server::new("127.0.0.1:8080".to_string());
    http_server.run();
}


