use game_board::*;
use picross::PicrossGame;
use std::env;

mod game_board;
mod iterators;
mod picross;

fn main() {
    let mut args = env::args();
    let _ = args.next();
    let row_rules = args.next().expect("row argument is required");
    let column_rules = args.next().expect("column argument is required");
    let mut game = PicrossGame::from_rules(&row_rules, &column_rules)
        .expect("failed to build game from the provided arguments");
    let answer = game.solve();
    match answer {
        Ok(board) => {
            println!("\n\n{}\n\n", board.render());
        }
        Err(_) => eprintln!("failed to solve the puzzle"),
    }
}
