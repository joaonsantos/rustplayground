use std::{net::TcpListener, io::Read};

use crate::http::Request;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {addr}
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("server started on {}", self.addr);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));
                            match Request::try_from(&buf[..]) {
                                Ok(r) => {
                                    dbg!(r);
                                },
                                Err(e) => println!("Failed to parse request: {}", e),
                            }
                        },
                        Err(e) => println!("Failed to read from stream: {}", e),
                    }
                    
                },
                Err(e) => {
                    println!("Failed to start connection: {}", e);
                },
            };
        }
       
        
       
    }
}