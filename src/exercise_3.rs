use bson::from_slice;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use bson::{from_reader, to_bson, Bson, ser::to_vec, from_bson, Document};

#[derive(Debug, Serialize, Deserialize)]
struct Move {
    distance: u16,
    direction: String,
}

fn main() -> std::io::Result<()> {

    //serialize
    let mut f = File::create("moves.bson")?;

    for i in 1..=1000 {
        let m: Move = Move {
            distance: i,
            direction: String::from("North"),
        };

        let ser: Bson = to_bson(&m).unwrap();

        if let Bson::Document(document) = ser {
            let bson_bytes = to_vec(&document).unwrap();
            f.write_all(&bson_bytes)?;
        } else {
            eprintln!("Error writing to bson file");
        }
    }

    // deserialize
    let mut file = File::open("moves.bson")?;
    
    loop {
        let maybe_doc: bson::de::Result<Document> = from_reader(&mut file);

        match maybe_doc {
            Ok(doc) => {
                let point: Move = from_bson(Bson::Document(doc)).expect("Failed to deserialize");
                println!("{:?}", point);
            },
            Err(e) => {
                println!("{:?}", e);
                break;
            },
        }
    }

    let mut bson_vec: Vec<Vec<u8>> = vec![]; 
    
    // serialize to vec
    for i in 1..=1000 {
        let m: Move = Move {
            distance: i,
            direction: String::from("SOUTH"),
        };

        let ser: Bson = to_bson(&m).unwrap();

        if let Bson::Document(document) = ser {
            let serialized_bytes = bson::ser::to_vec(&document).unwrap();
            bson_vec.push(serialized_bytes);
        }
    }
    println!("{:?}", bson_vec);

    let mut moves: Vec<Move> = vec![];
    
    for serialized_move in bson_vec {
        match from_slice::<Document>(&serialized_move) {
            Ok(document) => {
                match bson::from_bson::<Move>(Bson::Document(document)) {
                    Ok(move_instance) => moves.push(move_instance),
                    Err(e) => eprintln!("Error deserializing BSON document: {}", e),
                }
            },
            Err(e) => eprintln!("Error converting bytes to bson document: {:?}", e),
        }
    }
    println!("{:?}", moves);

    Ok(())
}

