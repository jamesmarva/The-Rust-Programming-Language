use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        handle_tcpstream(stream.unwrap());
    }
}

fn handle_tcpstream(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024 * 8];

    stream.read(&mut buffer).unwrap();
    println!("request: \n{}", String::from_utf8_lossy(&buffer[..]));

    let get_method_req_line_bytes = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get_method_req_line_bytes) {
        let html = fs::read_to_string("hello.html").unwrap();
        let resp_str = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            html.len(),
            html
        );
        stream.write(resp_str.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let html = fs::read_to_string("404.html").unwrap();
        let resp_header = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let resp = format!("{}{}", resp_header, html);
        stream.write(resp.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
