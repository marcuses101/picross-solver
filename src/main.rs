use app::App;
use std::error;

mod app;
mod game_board;
mod iterators;
mod picross;
mod render;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut app = App::new();
    let _ = app.run()?;
    Ok(())
}
