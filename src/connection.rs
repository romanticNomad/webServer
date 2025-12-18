use std::{io::{BufRead, BufReader}, net::TcpStream};

pub fn recieve(stream: TcpStream) {
    let stream_data = BufReader::new(&stream);
    let request: Vec<String> = stream_data
        .lines()
        .map(|data| data.unwrap())
        .take_while(|line| !line.is_empty()) //htpps request ends with \n\n.
        .collect();

    println!("http request; {request:#?}")
}
