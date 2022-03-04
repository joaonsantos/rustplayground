use std::{net::TcpListener, io::Read};
use std::convert::TryFrom;
use crate::http::Request;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {addr}
    }

    pub fn run(self) {
        println!("server started on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        //loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("received a request: {}", String::from_utf8_lossy(&buf));

                            match Request::try_from(&buf[..]) {
                                Ok(req) => {
                                    dbg!(req.path);

                                },
                                Err(e) => println!("failed to parse request: {}", e),
                            };
                        },
                        Err(e) => println!("failed to read from connection: {}", e)
                    }
                }
                Err(e) => println!("failed to establish a connection: {}", e)
            }
            //let (stream, sock_addr) = listener.accept().unwrap();
        //}
    }
}