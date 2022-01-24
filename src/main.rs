use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let server = TcpListener::bind("127.0.0.1:9090").unwrap();
    for stream in server.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let get = b"GET / HTTP/1.1\r\n";
    stream.read(&mut buffer).unwrap();

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    println!("Get {}", String::from_utf8_lossy(&buffer));
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\n\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
