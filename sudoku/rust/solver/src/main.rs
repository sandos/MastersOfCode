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



fn example() -> Result<(), Box<Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;

        //println!("{:?}", record);
        for c in record.quizzes.chars() {
            print!("{}", c);
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
