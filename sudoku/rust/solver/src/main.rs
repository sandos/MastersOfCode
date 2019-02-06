extern crate csv;
extern crate lz4;
#[macro_use]
extern crate serde_derive;

pub mod majs {

use std::error::Error;
use std::io::{BufRead, BufReader};
use std::fs::File;

use lz4::{Decoder};

#[derive(Debug,Deserialize)]
struct Record {
    quizzes: String,
    solutions: String,
}

pub struct Board {
    pub brd: [u8; 81]
}

pub fn pars() -> Result<Box<Vec<Board>>, Box<dyn Error>> {
    let mut boards: Vec<Board> = vec![];

    //let mut num = 0;

    let input_file = File::open("C:/Users/SEJOBAC8/projs/MastersOfCode/sudoku/sudoku.lz4")?;
    let decoder = Decoder::new(input_file)?;

    let mut f = BufReader::new(decoder);
    let mut line = String::new();
    while f.read_line(&mut line).unwrap() > 0 {
        if line.len() < 81 {
            continue;
        }
        //print!("{}\r\n", line);
        let mut it = line.bytes();
        let mut board = Board {
            brd: [0; 81]
        };
        for x in 0..81 {
            board.brd[x] = it.next().unwrap();

        }
        //it.skip(1);

        boards.push(board);
        line.clear();
    }

    /*let mut rdr = csv::Reader::from_reader(io::stdin());
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
            //board.brd[index] = (c as u8) - 32;
            index += 1;
        }
        boards.insert(0, board);
        //println!("x");

        num += 1;

        if num == 10000 {
            break;
        }
    }
    */
    Ok(Box::new(boards))
    //return Result::Err(Box::new(Error("asdasd".into())));
    //Err::new("adsasd");
}
}
fn main() {
    let r = majs::parse();

    let f = match r {
        Ok(result) => result,
        Err(err) => {
            panic!("There was a problem! {:?}", err)
        },
    };

    // for board in *f {
    //     println!("{}", board.brd[0]);
    // }
}