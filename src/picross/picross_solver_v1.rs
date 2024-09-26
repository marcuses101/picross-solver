use std::slice::Iter;

use crate::{iterators::PicrossLineIter, picross::{validate_board, BoardState, LineRule}, game_board::GameBoard};

use super::{PicrossGame, PicrossSolver};

pub struct PicrossSolverV1(pub PicrossGame);

impl PicrossSolver for PicrossSolverV1 {
    fn set_game(&mut self, game: PicrossGame) {
        self.0 = game;
    }
    fn solve(&self) -> Result<GameBoard, &'static str> {
       let width = self.0.columns.0.len();

        struct StackEntry<'a> {
            row_iter: Iter<'a, LineRule>,
            row_layout_iter: Option<PicrossLineIter<'a>>,
            board: GameBoard,
        }
        let mut stack = vec![StackEntry {
            row_iter: self.0.rows.0.iter(),
            row_layout_iter: None,
            board: GameBoard(vec![]),
        }];

        while let Some(StackEntry {
            mut row_iter,
            row_layout_iter,
            board,
        }) = stack.pop()
        {
            match validate_board(&self.0, &board)? {
                BoardState::Invalid => (),
                BoardState::Complete(complete_board) => return Ok(complete_board),
                BoardState::InProgress => {
                    let next_row_layout_iter = row_iter
                        .next()
                        .map(|row_rule| PicrossLineIter::new(&row_rule.0, width));
                    match row_layout_iter {
                        Some(row_layout_iter) => {
                            for row_layout in row_layout_iter {
                                let mut new_board = board.clone();
                                new_board.0.push(row_layout);
                                stack.push(StackEntry {
                                    row_iter: row_iter.clone(),
                                    row_layout_iter: next_row_layout_iter.clone(),
                                    board: new_board,
                                });
                            }
                        }
                        None => {
                            stack.push(StackEntry {
                                row_iter: row_iter.clone(),
                                row_layout_iter: next_row_layout_iter.clone(),
                                board,
                            });
                        }
                    }
                }
            }
        }

        Err("no solution found")
    }

    fn from_game(game: PicrossGame) -> Self {
        Self(game)
    }
}
