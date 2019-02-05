extern crate csv;
extern crate lz4;
#[macro_use]
extern crate serde_derive;

pub mod majs {

use std::error::Error;
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::{Path, PathBuf};

use lz4::{Decoder, EncoderBuilder};

#[derive(Debug,Deserialize)]
struct Record {
    quizzes: String,
    solutions: String,
}

pub struct Board {
    pub brd: [u8; 81]
}

pub fn example() -> Result<Box<Vec<Board>>, Box<dyn Error>> {
    let mut boards: Vec<Board> = vec![];

    let mut num = 0;

    let input_file = File::open("C:/Users/SEJOBAC8/projs/MastersOfCode/sudoku/sudoku.lz4")?;
    let mut decoder = Decoder::new(input_file)?;

    let f = BufReader::new(decoder);
    let mut sum: i64 = 0;
    for line in f.lines() {
        let line: String = line.unwrap();
        for byte in line.bytes() {
            //print!("{}", byte);
            sum += byte as i64;
        }
        //print!("{}\n", &line[0..1]);
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
    let r = majs::example();

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