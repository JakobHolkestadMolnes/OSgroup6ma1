use std::io::{Read, Write};
use std::net::TcpStream;

pub trait Server {
    fn new() -> Self
    where
        Self: Sized;
    fn run(&self);
}

fn html_body() -> (i64, String) {
    let body = String::from(
        r#"<!DOCTYPE html>
        <html>
        <head>
        <title>My first Rust web server</title>
        </head>
        <body>
        <h1>Hello, world!</h1>
        </body>
        </html>"#,
    );
    let content_length = body.len() as i64;
    (content_length, body)
}

pub fn html_response(content_length: i64, body: &str) -> String {
    format!(
        r#"HTTP/1.1 200 OK
        Content-Type: text/html; charset=UTF-8
        Content-Length: {}

        {}"#,
        content_length, body
    )
}

/// Simulate a heavy computation, sleeps for 2 seconds.
pub fn heavy_computation() {
    // sleep for 2 seconds
    std::thread::sleep(std::time::Duration::from_secs(2));
}
pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer);

    heavy_computation();

    let (content_length, body) = html_body();
    let html = html_response(content_length, &body);
    let _ = stream.write(html.as_bytes());
    let _ = stream.flush();
}
