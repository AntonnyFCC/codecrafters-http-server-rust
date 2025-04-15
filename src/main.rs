use std::io::{Read, Write};
#[allow(unused_imports)]
use std::net::TcpListener;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut buffer = [0; 1024];

                match stream.read(&mut buffer) {
                    Ok(_) => {
                        let request = String::from_utf8_lossy(&buffer);
                        let request_line = request.lines().next().unwrap_or("");

                        let response = match request_line {
                            "GET / HTTP/1.1" | "GET /index.html HTTP/1.1" => {
                                "HTTP/1.1 200 OK\r\n\r\n"
                            }
                            _ => {
                                "HTTP/1.1 404 Not Found\r\n\r\n"
                            }
                            
                        };
                        stream.write(response.as_bytes()).unwrap();
                    }
                    Err(e) => {
                        println!("error: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
