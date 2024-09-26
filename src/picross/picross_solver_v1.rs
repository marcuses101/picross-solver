use std::slice::Iter;

use crate::{
    game_board::{GameBoard, GameBoardRow},
    iterators::PicrossLineIter,
    picross::{validate_board, BoardState, LineRule},
    render::{GameState, PicrossFrame},
};

use super::{PicrossGame, PicrossSolver};

pub struct PicrossSolverV1(pub PicrossGame);

impl PicrossSolver for PicrossSolverV1 {
    fn set_game(&mut self, game: PicrossGame) {
        self.0 = game;
    }
    fn solve(&self) -> Result<PicrossFrame, &'static str> {
        let initial_board = GameBoard::new(self.0.width(), self.0.height());
        let frame =
            PicrossFrame::new(self.0.clone(), initial_board.clone(), GameState::InProgress)?;
        frame.print(true);
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

        let mut last_board: Option<GameBoard> = None;

        while let Some(StackEntry {
            mut row_iter,
            row_layout_iter,
            board,
        }) = stack.pop()
        {
            last_board = Some(board.clone());

            let mut render_board = board.clone();
            let missing_rows = self.0.height() - render_board.height();
            if missing_rows > 0 {
                for _ in 0..missing_rows {
                    render_board.0.push(GameBoardRow::new(self.0.width()));
                }
            }
            let frame = PicrossFrame::new(self.0.clone(), render_board, GameState::InProgress)?;
            frame.print(false);

            match validate_board(&self.0, &board)? {
                BoardState::Invalid => (),
                BoardState::Complete(complete_board) => {
                    return Ok(PicrossFrame::new(
                        self.0.clone(),
                        complete_board,
                        GameState::Complete,
                    )?)
                }
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
        let failed_frame = PicrossFrame::new(
            self.0.clone(),
            last_board.ok_or("no last board")?.clone(),
            GameState::Invalid,
        )?;
        Ok(failed_frame)
    }

    fn from_game(game: PicrossGame) -> Self {
        Self(game)
    }
}
