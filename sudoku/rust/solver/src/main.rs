extern crate csv;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io;

#[derive(Debug,Deserialize)]
struct Record {
    quizzes: String,
    solutions: String,
}

struct Board {
    brd: [u8; 81]
}

fn example() -> Result<Box<Vec<Board>>, Box<Error>> {
    let mut boards: Vec<Board> = vec![];

    let mut num = 0;

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
            //print!("{}", c);
            board.brd[index] = (c as u8) - 32;
            index += 1;
        }
        boards.insert(0, board);
        //println!("x");

        num += 1;

        if num == 10 {
            break;
        }
    }
    Ok(Box::new(boards))
}

fn main() {
    let r = example();

    let f = match r {
        Ok(result) => result,
        Err(err) => {
            panic!("There was a problem! {:?}", err)
        },
    };

    for board in *f {
        println!("{}", board.brd[0]);
    }
}