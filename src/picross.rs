use crate::{game_board::GameBoard, iterators::PicrossLineIter, GameBoardRow, TileState};
use std::{collections::VecDeque, fs::read_to_string, slice::Iter, str::FromStr};

const DIVIDER: &str = "-----";

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
    pub fn from_filepath(filename: &str) -> Result<Self, String> {
        let content = read_to_string(filename).map_err(|_| "Failed to read file")?;
        PicrossGame::from_rules_file_string(&content)
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

    pub fn from_rules_file_string(input: &str) -> Result<Self, String> {
        let mut rules = input
            .split(DIVIDER)
            .map(|section| section.trim().lines().collect::<Vec<&str>>().join(","));
        let row_rules = rules.next().ok_or("invalid file format")?;
        let col_rules = rules.next().ok_or("invalid file format")?;
        PicrossGame::from_rules(&row_rules, &col_rules)
    }

    pub fn to_rules_file_string(&self) -> String {
        let row_string: String = self
            .rows
            .0
            .iter()
            .map(|rule| {
                rule.0
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");
        let col_string: String = self
            .columns
            .0
            .iter()
            .map(|rule| {
                rule.0
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");
        format!("{row_string}\n{DIVIDER}\n{col_string}")
    }

    fn get_row_iter(&self, index: usize) -> Result<PicrossLineIter, &'static str> {
        let rules = self.rows.0.get(index).ok_or("invalid row index")?;
        Ok(PicrossLineIter::new(&rules.0, self.width()))
    }

    fn get_column_iter(&self, index: usize) -> Result<PicrossLineIter, &'static str> {
        let rules = self.columns.0.get(index).ok_or("invalid column index")?;
        Ok(PicrossLineIter::new(&rules.0, self.height()))
    }
    pub fn get_partial_board_from_rows(
        &self,
        reference_board: Option<GameBoard>,
    ) -> Result<GameBoard, &'static str> {
        let board = match reference_board {
            Some(board) => board,
            None => GameBoard::new(self.width(), self.height()),
        };
        let rows: Result<Vec<GameBoardRow>, &'static str> = self
            .rows
            .0
            .iter()
            .zip(board.0)
            .map(|(rule, reference_row)| {
                let mut row_iter = PicrossLineIter::new(&rule.0, self.width());
                row_iter.get_partially_solved_line(Some(&reference_row))
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
                let mut row_iter = PicrossLineIter::new(&rule.0, self.height());
                row_iter.get_partially_solved_line(Some(&reference_row))
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

    #[allow(unused_variables)]
    pub fn solve_v3(&self) -> Result<GameBoard, &'static str> {
        let mut board = GameBoard::new(self.width(), self.height());
        #[derive(Debug, PartialEq)]
        enum ToCheck {
            Row,
            Column,
        }
        let mut queue: VecDeque<(ToCheck, usize)> = VecDeque::new();
        (0..self.width()).for_each(|col_index| queue.push_back((ToCheck::Column, col_index)));
        (0..self.height()).for_each(|row_index| queue.push_back((ToCheck::Row, row_index)));
        // populate the rows initially
        while let Some((board_axis, index)) = queue.pop_front() {
            match board_axis {
                ToCheck::Row => {
                    let row_index = index;
                    // maybe rethink this...
                    let rules = &self.rows.0.get(index).ok_or("failed to get row rules")?;
                    let mut line_iter = PicrossLineIter::new(&rules.0, self.width());
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
                        .columns
                        .0
                        .get(col_index)
                        .ok_or("failed to get col rules")?;
                    let mut line_iter = PicrossLineIter::new(&rules.0, self.height());
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
                .all(|tile| !matches!(tile, crate::TileState::Undetermined))
        }) {
            Ok(board)
        } else {
            eprintln!("\n-----\nCould not determine:\n{}\n------", board.render());
            Err("Not Complete")
        }
    }

    pub fn solve_v2(&self) -> Result<GameBoard, &'static str> {
        let mut current_board =
            self.get_partial_board_from_columns(Some(self.get_partial_board_from_rows(None)?))?;
        loop {
            let new_board = self.get_partial_board_from_columns(Some(
                self.get_partial_board_from_rows(Some(current_board.clone()))?,
            ))?;
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

    pub fn solve_v1(&self) -> Result<GameBoard, &'static str> {
        let width = self.columns.0.len();

        struct StackEntry<'a> {
            row_iter: Iter<'a, RowColumnRule>,
            row_layout_iter: Option<PicrossLineIter<'a>>,
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
            println!("{}", &board.render());
            match validate_board(self, &board)? {
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
}

#[cfg(test)]
mod tests {
    use crate::{GameBoardRow, TileState::*};

    use super::*;

    #[test]
    fn test_to_rules_string() {
        let game = PicrossGame::from_rules("1 1,1,1 1", "1 1,1,1 1").unwrap();
        let output = game.to_rules_file_string();
        let expected = "\
1 1
1
1 1
-----
1 1
1
1 1";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_from_and_to_rules_string() {
        let input = "\
1 1
1
1 1
-----
1 1
1
1 1";
        let game = PicrossGame::from_rules_file_string(input).unwrap();
        let output = game.to_rules_file_string();
        assert_eq!(input, output);
    }

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
        let game_res = game.solve_v1();
        assert!(game_res.is_ok());
        let solved_board = game_res.unwrap();
        let expected = GameBoard(vec![GameBoardRow(vec![Empty, Empty, Filled])]);
        assert_eq!(solved_board, expected);
    }

    #[test]
    fn test_solve_medium_picross() {
        let game = PicrossGame::from_rules("1 1,1,1 1", "1 1,1,1 1").unwrap();
        let game_res = game.solve_v1();
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
        let game_res = game.solve_v1();
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
    fn test_solve_v3_basic_picross() {
        let game = PicrossGame::from_rules("1", "0,0,1").unwrap();
        let expected = GameBoard(vec![GameBoardRow(vec![Empty, Empty, Filled])]);
        assert_eq!(game.solve_v3(), Ok(expected));
    }

    #[test]
    fn test_solve_v3_medium_picross() {
        let game = PicrossGame::from_rules("1 1,1,1 1", "1 1,1,1 1").unwrap();
        let expected = GameBoard(vec![
            GameBoardRow(vec![Filled, Empty, Filled]),
            GameBoardRow(vec![Empty, Filled, Empty]),
            GameBoardRow(vec![Filled, Empty, Filled]),
        ]);
        assert_eq!(game.solve_v3(), Ok(expected));
    }

    #[test]
    fn test_solve_v3_complex_picross() {
        let game = PicrossGame::from_rules("1 1,1 1,5,1 1 1,5", "3,3 1,3,3 1,3").unwrap();
        let expected = GameBoard(vec![
            GameBoardRow(vec![Empty, Filled, Empty, Filled, Empty]),
            GameBoardRow(vec![Empty, Filled, Empty, Filled, Empty]),
            GameBoardRow(vec![Filled, Filled, Filled, Filled, Filled]),
            GameBoardRow(vec![Filled, Empty, Filled, Empty, Filled]),
            GameBoardRow(vec![Filled, Filled, Filled, Filled, Filled]),
        ]);
        assert_eq!(game.solve_v3(), Ok(expected));
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
