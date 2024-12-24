#[allow(unused_imports)]
use std::net::TcpListener;
use std::io::{Read, Write};
use std::thread;
use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let args: Vec<String> = env::args().collect();
    let mut directory = None;

    for i in 0..args.len() - 1 {
        if args[i] == "--directory" {
            directory = Some(PathBuf::from(&args[i + 1]));
        }
    }

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");

                let dir = directory.clone();

                thread::spawn(move || {
                    handle_connection(stream, dir);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


fn handle_connection(mut stream: std::net::TcpStream, directory: Option<PathBuf>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer);
    let lines: Vec<&str> = request.lines().collect();
    let request_line = request.lines().next().unwrap_or("");

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    let path = parts.get(1).unwrap_or(&"");


    let response = if *path == "/" {
        "HTTP/1.1 200 OK\r\n\r\n".to_string()
    } else if path.starts_with("/echo/"){
        let echo_string= &path[6..];
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            echo_string.len(),
            echo_string
        )
    } else if *path == "/user-agent" {
        let user_agent = lines.iter()
        .find(|line| line.to_lowercase().starts_with("user-agent: "))
        .map(|line| line.splitn(2, ": ").nth(1).unwrap_or(""))
        .unwrap_or("");

        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            user_agent.len(),
            user_agent
        )
    } else if path.starts_with("/files/") {
        if let Some(dir) = directory {
            let filename = &path[7..];
            let file_path = dir.join(filename);

            match fs::read(&file_path) {
                Ok(contents ) => {
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                        contents.len(),
                        String::from_utf8_lossy(&contents)
                    )
                }
                Err(_) => "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
            }
        } else {
            "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
        }
    }    
    else {
        "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
    };

    if let Err(e) = stream.write_all(response.as_bytes()) {
        println!("Failed to write to connection: {}", e);
    }
}
