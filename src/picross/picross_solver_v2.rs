use crate::{
    game_board::{GameBoard, TileState},
    render::{GameState, PicrossFrame},
};

use super::{picross_solver_trait::PicrossSolver, PicrossGame};

struct V2Iterator {
    current_board: Option<GameBoard>,
    game: PicrossGame,
    is_complete: bool,
}
impl Iterator for V2Iterator {
    type Item = PicrossFrame;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_complete {
            return None;
        };
        if let None = self.current_board {
            let mut current_board = self
                .game
                .get_partial_board_from_columns(Some(
                    self.game.get_partial_board_from_rows(None).ok()?,
                ))
                .ok()?;
            self.current_board = Some(current_board);
        }
        let new_board = self
            .game
            .get_partial_board_from_columns(Some(
                self.game
                    .get_partial_board_from_rows(self.current_board.clone())
                    .ok()?,
            ))
            .ok()?;
        if &self.current_board.unwrap() == new_board {
            if new_board.0.iter().all(|row| {
                row.0
                    .iter()
                    .all(|tile| !matches!(tile, TileState::Undetermined))
            }) {
                let frame =
                    PicrossFrame::new(self.game.clone(), new_board, GameState::Complete).ok()?;
                self.is_complete = true;
                return Some(frame);
            } else {
                let frame =
                    PicrossFrame::new(self.game.clone(), new_board, GameState::Invalid).ok()?;
                self.is_complete = true;
                return Some(frame);
            }
        } else {
            self.current_board = Some(new_board.clone());
            let frame = PicrossFrame::new(
                self.game.clone(),
                self.current_board.unwrap().clone(),
                GameState::InProgress,
            )
            .ok()?;
            Some(frame)
        }
    }
}

pub struct PicrossSolverV2(pub PicrossGame);

impl PicrossSolver for PicrossSolverV2 {
    fn solve(&self) -> Result<PicrossFrame, &'static str> {
        let iter = self.iter();
        match iter.last() {
            Some(_) => todo!(),
            None => todo!(),
        }
    }

    fn from_game(game: PicrossGame) -> Self {
        Self(game)
    }
    fn set_game(&mut self, game: PicrossGame) {
        self.0 = game;
    }

    type Iter = V2Iterator;

    fn iter(&self) -> Self::Iter {
        Self::Iter {
            current_board: None,
            game: self.0.clone(),
            is_complete: false,
        }
    }
}
