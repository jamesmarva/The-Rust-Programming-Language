use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for s in listener.incoming() {
        let stream = s.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: \n{}", String::from_utf8_lossy(&buffer[..]));

}
