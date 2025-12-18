use std::{
    fs,
    io::{BufRead, BufReader, Write}, 
    net::TcpStream,
    //  thread, time::Duration,
};

pub fn io(mut stream: TcpStream) {
    let stream_data = BufReader::new(&stream);
    let request = stream_data
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let (status_line, path) = if request == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "html/hello.html") }
    // } else if request == "GET /sleep HTTP/1.1" {
    //     thread::sleep(Duration::from_secs(5));
        //     ("HTTP/1.1 200 OK", "html/hello.html") } slow request block other requests on single thread server.
    else {
        ("HTTP/1.1 404 NOT FOUND", "html/error.html")
    };
    let content = fs::read_to_string(path).unwrap();

    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}
