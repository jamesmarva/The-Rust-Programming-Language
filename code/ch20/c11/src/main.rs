use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        thread::spawn(|| {
            handle_tcpstream(stream.unwrap());
        });
    }
}

fn handle_tcpstream(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024 * 8];

    stream.read(&mut buffer).unwrap();

    println!("{}", String::from_utf8_lossy(&buffer));

    let get_req_line = b"GET / HTTP/1.1";
    let sleep_req_line = b"GET /sleep HTTP/1.1";

    let (statue_line, filename) = if buffer.starts_with(get_req_line) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep_req_line) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let html = fs::read_to_string(filename).unwrap();
    let resp = format!("{}{}", statue_line, html);
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}
