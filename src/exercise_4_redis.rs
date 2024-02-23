use std::net::{TcpStream, TcpListener};
use std::io::{Write, Read};

fn main() {
    start_server();
}

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    break;
                }
                let request = match std::str::from_utf8(&buffer[..size]) {
                    Ok(req) => req,
                    Err(_) => {
                        eprintln!("Invalid request");
                        return;
                    }
                };
                let response: &str = validate_req(request);

                let _ = stream.write_all(response.as_bytes());

            },
            Err(e) => {
                eprintln!("Failed to read from connection: {}", e);
            },
        } {}
    }
}

fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_stream(stream);
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn validate_req(req: &str) -> &str {
    if !req.starts_with("PING") {
        return "-ERR Not a valid PING command \r\n"
    }

    if req.len() == 5 {
        "PONG"
    } else {
        &req[5..]
    }
}