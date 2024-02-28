use std::net::{TcpStream, TcpListener};
use std::io::{Write, Read};
use serde::{Serialize, Deserialize};
use serde_json;

// SERVER

#[derive(Debug, Serialize, Deserialize)]
enum RedisCommand {
    Ping,
    PingMsg(String),
}

#[derive(Debug, Serialize, Deserialize)]
enum RedisResponse {
    Pong,
    PongMsg(String),
    Error(String),
}

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
                let response_ser = serde_json::to_string(&response).unwrap();
                let _ = stream.write_all(response_ser.as_bytes());

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

fn validate_req(req: &str) -> RedisResponse {
    let command: Result<RedisCommand, serde_json::Error> = serde_json::from_str(req);

    match command {
        Ok(comm) => {
            match comm {
                RedisCommand::Ping => return RedisResponse::Pong,
                RedisCommand::PingMsg(msg) => return RedisResponse::PongMsg(msg),
            }
        },
        Err(e) => return RedisResponse::Error(e.to_string())
    }
}