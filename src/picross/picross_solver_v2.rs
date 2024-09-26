use crate::{
    game_board::TileState,
    render::{GameState, PicrossFrame},
};

use super::{PicrossGame, PicrossSolver};

pub struct PicrossSolverV2(pub PicrossGame);

impl PicrossSolver for PicrossSolverV2 {
    fn solve(&self) -> Result<PicrossFrame, &'static str> {
        let mut current_board = self
            .0
            .get_partial_board_from_columns(Some(self.0.get_partial_board_from_rows(None)?))?;
        loop {
            let new_board = self.0.get_partial_board_from_columns(Some(
                self.0
                    .get_partial_board_from_rows(Some(current_board.clone()))?,
            ))?;
            if current_board == new_board {
                if new_board.0.iter().all(|row| {
                    row.0
                        .iter()
                        .all(|tile| !matches!(tile, TileState::Undetermined))
                }) {
                    let frame = PicrossFrame::new(self.0.clone(), new_board, GameState::Complete)?;
                    return Ok(frame);
                } else {
                    let frame = PicrossFrame::new(self.0.clone(), new_board, GameState::Invalid)?;
                    return Ok(frame);
                }
            } else {
                current_board = new_board;
            }
        }
    }

    fn from_game(game: PicrossGame) -> Self {
        Self(game)
    }
    fn set_game(&mut self, game: PicrossGame) {
        self.0 = game;
    }
}
