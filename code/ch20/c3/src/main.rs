use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for s in listener.incoming() {
        handle_connection(s.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024 * 8];

    stream.read(&mut buffer).unwrap();

    let resp = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}
