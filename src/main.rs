#[allow(dead_code)]
mod connection;
#[allow(dead_code)]
mod pool;

use std::net::TcpListener;
use pool::*;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    // println!("online");
    let pool = ThreadPool::new(4);

    for stream in listner.incoming().take(5) {
        // println!("incoming");
        let stream = stream.unwrap();
        
        pool.execute(|| {
            connection::io(stream);
        });
    }
    println!("Shutting down!");
}
