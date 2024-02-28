use std::net::{TcpStream, TcpListener};
use std::io::{Write, Read};

fn main() {
    start_server();
}

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        buffer.fill(0);
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
                let response = validate_req(request);

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

fn validate_req(req: &str) -> String {
    if req == "*1\r\n$4\r\nPING\r\n" {
        return String::from("+PONG\r\n")
    }
    
    // string is either in the format *2\r\n$4\r\nPING\r\n$5\r\nhello\r\n or its invalid
    let mut parts = req.split("\r\n");
    let first = parts.next();
    let second = parts.next();
    let third = parts.next();

    if first != Some("*2") || second != Some("$4") || third != Some("PING") {
        return String::from("-ERR Invalid starting sequence\r\n");
    }

    let len_str = parts.next().unwrap_or("");
    let res: Option<i32> = len_str.trim_start_matches('$').parse().ok();

    if res.is_none() {
        return String::from("-ERR invalid length input for 2nd array object\r\n");
    }

    let ret_str = parts.next().unwrap_or("");
    if ret_str.len() as i32 != res.unwrap() {
        return String::from("-ERR Error parsing command: incorrect string len\r\n");
    }

    if parts.next() != Some("") || parts.next().is_some() {
        return String::from("-ERR unknown command or extra arguments provided\r\n")
    }

    format!("+{}\r\n", ret_str)
}