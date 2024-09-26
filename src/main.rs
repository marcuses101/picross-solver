use app::App;
use clap::Parser;
use std::error;

mod app;
mod game_board;
mod iterators;
mod picross;
mod render;

/// A program to solve Picross Puzzles
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// set the solver version
    #[arg(short, long)]
    solver_version: Option<String>,

    #[arg(short, long)]
    puzzle: Option<String>,

    #[arg(short, long)]
    image: Option<String>,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Cli::parse();

    let mut app = App::new();

    if let Some(version) = args.solver_version {
        app.change_version(&version)?;
    }

    if let Some(puzzle) = args.puzzle {
        app.select_game_from_puzzles(&puzzle)?;
        app.solve()?;
        return Ok(());
    }
    if let Some(image_file) = args.image {
        app.select_game_from_text_image(&image_file)?;
        app.solve()?;
        return Ok(());
    }
    println!("Either puzzle or image argument must be provided");
    Ok(())
}
