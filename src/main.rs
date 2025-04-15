use std::io::{Read, Write};
use regex::Regex;
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
                            "GET / HTTP/1.1" => {
                                "HTTP/1.1 200 OK\r\n\r\n".to_string()
                            }
                            request_line if request_line.starts_with("GET /echo") => {
                                let re = Regex::new(r"GET /echo/(.*) HTTP/1.1").unwrap();
                                if let Some(caps) = re.captures(request_line) {
                                    if let Some(param) = caps.get(1) {
                                        let param = param.as_str();
                                        let param_length = param.len();
                                        format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", param_length, param)
                                    } else {
                                        "HTTP/1.1 400 Bad Request\r\n\r\n".to_string()
                                    }
                                } else {
                                    "HTTP/1.1 400 Bad Request\r\n\r\n".to_string()
                                }
                            }
                            _ => {
                                "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
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
