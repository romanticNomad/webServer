use std::{
    fs,
    io::{BufRead, BufReader, Write}, 
    net::TcpStream,
};

pub fn io(mut stream: TcpStream) {
    let stream_data = BufReader::new(&stream);
    let request = stream_data
        .lines()
        .next()
        .unwrap()
        .unwrap();

    if request == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("html/hello.html").unwrap();
        let length = contents.len();

        let response = 
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let error_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("html/error.html").unwrap();
        let length = contents.len();

        let response = format!("{error_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}
