use crate::{
    game_board::{GameBoard, GameBoardRow},
    iterators::PicrossLineIter,
};
use std::str::FromStr;

pub mod picross_solver_trait;
pub mod picross_solver_v1;
pub mod picross_solver_v2;
pub mod picross_solver_v3;

const DIVIDER: &str = "-----";

#[derive(Debug, PartialEq, Clone)]
pub struct LineRule(pub Vec<usize>);

#[allow(dead_code)]
impl LineRule {
    fn from_render_line(input: &str) -> Self {
        let rule: Vec<usize> = input
            .chars()
            .enumerate()
            .fold(
                (false, 0, vec![]),
                |(is_collecting, count, mut rule), (index, char)| {
                    let is_last_char = index + 1 == input.len();
                    match char {
                        'x' | 'X' => {
                            if is_last_char {
                                rule.push(count + 1);
                                return (false, 0, rule);
                            }
                            return (true, count + 1, rule);
                        }
                        ' ' => {
                            if is_last_char && rule.len() == 0 && count == 0 {
                                rule.push(0);
                                return (false, 0, rule);
                            }
                            if is_collecting && (count != 0) {
                                rule.push(count);
                            }
                            return (false, 0, rule);
                        }
                        _ => panic!(),
                    }
                },
            )
            .2;
        Self(rule)
    }
}

