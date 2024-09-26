use std::{fs::read_to_string, time::Instant};

use crate::{
    picross::{
        picross_solver_v1::PicrossSolverV1, picross_solver_v2::PicrossSolverV2,
        picross_solver_v3::PicrossSolverV3, PicrossGame, PicrossSolver,
    },
    render::PicrossFrame,
};

pub enum SolverVersion {
    One(PicrossSolverV1),
    Two(PicrossSolverV2),
    Three(PicrossSolverV3),
}

impl SolverVersion {
    fn solve(&mut self) -> Result<PicrossFrame, &str> {
        match self {
            SolverVersion::One(solver) => solver.solve(),
            SolverVersion::Two(solver) => solver.solve(),
            SolverVersion::Three(solver) => solver.solve(),
        }
    }
    fn set_game(&mut self, game: PicrossGame) {
        match self {
            SolverVersion::One(solver) => solver.set_game(game),
            SolverVersion::Two(solver) => solver.set_game(game),
            SolverVersion::Three(solver) => solver.set_game(game),
        }
    }
}

pub struct App {
    pub version: SolverVersion,
    game: Option<PicrossGame>,
}

impl App {
    pub fn new() -> Self {
        let default_game = PicrossGame::default();
        App {
            version: SolverVersion::Three(PicrossSolverV3::from_game(default_game)),
            game: None,
        }
    }
    pub fn solve(&mut self) -> Result<(), String> {
        let game = self
            .game
            .as_ref()
            .ok_or("Picross Game not set prior to solving")?
            .clone();
        self.version.set_game(game);
        let start = Instant::now();
        let result = self.version.solve()?;
        let duration = start.elapsed();
        let rendered_result = result.render();
        println!(
            "{}{}{}\n\nElapsed time: {:?}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            rendered_result,
            duration
        );
        Ok(())
    }
    pub fn change_version(&mut self, version: &str) -> Result<(), String> {
        match version {
            "v1" => {
                self.version =
                    SolverVersion::One(PicrossSolverV1(PicrossGame::from_rules("0", "0")?))
            }
            "v2" => {
                self.version =
                    SolverVersion::Two(PicrossSolverV2(PicrossGame::from_rules("0", "0")?))
            }
            "v3" => {
                self.version =
                    SolverVersion::Three(PicrossSolverV3(PicrossGame::from_rules("0", "0")?))
            }
            _ => {
                return Err(
                    "invalid version selection\n Available versions are: v1, v2, v3".to_string(),
                )
            }
        }
        Ok(())
    }
    pub fn select_game_from_text_image(&mut self, text_file_name: &str) -> Result<(), String> {
        let filepath = format!("./text_images/{}.txt", text_file_name);
        let text_render =
            read_to_string(&filepath).map_err(|_| format!("could not read \"{}\"", filepath))?;
        let game = PicrossGame::from_text_render(&text_render)?;
        self.game = Some(game);
        Ok(())
    }
    pub fn select_game_from_puzzles(&mut self, puzzle_name: &str) -> Result<(), String> {
        let filepath = format!("./puzzles/{}.pic", puzzle_name);
        let rules_file_content =
            read_to_string(&filepath).map_err(|_| format!("could not read \"{}\"", filepath))?;
        let game = PicrossGame::from_rules_file_string(&rules_file_content)?;
        self.game = Some(game);
        Ok(())
    }
}
