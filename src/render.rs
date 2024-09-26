use std::{cmp, fmt::Display};

use crate::{
    game_board::GameBoard,
    picross::{AxisRules, LineRule, PicrossGame},
};

#[derive(PartialEq, Debug)]
pub enum GameState {
    #[allow(dead_code)]
    InProgress,
    Invalid,
    Complete,
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            GameState::InProgress => "In Progress",
            GameState::Invalid => "Invalid",
            GameState::Complete => "Complete",
        };
        write!(f, "{}", output)
    }
}

pub struct PicrossFrame {
    pub game_state: GameState,
    game: PicrossGame,
    pub board: GameBoard,
}

fn render_row_line_rule(rule: &LineRule) -> String {
    rule.0
        .iter()
        .map(|chunk_size| chunk_size.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn render_column_axis_rules(rules: &AxisRules) -> String {
    let row_count = rules
        .0
        .iter()
        .fold(0, |acc, cur| cmp::max(acc, cur.0.len()));
    (0..row_count)
        .rev()
        .map(|row_index| {
            rules
                .0
                .iter()
                .map(|rule| {
                    let row_rule = rule.0.get(row_index);
                    match row_rule {
                        Some(rule_number) => {
                            let mut rule_string = rule_number.to_string();
                            if rule_string.len() == 1 {
                                rule_string.push_str(" ")
                            }
                            rule_string
                        }
                        None => "  ".to_string(),
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

impl PicrossFrame {
    pub fn new(
        game: PicrossGame,
        board: GameBoard,
        game_state: GameState,
    ) -> Result<Self, &'static str> {
        if game.width() != board.width() || game.height() != board.height() {
            return Err("game dimensions don't match board dimensions");
        }

        Ok(Self {
            game,
            board,
            game_state,
        })
    }
    pub fn render(&self) -> String {
        let row_padding = self.game.rows.0.iter().fold(0, |acc, rule| {
            let width = render_row_line_rule(&rule).len();
            cmp::max(acc, width)
        });
        let column_rules_rendered = render_column_axis_rules(&self.game.columns);
        let column_rules_rendered: String = column_rules_rendered
            .lines()
            .map(|line| " ".repeat(row_padding) + line)
            .collect::<Vec<String>>()
            .join("\n");
        let board = &self.board.render();
        let rows_and_board: String = self
            .game
            .rows
            .0
            .iter()
            .zip(board.lines())
            .map(|(rule, board_row)| {
                let rule_rendered = render_row_line_rule(rule);
                let padding = row_padding - rule_rendered.len();
                format!("{}{}{}", " ".repeat(padding), rule_rendered, board_row)
            })
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "{}\n\n{}\n{}",
            self.game_state, column_rules_rendered, rows_and_board
        )
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use pretty_assertions::assert_eq;

    use crate::game_board::GameBoardRow;
    use crate::game_board::TileState::*;

    use super::*;

    #[test]
    fn test_render_column_axis_rules() {
        let rules = AxisRules::from_str("1 1,1,1 1").unwrap();
        let rendered = render_column_axis_rules(&rules);
        let expected = "\
1   1 
1 1 1 ";
        assert_eq!(rendered, expected.to_string());
    }

    #[test]
    fn test_frame_produces_expected_output() {
        let board = GameBoard(vec![
            GameBoardRow(vec![Filled, Empty, Filled]),
            GameBoardRow(vec![Empty, Filled, Empty]),
            GameBoardRow(vec![Filled, Empty, Filled]),
        ]);
        let game = PicrossGame::from_rules("1 1,1,1 1", "1 1,1,1 1").unwrap();
        let frame = PicrossFrame::new(game, board, GameState::InProgress).unwrap();
        let expected = "In Progress\n
   1   1 
   1 1 1 
1 1██  ██
  1  ██  
1 1██  ██";
        assert_eq!(frame.render(), expected.to_string())
    }
}
