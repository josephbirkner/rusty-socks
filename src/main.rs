use std::net::TcpListener;
use std::io::Write;

fn main() {
    let srv = TcpListener::bind("0.0.0.0:8088").unwrap();
    println!("Listening!");
    for conn in srv.incoming() {
        let mut stream = conn.unwrap();
        stream.write("Hello there!".as_bytes()).unwrap();
    }
}
