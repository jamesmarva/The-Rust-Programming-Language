use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for s in listener.incoming() {
        handle_tcpstream(s.unwrap());
    }
}


fn handle_tcpstream(mut s: TcpStream) {
    let mut buffer = [0u8; 1024 * 8];

    s.read(&mut buffer).unwrap();

    let resp_str = "HTTP/1.1 200 OK\r\n\r\nhello james";

    s.write(resp_str.as_bytes()).unwrap();
    s.flush().unwrap();
}
