use std::net::TcpStream;
use std::io::{Write, Read};
use serde::{Deserialize, Serialize};
use serde_json;

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

fn main() -> std::io::Result<()>{

    let mut stream = TcpStream::connect("127.0.0.1:6379")?;

    let command = RedisCommand::Ping;
    let ser_command = serde_json::to_string(&command).unwrap();

    stream.write_all(ser_command.as_bytes())?;

    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer)?;
    let recieved = std::str::from_utf8(&buffer[..size]).unwrap();
    let response: RedisResponse = serde_json::from_str(recieved).unwrap();
    println!("Recieved data: {:?}", response);


    Ok(())
}