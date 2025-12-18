#[allow(dead_code)]
mod connection;
#[allow(dead_code)]
mod pool;

use std::net::TcpListener;
use pool::*;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listner.incoming() {
        let stream = stream.unwrap();
        
        pool.execute(|| {
            connection::io(stream);
        });
    };
}
