use std::collections::VecDeque;

use crate::{
    game_board::{GameBoard, GameBoardRow, TileState},
    iterators::PicrossLineIter,
};

use super::{PicrossGame, PicrossSolver};

pub struct PicrossSolverV3(pub PicrossGame);

impl PicrossSolver for PicrossSolverV3 {
    fn solve(&self) -> Result<GameBoard, &'static str> {
        let mut board = GameBoard::new(self.0.width(), self.0.height());
        #[derive(Debug, PartialEq)]
        enum ToCheck {
            Row,
            Column,
        }
        let mut queue: VecDeque<(ToCheck, usize)> = VecDeque::new();
        (0..self.0.width()).for_each(|col_index| queue.push_back((ToCheck::Column, col_index)));
        (0..self.0.height()).for_each(|row_index| queue.push_back((ToCheck::Row, row_index)));
        let mut loop_count = 0;
        // populate the rows initially
        while let Some((board_axis, index)) = queue.pop_front() {
            loop_count += 1;
            println!(
                "loop count: {} board axis: {:?} index:{}",
                loop_count, board_axis, index
            );
            match board_axis {
                ToCheck::Row => {
                    let row_index = index;
                    // maybe rethink this...
                    let rules = &self.0.rows.0.get(index).ok_or("failed to get row rules")?;
                    let mut line_iter = PicrossLineIter::new(&rules.0, self.0.width());
                    let board_row = board
                        .0
                        .get_mut(row_index)
                        .ok_or("failed to get board row")?;
                    let solved = line_iter.get_partially_solved_line(Some(board_row))?;

                    for (col_index, tile) in board_row.0.iter_mut().enumerate() {
                        let solved_tile = &solved.0[col_index];
                        match (&tile, solved_tile) {
                            (TileState::Undetermined, TileState::Filled) => {
                                if !queue.contains(&(ToCheck::Column, col_index)) {
                                    queue.push_back((ToCheck::Column, col_index));
                                }
                                *tile = TileState::Filled
                            }
                            (TileState::Undetermined, TileState::Empty) => {
                                if !queue.contains(&(ToCheck::Column, col_index)) {
                                    queue.push_back((ToCheck::Column, col_index));
                                }
                                queue.push_back((ToCheck::Column, col_index));
                                *tile = TileState::Empty
                            }
                            _ => (),
                        }
                    }
                }
                ToCheck::Column => {
                    let col_index = index;
                    let rules = self
                        .0
                        .columns
                        .0
                        .get(col_index)
                        .ok_or("failed to get col rules")?;
                    let mut line_iter = PicrossLineIter::new(&rules.0, self.0.height());
                    let board_col = GameBoardRow(board.get_column(col_index));
                    let solved_col = line_iter.get_partially_solved_line(Some(&board_col))?;
                    for (row_index, tile) in board_col.0.iter().enumerate() {
                        let solved_tile = &solved_col
                            .0
                            .get(row_index)
                            .ok_or("failed to get solved column tile")?;
                        match (&tile, solved_tile) {
                            (TileState::Undetermined, TileState::Filled) => {
                                if !queue.contains(&(ToCheck::Row, row_index)) {
                                    queue.push_back((ToCheck::Row, row_index));
                                }
                                let _ = &board.set_tile(col_index, row_index, TileState::Filled)?;
                            }
                            (TileState::Undetermined, TileState::Empty) => {
                                if !queue.contains(&(ToCheck::Row, row_index)) {
                                    queue.push_back((ToCheck::Row, row_index));
                                }
                                let _ = &board.set_tile(col_index, row_index, TileState::Empty)?;
                            }
                            _ => (),
                        }
                    }
                }
            }
        }

        if board.0.iter().all(|row| {
            row.0
                .iter()
                .all(|tile| !matches!(tile, TileState::Undetermined))
        }) {
            Ok(board)
        } else {
            eprintln!("\n-----\nCould not determine:\n{}\n------", board.render());
            Err("Not Complete")
        }
    }

    fn from_game(game: PicrossGame) -> Self {
        Self(game)
    }

    fn set_game(&mut self, game: PicrossGame) {
        self.0 = game;
    }
}
