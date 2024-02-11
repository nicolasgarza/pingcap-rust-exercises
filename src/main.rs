use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Move {
    distance: u8,
    direction: String,
}


fn main() -> std::io::Result<()> {
    let a = Move {
        distance: 5,
        direction: String::from("Up"),
    };
    let ser = serde_json::to_string(&a)?;
    let mut file = File::create("serialize_output.txt")?;

    match file.write_all(ser.as_str().as_bytes()) {
        Ok(_) => println!("Serialization to file successful"),
        Err(e) => println!("{}", e),
    };

    let file_as_str = fs::read_to_string("serialize_output.txt")?;
    let b: Move = serde_json::from_str(&file_as_str)?;

    println!("{:?}", b);

    Ok(())
}
