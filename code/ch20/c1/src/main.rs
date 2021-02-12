use std::net::TcpListener;


fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for s in listener.incoming() {
        let stream = s.unwrap();
        println!("Connection established!");
    }

}
