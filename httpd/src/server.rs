use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::http::Request;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("server started on {}", self.addr);

        for stream in listener.incoming() {
            match stream {
                Ok(s) => handle_client(s),
                Err(e) => println!("Failed to read from stream: {}", e),
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 1024];

    match stream.read(&mut buf) {
        Ok(_) => {
            println!("Received a request: {}", String::from_utf8_lossy(&buf));
            match Request::try_from(&buf[..]) {
                Ok(r) => {
                    dbg!(r);
                    let response = "HTTP/1.1 200 OK\r\n\r\n";

                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                Err(e) => println!("Failed to parse request: {}", e),
            }
        }
        Err(e) => println!("Failed to read from stream: {}", e),
    }
}
