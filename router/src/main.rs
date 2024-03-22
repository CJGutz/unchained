use std::{net::{TcpListener, TcpStream}, io::{Read, Write}};

fn handle_connection(stream: &mut TcpStream) {
    let mut content_read = String::new();
    stream.read_to_string(&mut content_read).expect("Could not parse content");
    println!("Content:\n{}", content_read);
    stream.write(b"HTTP/1.1 200 ok\r\n\r\nHello world").expect("Could not write to stream");
}

const ADDRESS: &str = "127.0.0.1:8080";

fn main() {
    let address = TcpListener::bind(ADDRESS).unwrap();
    for stream in address.incoming() {
        match stream {
            Ok(mut stream) => handle_connection(&mut stream),
            Err(_) => {
                println!("Could not handle tcp connection.");
                return;
            }
        }
    }
}
