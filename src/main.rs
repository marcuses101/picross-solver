use game_board::*;
use picross::PicrossGame;
use std::{env, error, time::Instant};

mod game_board;
mod iterators;
mod picross;

fn main() -> Result<(), Box<dyn error::Error>> {
    let path = "./puzzles/136.pic";
    let game = PicrossGame::from_filepath(path)?;
    let mut args = env::args();
    args.next();
    let version = args.next().unwrap_or("3".into()).parse::<usize>()?;
    let start = Instant::now();
    let answer = match version {
        1 => game.solve_v1(),
        2 => game.solve_v2(),
        _ => game.solve_v3(),
    };
    let duration = start.elapsed();

    println!("Time elapsed in solve_v{} is: {:?}", version, duration);
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
