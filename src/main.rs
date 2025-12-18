#[allow(dead_code)]
mod connection;
use std::net::TcpListener;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listner.incoming() {
        let stream = stream.unwrap();
        connection::io(stream);
    };
}
