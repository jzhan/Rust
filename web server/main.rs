use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::fs;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 698];

    stream.read(&mut buffer).unwrap();

    // println!("READ:\n\n {}", String::from_utf8_lossy(&buffer[..]));

    let content = fs::read_to_string("index.html").unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_client(stream);
        });
    }
}
