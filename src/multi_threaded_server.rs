use crate::commons::handle_client;
use std::net::TcpListener;

pub struct MultiThreadedServer {
    listener: TcpListener,
}
impl MultiThreadedServer {
    pub fn new() -> Self {
        let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
        Self { listener }
    }
    pub fn run(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    std::thread::spawn(move || {
                        handle_client(stream);
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}
