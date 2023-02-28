use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use chrono::Local;

fn handle_client(mut stream: TcpStream) {
    println!("New client connected: {:?}", stream.peer_addr().unwrap());

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request received:\n{}", request);

    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n{}\r\n", Local::now().to_string());
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("Response sent:\n{}", response);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connection: {:?}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error accepting new connection: {}", e);
            }
        }
    }

    drop(listener);
}
