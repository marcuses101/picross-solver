use crate::{game_board::GameBoard, iterators::PicrossRowIter, GameBoardRow};
use std::{slice::Iter, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct RowColumnRule(Vec<usize>);

impl FromStr for RowColumnRule {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rule: Result<Vec<usize>, _> = s.trim().split(' ').map(|part| part.parse()).collect();
        match rule {
            Ok(rule) => Ok(Self(rule)),
            _ => Err("failed to parse"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct RowColumnRules(Vec<RowColumnRule>);

impl FromStr for RowColumnRules {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules_result: Result<Vec<RowColumnRule>, _> =
            s.trim().split(',').map(RowColumnRule::from_str).collect();
        match rules_result {
            Ok(rules) => Ok(Self(rules)),
            Err(_) => Err("failed to parse"),
        }
    }
}

#[derive(Debug)]
pub struct PicrossGame {
    rows: RowColumnRules,
    columns: RowColumnRules,
}

#[derive(Debug)]
enum BoardState {
    Complete(GameBoard),
    InProgress,
    Invalid,
}

#[derive(PartialEq, Debug)]
enum ChunksValidation {
    Valid,
    InProgress,
    Invalid,
}

fn validate_chunks(rule: &RowColumnRule, chunks: Vec<usize>) -> ChunksValidation {
    if rule.0 == chunks {
        return ChunksValidation::Valid;
    }
    if chunks.len() > rule.0.len() {
        return ChunksValidation::Invalid;
    }
    for (chunk, rule) in chunks.iter().zip(&rule.0) {
        if chunk > rule {
            return ChunksValidation::Invalid;
        }
    }
    ChunksValidation::InProgress
}

fn validate_board(game: &PicrossGame, board: &GameBoard) -> Result<BoardState, &'static str> {
    let mut board_state = BoardState::Complete(board.clone());
    if board.0.len() > game.rows.0.len() {
        return Ok(BoardState::Invalid);
    }
    for (index, column_rule) in game.columns.0.iter().enumerate() {
        let column_chunks = board.get_column_chunks(index)?;
        match validate_chunks(column_rule, column_chunks) {
            ChunksValidation::Valid => (),
            ChunksValidation::InProgress => board_state = BoardState::InProgress,
            ChunksValidation::Invalid => return Ok(BoardState::Invalid),
        }
    }
    Ok(board_state)
}

#[allow(dead_code)]
impl PicrossGame {
    pub fn width(&self) -> usize {
        self.columns.0.len()
    }
    pub fn height(&self) -> usize {
        self.rows.0.len()
    }
    pub fn from_rules(row_rules: &str, column_rules: &str) -> Result<Self, String> {
        let rows = RowColumnRules::from_str(row_rules)?;
        let columns = RowColumnRules::from_str(column_rules)?;
        let row_sum: usize = rows.0.iter().map(|r| r.0.iter().sum::<usize>()).sum();
        let col_sum: usize = columns.0.iter().map(|r| r.0.iter().sum::<usize>()).sum();
        if row_sum != col_sum {
            eprintln!(
                "Row Count:{} Row Sum:{}\nCol Count:{} Col Sum:{}",
                rows.0.len(),
                row_sum,
                columns.0.len(),
                col_sum
            );
            return Err(format!(
                "Invalid Rules: Sum of row rules must equal sum of col rules.\nRow sum:{}\nColumn sum:{}"
            ,row_sum,col_sum));
        }
        Ok(Self { rows, columns })
    }

    fn get_row_iter(&self, index: usize) -> Result<PicrossRowIter, &'static str> {
        let rules = self.rows.0.get(index).ok_or("invalid row index")?;
        Ok(PicrossRowIter::new(&rules.0, self.width()))
    }

    fn get_column_iter(&self, index: usize) -> Result<PicrossRowIter, &'static str> {
        let rules = self.columns.0.get(index).ok_or("invalid column index")?;
        Ok(PicrossRowIter::new(&rules.0, self.height()))
    }
    pub fn get_partial_board_from_rows(
        &self,
        reference_board: Option<GameBoard>,
    ) -> Result<GameBoard, &'static str> {
        let current_board = match reference_board {
            Some(board) => board,
            None => GameBoard::new(self.width(), self.height()),
        };
        let rows: Result<Vec<GameBoardRow>, &'static str> = self
            .rows
            .0
            .iter()
            .zip(current_board.0)
            .map(|(rule, reference_row)| {
                let mut row_iter = PicrossRowIter::new(&rule.0, self.width());
                row_iter.get_partially_solved_row(Some(reference_row))
            })
            .collect();
        rows.map(GameBoard)
    }
    fn get_partial_board_from_columns(
        &self,
        reference_board: Option<GameBoard>,
    ) -> Result<GameBoard, &'static str> {
        let current_board = match reference_board {
            Some(board) => board,
            None => GameBoard::new(self.width(), self.height()),
        };
        let mut board_flipped = GameBoard::new(self.height(), self.width());
        for (y, row) in current_board.0.iter().enumerate() {
            for (x, tile) in row.0.iter().enumerate() {
                board_flipped.set_tile(y, x, tile.clone())?;
            }
        }

        let columns: Result<Vec<GameBoardRow>, &'static str> = self
            .columns
            .0
            .iter()
            .zip(board_flipped.0)
            .map(|(rule, reference_row)| {
                let mut row_iter = PicrossRowIter::new(&rule.0, self.height());
                row_iter.get_partially_solved_row(Some(reference_row))
            })
            .collect();
        let mut output_board = GameBoard::new(self.width(), self.height());
        for (x, column) in columns?.iter().enumerate() {
            for (y, tile) in column.0.iter().enumerate() {
                output_board.set_tile(x, y, tile.clone())?;
            }
        }
        Ok(output_board)
    }

    pub fn solve_v2(&self) -> Result<GameBoard, &'static str> {
        let mut current_board =
            self.get_partial_board_from_columns(Some(self.get_partial_board_from_rows(None)?))?;
        let mut iterations = 1;
        loop {
            let new_board = self.get_partial_board_from_columns(Some(
                self.get_partial_board_from_rows(Some(current_board.clone()))?,
            ))?;
            iterations += 1;
            println!(
                "-----\nIteration:{}\n\n{}\n------",
                iterations,
                new_board.render()
            );
            if current_board == new_board {
                if new_board.0.iter().all(|row| {
                    row.0
                        .iter()
                        .all(|tile| !matches!(tile, crate::TileState::Undetermined))
                }) {
                    return Ok(new_board);
                } else {
                    eprintln!(
                        "\n-----\nCould not determine:\n{}\n------",
                        new_board.render()
                    );
                    return Err("Not Complete");
                }
            } else {
                current_board = new_board;
            }
        }
    }

    pub fn solve(&self) -> Result<GameBoard, &'static str> {
        let width = self.columns.0.len();

        struct StackEntry<'a> {
            row_iter: Iter<'a, RowColumnRule>,
            row_layout_iter: Option<PicrossRowIter<'a>>,
            board: GameBoard,
        }
        let mut stack = vec![StackEntry {
            row_iter: self.rows.0.iter(),
            row_layout_iter: None,
            board: GameBoard(vec![]),
        }];

        while let Some(StackEntry {
            mut row_iter,
            row_layout_iter,
            board,
        }) = stack.pop()
        {
            match validate_board(self, &board)? {
                BoardState::Invalid => (),
                BoardState::Complete(complete_board) => return Ok(complete_board),
                BoardState::InProgress => {
                    let next_row_layout_iter = row_iter
                        .next()
                        .map(|row_rule| PicrossRowIter::new(&row_rule.0, width));
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
}

#[cfg(test)]
mod tests {
    use crate::{GameBoardRow, TileState::*};

    use super::*;

    #[test]
    fn test_row_column_rules_from_string() {
        let res = RowColumnRules::from_str("1,0,1 2");
        let expected = RowColumnRules(vec![
            RowColumnRule(vec![1]),
            RowColumnRule(vec![0]),
            RowColumnRule(vec![1, 2]),
        ]);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn test_solve_basic_picross() {
        let game = PicrossGame::from_rules("1", "0,0,1").unwrap();
        let game_res = game.solve();
        assert!(game_res.is_ok());
        let solved_board = game_res.unwrap();
        let expected = GameBoard(vec![GameBoardRow(vec![Empty, Empty, Filled])]);
        assert_eq!(solved_board, expected);
    }

    #[test]
    fn test_solve_medium_picross() {
        let game = PicrossGame::from_rules("1 1,1,1 1", "1 1,1,1 1").unwrap();
        let game_res = game.solve();
        assert!(game_res.is_ok());
        let solved_board = game_res.unwrap();
        let expected = GameBoard(vec![
            GameBoardRow(vec![Filled, Empty, Filled]),
            GameBoardRow(vec![Empty, Filled, Empty]),
            GameBoardRow(vec![Filled, Empty, Filled]),
        ]);
        assert_eq!(solved_board, expected);
    }

    #[test]
    fn test_solve_complex_picross() {
        let game = PicrossGame::from_rules("1 1,1 1,5,1 1 1,5", "3,3 1,3,3 1,3").unwrap();
        let game_res = game.solve();
        assert!(game_res.is_ok());
        let solved_board = game_res.unwrap();
        let expected = GameBoard(vec![
            GameBoardRow(vec![Empty, Filled, Empty, Filled, Empty]),
            GameBoardRow(vec![Empty, Filled, Empty, Filled, Empty]),
            GameBoardRow(vec![Filled, Filled, Filled, Filled, Filled]),
            GameBoardRow(vec![Filled, Empty, Filled, Empty, Filled]),
            GameBoardRow(vec![Filled, Filled, Filled, Filled, Filled]),
        ]);
        assert_eq!(solved_board, expected);
    }

    #[test]
    fn test_solve_v2_basic_picross() {
        let game = PicrossGame::from_rules("1", "0,0,1").unwrap();
        let game_res = game.solve_v2();
        assert!(game_res.is_ok());
        let solved_board = game_res.unwrap();
        let expected = GameBoard(vec![GameBoardRow(vec![Empty, Empty, Filled])]);
        assert_eq!(solved_board, expected);
    }

    #[test]
    fn test_solve_v2_medium_picross() {
        let game = PicrossGame::from_rules("1 1,1,1 1", "1 1,1,1 1").unwrap();
        let game_res = game.solve_v2();
        assert!(game_res.is_ok());
        let solved_board = game_res.unwrap();
        let expected = GameBoard(vec![
            GameBoardRow(vec![Filled, Empty, Filled]),
            GameBoardRow(vec![Empty, Filled, Empty]),
            GameBoardRow(vec![Filled, Empty, Filled]),
        ]);
        assert_eq!(solved_board, expected);
    }

    #[test]
    fn test_solve_v2_complex_picross() {
        let game = PicrossGame::from_rules("1 1,1 1,5,1 1 1,5", "3,3 1,3,3 1,3").unwrap();
        let game_res = game.solve_v2();
        assert!(game_res.is_ok());
        let solved_board = game_res.unwrap();
        let expected = GameBoard(vec![
            GameBoardRow(vec![Empty, Filled, Empty, Filled, Empty]),
            GameBoardRow(vec![Empty, Filled, Empty, Filled, Empty]),
            GameBoardRow(vec![Filled, Filled, Filled, Filled, Filled]),
            GameBoardRow(vec![Filled, Empty, Filled, Empty, Filled]),
            GameBoardRow(vec![Filled, Filled, Filled, Filled, Filled]),
        ]);
        assert_eq!(solved_board, expected);
    }

    #[test]
    fn test_validate_chunks() {
        let rule = RowColumnRule(vec![3]);
        assert_eq!(
            validate_chunks(&rule, vec![0]),
            ChunksValidation::InProgress
        );

        let rule = RowColumnRule(vec![0]);
        assert_eq!(validate_chunks(&rule, vec![0]), ChunksValidation::Valid);

        let rule = RowColumnRule(vec![1, 2]);
        assert_eq!(validate_chunks(&rule, vec![1, 2]), ChunksValidation::Valid);

        let rule = RowColumnRule(vec![1, 2]);
        assert_eq!(
            validate_chunks(&rule, vec![2, 2]),
            ChunksValidation::Invalid
        );

        let rule = RowColumnRule(vec![1, 2, 3]);
        assert_eq!(
            validate_chunks(&rule, vec![1, 2, 2]),
            ChunksValidation::InProgress
        );
    }
}
