use crate::commons::{handle_client, Server};
use std::net::TcpListener;

pub struct SingleThreadedServer {
    listener: TcpListener,
}
impl Server for SingleThreadedServer {
    fn new() -> Self {
        let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
        Self { listener }
    }
    fn run(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_client(stream);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}
