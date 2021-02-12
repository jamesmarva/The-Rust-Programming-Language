use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for s in listener.incoming() {
        handle_tcpstream(s.unwrap());
    }

}

fn handle_tcpstream(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024 * 8];
    stream.read(&mut buffer).unwrap();
    println!("{}",String::from_utf8_lossy(&buffer));
    let content = fs::read_to_string("hello.html").unwrap();
    
    let resp_content = format!( 
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );
    stream.write(resp_content.as_bytes()).unwrap();
    stream.flush().unwrap();

}
