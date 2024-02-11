use serde::{Deserialize, Serialize};
use ron::ser::to_writer; 
use ron::de::from_bytes;

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
    // serialize to buffer (json)
    let ser: Vec<u8> = serde_json::to_vec(&a)?;
    println!("Serialized buffer with JSON: {:?}", ser);

    // deserialize from buffer
    let deser: Move = serde_json::from_slice(&ser)?;
    println!("Deserialized json from buffer: {:?}", deser);

    // serialize to buffer (ron)
    let mut v: Vec<u8> = vec![];
    to_writer(&mut v, &a).unwrap();
    println!("Serialized buffer with RON: {:?}", v);

    // deserialize from buffer
    let deser: Move = from_bytes(&v).unwrap();
    println!("Deserialized buffer with RON: {:?}", deser);

    Ok(())
}
