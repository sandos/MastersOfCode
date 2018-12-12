extern crate csv;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io;
use std::process;

#[derive(Debug,Deserialize)]
struct Record {
    quizzes: String,
    solutions: String,
}

struct Board {
    brd: [u8; 81]
}

fn example() -> Result<(), Box<Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.

        let record: Record = result?;

        //println!("{:?}", record);
        let mut board = Board {
            brd: [0; 81]
        };
        let mut index: usize = 0;
        for c in record.quizzes.chars() {
            print!("{}", c);
            board.brd[index] = (c as u8) - 32;
            index += 1;
        }
        println!("");
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
