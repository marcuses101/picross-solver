use game_board::*;
use picross::PicrossGame;
use std::{env, error, time::Instant};

mod game_board;
mod iterators;
mod picross;

#[allow(dead_code)]
const ROW_INPUT: &str = "4 2,1 5,1 1 1 4,1 1 1 1,1 1 1 1 1,5 1 1 1,3 3 1 1,3 5,3 3,5";
#[allow(dead_code)]
const COLUMN_INPUT: &str = "2,2 2,2 2,1 2 2,1 2,2 1,1 4,10,1 4,2 1,1 2,1 2 2,2 2,2 2,2";

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut args = env::args();
    let _ = args.next();
    let row_rules = args.next().expect("row argument is required");
    let column_rules = args.next().expect("column argument is required");
    let game = PicrossGame::from_rules(&row_rules, &column_rules)?;

    let start = Instant::now();
    let answer = game.solve();
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    match answer {
        Ok(board) => {
            println!("\n\n{}\n\n", board.render());
            Ok(())
        }
        Err(e) => {
            eprintln!("failed to solve the puzzle");
            Err(e.into())
        }
    }
}
