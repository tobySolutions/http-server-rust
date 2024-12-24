# HTTP Server in Rust

A ground-up implementation of an HTTP/1.1 server in Rust, demonstrating low-level networking, concurrent programming, and file system operations.

## Features

- **HTTP/1.1 Protocol Support**: Implements core HTTP/1.1 specifications
- **Concurrent Connections**: Handles multiple client connections simultaneously using Rust's threading
- **Request Routing**: Supports multiple endpoints including:
  - `/` - Basic server health check
  - `/echo/{string}` - Echoes back the provided string
  - `/user-agent` - Returns the client's User-Agent
  - `/files/{filename}` - File operations (GET and POST)
- **File Operations**: 
  - GET: Retrieve file contents
  - POST: Create new files with provided content
- **Header Processing**: Handles standard HTTP headers including Content-Type and Content-Length

## Technical Details

- Built using Rust's standard library
- Uses `TcpListener` for network communication
- Implements thread-based concurrency
- Handles both static and dynamic responses
- Supports binary file content via `application/octet-stream`

## Usage

### Building the Server

```bash
cargo build --release
```

### Running the Server

Basic server start:
```bash
./target/release/http-server
```

With file directory support:
```bash
./target/release/http-server --directory /path/to/files
```

### Example Requests

GET request:
```bash
curl -v http://localhost:4221/
```

Echo endpoint:
```bash
curl -v http://localhost:4221/echo/hello-world
```

File upload:
```bash
curl -v --data "content" -H "Content-Type: application/octet-stream" http://localhost:4221/files/example.txt
```

## Implementation Details

- **TCP Connection Handling**: Uses Rust's `TcpListener` for accepting connections
- **Request Parsing**: Custom implementation of HTTP request parsing
- **Response Formation**: Properly formatted HTTP responses with headers
- **Concurrency**: Thread-per-connection model for handling multiple clients
- **Error Handling**: Robust error handling for file operations and network issues

## Learning Outcomes

This project demonstrates:
- Low-level networking in Rust
- HTTP protocol implementation
- Concurrent programming patterns
- File system operations
- Error handling in network applications
- Request/Response parsing and formatting

## Contributing

Feel free to submit issues and enhancement requests!