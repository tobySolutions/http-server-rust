#[allow(unused_imports)]
use std::net::TcpListener;
use std::io::{Read, Write};
use std::thread;
use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    println!("Logs from your program will appear here!");

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
    let method = parts.get(0).unwrap_or(&"");
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

            match *method {
                "GET" => {
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
                },
                "POST" => {
                    let content_length: usize = lines.iter()
                        .find(|line| line.to_lowercase().starts_with("content-length: "))
                        .and_then(|line| line.splitn(2, ": ").nth(1))
                        .and_then(|len| len.parse().ok())
                        .unwrap_or(0);

                        if let Some(body_start) = request.find("\r\n\r\n") {
                            let body = &request[body_start + 4..body_start + 4 + content_length];
                            
                           
                            if let Err(_) = fs::write(&file_path, body) {
                                "HTTP/1.1 500 Internal Server Error\r\n\r\n".to_string()
                            } else {
                                "HTTP/1.1 201 Created\r\n\r\n".to_string()
                            }
                        } else {
                            "HTTP/1.1 400 Bad Request\r\n\r\n".to_string()
                        }

                },
                _ => "HTTP/1.1 405 Method Not Allowed\r\n\r\n".to_string()
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



// origin  https://git.codecrafters.io/b0c049671a193d1a (fetch)
// origin  https://git.codecrafters.io/b0c049671a193d1a (push)