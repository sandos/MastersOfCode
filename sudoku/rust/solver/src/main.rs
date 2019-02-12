extern crate csv;
extern crate lz4;
#[macro_use]
extern crate serde_derive;

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
    pub brd: [u16; 81]
}

pub fn parse() -> Result<Box<Vec<Board>>, Box<dyn Error>> {
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
            let v = it.next().unwrap() - 48;
            if v > 0 {
                board.brd[x] = 1 << (v-1);
                //print!("{} {:09b} ", v, board.brd[x])
            }

        }
        //it.skip(1);

        boards.push(board);
        line.clear();
    }

    Ok(Box::new(boards))
}

pub fn solve(board: Board) -> Result<u32, Box<Error>> {

    //Create constraints for every cell
    // bit 0 - 8 means 1...9 is still possible
    let mut constraints: [u16; 81] = [0; 81];
    let mut counts: [u32; 81] = [0; 81];

    for row in 0..9 {
        let mut mask: u16 = 0;
        let addr = row * 9;
        for col in 0..9 {
            //print!("{} {} {}\n", row, col, addr);
            mask |= board.brd[addr+col];
        }
        for col in 0..9 {
            constraints[addr+col] = mask;
        }
        //print!("{:08b}\n", mask);
    }

    for col in 0..9 {
        let mut mask: u16 = 0;
        for row in 0..9 {
            mask |= board.brd[col + row*9];
        }
        for row in 0..9 {
            constraints[col + row*9] |= mask;

            //Population count here
            counts[col + row*9] = mask.count_ones();
        }
    }

    let mut max = 0;
    let mut pos = 0;
    for a in 0..81 {
        if max < counts[a] {
            max = counts[a];
            pos = a;
        }
    }

    print!("{}:{} ", pos, max);

    // for row in 0..9 {
    //     for col in 0..9 {
    //         print!("{:09b}|{:3x}|{} ", constraints[row*9+col], board.brd[row*9+col], counts[row*9+col]);
    //     }
    //     print!("\n");
    // }
    // print!("\n");



    return Result::Ok(0);
}

fn main() {
    let r = parse();

    let f = match r {
        Ok(result) => result,
        Err(err) => {
            panic!("There was a problem! {:?}", err)
        },
    };

    for board in *f {
        let _ = match solve(board) {
            Ok(result) => result,
            Err(err) => {
                panic!("There was a problem {:?}", err)
            }
        };
    }
}