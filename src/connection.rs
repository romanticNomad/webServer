use std::{io::{BufRead, BufReader, Write}, net::TcpStream};

pub fn io(mut stream: TcpStream) {
    let stream_data = BufReader::new(&stream);
    let _request: Vec<String> = stream_data
        .lines()
        .map(|data| data.unwrap())
        .take_while(|line| !line.is_empty()) //htpps request ends with \n\n.
        .collect();

    let response = "HTTP/1.1 200 OK \r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}
