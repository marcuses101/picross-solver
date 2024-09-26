use crate::picross::{
    picross_solver_v1::PicrossSolverV1, picross_solver_v2::PicrossSolverV2,
    picross_solver_v3::PicrossSolverV3, PicrossGame, PicrossSolver,
};

pub enum AppScreen {
    Welcome,
    Solve,
}

pub enum SolverVersion {
    One(PicrossSolverV1),
    Two(PicrossSolverV2),
    Three(PicrossSolverV3),
}

impl SolverVersion {
    fn solve(&mut self) -> Result<crate::game_board::GameBoard, &str> {
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
    pub screen: AppScreen,
    pub version: SolverVersion,
    game: Option<PicrossGame>,
}

impl App {
    pub fn new() -> Self {
        let default_game = PicrossGame::default();
        App {
            screen: AppScreen::Welcome,
            version: SolverVersion::Three(PicrossSolverV3::from_game(default_game)),
            game: None,
        }
    }
    pub fn run(&mut self) -> Result<(), String> {
        match self.screen {
            AppScreen::Welcome => {
                todo!()
            }
            AppScreen::Solve => {
                let game = self
                    .game
                    .as_ref()
                    .ok_or("Picross Game not set prior to solving")?
                    .clone();
                self.version.set_game(game);
                let result = self.version.solve();
            }
        }
        Ok(())
    }
    fn change_version(&mut self, version: SolverVersion) {
        self.version = version;
    }
    fn select_game_from_text_image() {
        todo!();
    }
    fn select_game_from_puzzles() {
        todo!()
    }
}
