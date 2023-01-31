fn heavy_computation() {
    // sleep for 5 seconds
    std::thread::sleep(std::time::Duration::from_secs(5));
}

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer);

heavy_computation();

    let (content_length, body) = html_body();
    let html = html_response(content_length, body);
    let _ = stream.write(html.as_bytes());
    let _ = stream.flush();

}


struct SingleThreadedServer {
    listener: TcpListener,
}
impl SingleThreadedServer {
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

struct MultiThreadedServer {
    listener: TcpListener,
}
impl MultiThreadedServer {
    fn new() -> Self {
        let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
        Self { listener }
}
    fn run(&self) {
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


fn main() {
    // start a thread
    let handle = std::thread::spawn(|| {
        let server = MultiThreadedServer::new();
        server.run();
    });

    //wait for the thread to finish
    let _ = handle.join();

}

fn html_body() -> (i64, String) {
    let mut body = String::new();
    body.push_str("<!DOCTYPE html>");
    body.push_str("<html>");
    body.push_str("<head>");
    body.push_str("<title>My first Rust web server</title>");
    body.push_str("</head>");
    body.push_str("<body>");
    body.push_str("<h1>Hello, world!</h1>");
    body.push_str("</body>");
    body.push_str("</html>");
    let content_length = body.len() as i64;
    (content_length, body)
}

fn html_response(content_length: i64,body: String) -> String {
    let mut html = String::new();
    html.push_str("HTTP/1.1 200 OK\r");
    html.push_str("Content-Type: text/html; charset=UTF-8\r\n");
    html.push_str(&format!("Content-Length: {}\r", content_length));
    html.push_str("\n\r\n");
    html.push_str(&body);
    html
}