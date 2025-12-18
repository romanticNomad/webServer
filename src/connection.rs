use std::{
    fs,
    io::{BufRead, BufReader, Write}, 
    net::TcpStream,
};

pub fn io(mut stream: TcpStream) {
    let stream_data = BufReader::new(&stream);
    let _request: Vec<String> = stream_data
        .lines()
        .map(|data| data.unwrap())
        .take_while(|line| !line.is_empty()) //htpps request ends with \n\n.
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = 
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap();

}
