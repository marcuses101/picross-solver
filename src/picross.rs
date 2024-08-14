use crate::game_board::GameBoard;
use std::str::FromStr;

#[derive(Debug)]
struct RowColumnRule(Vec<usize>);

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

impl RowColumnRule {
    fn count(&self) -> usize {
        self.0.iter().sum()
    }
}

#[derive(Debug)]
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

#[allow(dead_code)]
impl RowColumnRules {
    fn count(&self) -> usize {
        self.0.iter().map(|entry| entry.count()).sum()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct PicrossGame {
    width: usize,
    height: usize,
    rows: RowColumnRules,
    columns: RowColumnRules,
    board: GameBoard,
}

#[allow(dead_code)]
enum BoardState {
    Complete,
    InProgress,
}

#[allow(dead_code)]
impl PicrossGame {
    pub fn from_rules(row_rules: &str, column_rules: &str) -> Result<Self, &'static str> {
        todo!();
    }

    fn validate() -> Result<BoardState, &'static str> {
        todo!()
    }
    pub fn solve() -> Result<GameBoard, &'static str> {
        todo!()
    }
}
