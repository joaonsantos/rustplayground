use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use crate::http::{Response, Request, StatusCode, Router};

pub struct Server {
    addr: String,
    router: Router,
}

impl Server {
    pub fn new(addr: String, router: Router) -> Self {
        Self { addr, router }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("server started on {}", self.addr);

        for stream in listener.incoming() {
            match stream {
                Ok(s) => self.handle_client(s),
                Err(e) => println!("Failed to read from stream: {}", e),
            }
        }
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let mut buf = [0; 1024];
    
        match stream.read(&mut buf) {
            Ok(_) => {
                println!("Received a request: {}", String::from_utf8_lossy(&buf));
                let resp = match Request::try_from(&buf[..]) {
                    Ok(req) => {
                        dbg!(&req);
                        self.router.handle_request(req)
                    }
                    Err(e) => {
                        println!("Failed to parse request: {}", e);
                        Response::new(StatusCode::BadRequest, None)
                    }
                };
                dbg!(&resp);
                if let Err(e) = resp.send(&mut stream) {
                    println!("failed to send resp: {}", e);
                }
            }
            Err(e) => println!("Failed to read from stream: {}", e),
        }
    }
}


