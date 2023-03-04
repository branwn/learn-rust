use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use rand::Rng;

fn handle_client(mut stream: TcpStream) {
    let password: String = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();
    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n{}\n", password);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Listening on port 8000...");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_client(stream);
        });
    }
}
