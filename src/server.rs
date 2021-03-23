use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Server {
        Server { addr }
    }

    pub fn run(self) {
        let listner = TcpListener::bind(&self.addr).unwrap();
        println!("Listening on {}", listner.local_addr().unwrap());

        loop {
            listner.accept();

        }
    }
}

