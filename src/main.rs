fn main() {
    let http_server = Server::new("127.0.0.1:8080".to_string());
    http_server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Server {
        Server {
            addr
        }
    }

    fn run(self) {
        println!("Running on {}", self.addr);
    }
}

struct Request {
    path: String,
    query_string: String,
    method: Method
}

enum Method {
    GET,
    DELTE,
    POST,
    PUT
}


