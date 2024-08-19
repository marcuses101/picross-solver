use game_board::*;
use picross::PicrossGame;
use std::{env, error, time::Instant};

mod game_board;
mod iterators;
mod picross;

#[allow(dead_code)]
const ROW_INPUT: &str = "0,0,0,1,0";
#[allow(dead_code)]
const COLUMN_INPUT: &str = "0,1,0,0,0";

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut args = env::args();
    let _ = args.next();
    let row_rules = args.next().expect("row argument is required");
    let column_rules = args.next().expect("column argument is required");
    let game = PicrossGame::from_rules(&row_rules, &column_rules)?;

    let start = Instant::now();
    let answer = game.solve_v2();
    let duration = start.elapsed();
    println!("Time elapsed in solve_v2 is: {:?}", duration);
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
