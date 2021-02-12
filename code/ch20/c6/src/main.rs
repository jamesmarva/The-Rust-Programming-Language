use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for s in listener.incoming() {
        handle_stream(s.unwrap());
    }
}

fn handle_tcpstream(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024 * 8];
    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer));

    let html_content = fs::read_to_string("hello.html").unwrap();
    let resp_str = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        html_content.len(),
        html_content
    );
    stream.write(resp_str.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024 * 8];
    stream.read(&mut buffer).unwrap();

    let get_req_line_bytes = b"GET / HTTP/1.1";
    if buffer.starts_with(get_req_line_bytes) {
        let html = fs::read_to_string("hello.html").unwrap();
        let resp = format!( "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        html.len(),
        html);
        stream.write(resp.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {

    }
}