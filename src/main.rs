#[allow(unused_imports)]
use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();

                let request = String::from_utf8_lossy(&buffer);
                let request_line = request.lines().next().unwrap_or("");

                let parts: Vec<&str> = request_line.split_whitespace().collect();
                let path = parts.get(1).unwrap_or(&"");


                let response = if *path == "/" {
                    "HTTP/1.1 200 OK\r\n\r\n"
                } else {
                    "HTTP/1.1 404 Not Found\r\n\r\n"
                };

                if let Err(e) = stream.write_all(response.as_bytes()) {
                    println!("Failed to write to connection: {}", e);
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