impl FromStr for LineRule {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rule: Result<Vec<usize>, _> = s.trim().split(' ').map(|part| part.parse()).collect();
        match rule {
            Ok(rule) => Ok(Self(rule)),
            _ => Err("failed to parse"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AxisRules(pub Vec<LineRule>);

impl FromStr for AxisRules {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules_result: Result<Vec<LineRule>, _> =
            s.trim().split(',').map(LineRule::from_str).collect();
        match rules_result {
            Ok(rules) => Ok(Self(rules)),
            Err(_) => Err("failed to parse"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PicrossGame {
    pub rows: AxisRules,
    pub columns: AxisRules,
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

fn validate_chunks(rule: &LineRule, chunks: Vec<usize>) -> ChunksValidation {
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

impl Default for PicrossGame {
    fn default() -> Self {
        Self::from_rules("0", "0").unwrap()
    }
}

impl PicrossGame {
    pub fn width(&self) -> usize {
        self.columns.0.len()
    }
    pub fn height(&self) -> usize {
        self.rows.0.len()
    }

    pub fn from_rules(row_rules: &str, column_rules: &str) -> Result<Self, String> {
        let rows = AxisRules::from_str(row_rules)?;
        let columns = AxisRules::from_str(column_rules)?;
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

    pub fn from_text_render(input: &str) -> Result<Self, String> {
        let rows: Vec<LineRule> = input
            .lines()
            .map(|line| {
                let rule = LineRule::from_render_line(line);
                rule
            })
            .collect();
        let column_count = input.lines().next().ok_or("no column")?.len();
        let columns: Vec<LineRule> = (0..column_count)
            // collect col_index chars into a string
            .map(|col_index| {
                input
                    .lines()
                    .map(|line| {
                        return line.chars().nth(col_index).unwrap();
                    })
                    .collect::<String>()
            })
            .map(|line| LineRule::from_render_line(&line))
            .collect();

        Ok(Self {
            rows: AxisRules(rows),
            columns: AxisRules(columns),
        })
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
        let current_board = reference_board.unwrap_or(GameBoard::new(self.width(), self.height()));
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
}

#[cfg(test)]
mod tests {
    use picross_solver_v1::PicrossSolverV1;
    use picross_solver_v2::PicrossSolverV2;
    use picross_solver_v3::PicrossSolverV3;

    use crate::{
        game_board::{GameBoardRow, TileState::*},
        render::GameState,
    };

    use super::*;

    fn run_solver_tests<T: PicrossSolver>(instance: &mut T) {
        let basic_game = PicrossGame::from_rules("1", "0,0,1").unwrap();
        let basic_expected = GameBoard(vec![GameBoardRow(vec![Empty, Empty, Filled])]);
        instance.set_game(basic_game);
        let solved_frame = instance.solve().unwrap();
        assert_eq!(solved_frame.game_state, GameState::Complete);

        let solved_board = solved_frame.board;
        assert_eq!(solved_board, basic_expected);

        let medium_game = PicrossGame::from_rules("1 1,1,1 1", "1 1,1,1 1").unwrap();
        let medium_expected = GameBoard(vec![
            GameBoardRow(vec![Filled, Empty, Filled]),
            GameBoardRow(vec![Empty, Filled, Empty]),
            GameBoardRow(vec![Filled, Empty, Filled]),
        ]);
        instance.set_game(medium_game);

        let solved_frame = instance.solve().unwrap();
        assert_eq!(solved_frame.game_state, GameState::Complete);

        let solved_board = solved_frame.board;
        assert_eq!(solved_board, medium_expected);

        let complex_game = PicrossGame::from_rules("1 1,1 1,5,1 1 1,5", "3,3 1,3,3 1,3").unwrap();
        let complex_expected = GameBoard(vec![
            GameBoardRow(vec![Empty, Filled, Empty, Filled, Empty]),
            GameBoardRow(vec![Empty, Filled, Empty, Filled, Empty]),
            GameBoardRow(vec![Filled, Filled, Filled, Filled, Filled]),
            GameBoardRow(vec![Filled, Empty, Filled, Empty, Filled]),
            GameBoardRow(vec![Filled, Filled, Filled, Filled, Filled]),
        ]);

        instance.set_game(complex_game);
        let solved_frame = instance.solve().unwrap();
        assert_eq!(solved_frame.game_state, GameState::Complete);

        let solved_board = solved_frame.board;
        assert_eq!(solved_board, complex_expected);
    }

    #[test]
    fn test_solver_v1() {
        let mut solver = PicrossSolverV1(PicrossGame::from_rules("0", "0").unwrap());
        run_solver_tests(&mut solver);
    }
    #[test]
    fn test_solver_v2() {
        let mut solver = PicrossSolverV2(PicrossGame::from_rules("0", "0").unwrap());
        run_solver_tests(&mut solver);
    }
    #[test]
    fn test_solver_v3() {
        let mut solver = PicrossSolverV3(PicrossGame::from_rules("0", "0").unwrap());
        run_solver_tests(&mut solver);
    }

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
        let res = AxisRules::from_str("1,0,1 2");
        let expected = AxisRules(vec![
            LineRule(vec![1]),
            LineRule(vec![0]),
            LineRule(vec![1, 2]),
        ]);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn test_validate_chunks() {
        let rule = LineRule(vec![3]);
        assert_eq!(
            validate_chunks(&rule, vec![0]),
            ChunksValidation::InProgress
        );

        let rule = LineRule(vec![0]);
        assert_eq!(validate_chunks(&rule, vec![0]), ChunksValidation::Valid);

        let rule = LineRule(vec![1, 2]);
        assert_eq!(validate_chunks(&rule, vec![1, 2]), ChunksValidation::Valid);

        let rule = LineRule(vec![1, 2]);
        assert_eq!(
            validate_chunks(&rule, vec![2, 2]),
            ChunksValidation::Invalid
        );

        let rule = LineRule(vec![1, 2, 3]);
        assert_eq!(
            validate_chunks(&rule, vec![1, 2, 2]),
            ChunksValidation::InProgress
        );
    }

    #[test]
    fn test_line_rule_from_render_text() {
        let line = "XX   XXXX   XX";
        let rule = LineRule::from_render_line(line);
        let expected = LineRule(vec![2, 4, 2]);
        assert_eq!(rule, expected);
    }

    #[test]
    fn test_from_text_render() {
        let input = "\
x x
 x 
   
xxx
xxx"
        .trim();
        let game = PicrossGame::from_text_render(input).unwrap();
        let expected = PicrossGame {
            rows: AxisRules::from_str("1 1,1,0,3,3").unwrap(),
            columns: AxisRules::from_str("1 2,1 2,1 2").unwrap(),
        };
        pretty_assertions::assert_eq!(game, expected);
    }
}
