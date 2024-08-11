use std::str::FromStr;

#[derive(Debug)]
struct RowColumnRule(Vec<u8>);

#[derive(Debug)]
struct PicrossRowIter {
    rule: RowColumnRule,
    width: usize,
    initial_index: usize,
}

impl PicrossRowIter {
    fn new(rule: Vec<u8>, width: usize) -> Self {
        Self {
            rule: RowColumnRule(rule),
            width,
            initial_index: 0,
        }
    }
}

struct Segment {
    length: usize,
    index: usize,
}

impl Iterator for PicrossRowIter {
    type Item = GameBoardRow;

    fn next(&mut self) -> Option<Self::Item> {
        // need the index of the start of each "Segment"
        // place the first rule
        // reduce the remaining space, track the index offset
        // recurse somehow...
        Some(GameBoardRow(vec![
            TileState::Filled,
            TileState::Empty,
            TileState::Empty,
        ]))
    }
}

impl FromStr for RowColumnRule {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rule: Result<Vec<u8>, _> = s.trim().split(" ").map(|part| part.parse()).collect();
        match rule {
            Ok(rule) => Ok(Self(rule)),
            _ => Err("failed to parse"),
        }
    }
}

impl RowColumnRule {
    fn count(&self) -> usize {
        self.0
            .iter()
            .fold(0 as usize, |acc, &num| acc + num as usize)
    }
}

#[derive(Debug)]
struct RowColumnRules(Vec<RowColumnRule>);

impl FromStr for RowColumnRules {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules_result: Result<Vec<RowColumnRule>, _> = s
            .trim()
            .split(",")
            .map(|input| RowColumnRule::from_str(input))
            .collect();
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
    fn from_rules(row_rules: &str, column_rules: &str) -> Result<Self, &'static str> {
        todo!();
    }

    fn validate() -> Result<BoardState, &'static str> {
        todo!()
    }
    fn solve() -> Result<GameBoard, &'static str> {
        todo!()
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
enum TileState {
    Filled,
    Empty,
    Undetermined,
}

#[derive(Debug, PartialEq)]
struct GameBoardRow(Vec<TileState>);

impl GameBoardRow {
    fn new(width: usize) -> Self {
        Self(vec![TileState::Undetermined; width])
    }
    fn set_tile(&mut self, index: usize, tile: TileState) {
        self.0[index] = tile;
    }
    fn build_from_segments(segments: Vec<Segment>, width: usize) -> Result<Self, &'static str> {
        let row: Vec<TileState> = vec![(); width]
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let is_filled = segments.iter().any(|seg| {
                    if i >= seg.index && i < seg.index + seg.length {
                        return true;
                    } else {
                        return false;
                    }
                });
                if is_filled {
                    TileState::Filled
                } else {
                    TileState::Empty
                }
            })
            .collect();
        Ok(Self(row))
    }
}

#[derive(Debug)]
struct GameBoard(Vec<GameBoardRow>);

#[allow(dead_code)]
impl GameBoard {
    fn new(width: usize, height: usize) -> Self {
        let board: Vec<GameBoardRow> = vec![(); height]
            .iter()
            .map(|_| GameBoardRow::new(width))
            .collect();
        Self(board)
    }

    fn set_tile(&mut self, x: usize, y: usize, state: TileState) -> Result<(), String> {
        let row = self.0.get_mut(y).unwrap();
        row.set_tile(x, state);
        Ok(())
    }
}

impl GameBoard {
    fn render(&self) -> String {
        let display_string = self
            .0
            .iter()
            .map(|row| {
                row.0
                    .iter()
                    .map(|tile| match tile {
                        TileState::Empty => ' ',
                        TileState::Filled => 'x',
                        TileState::Undetermined => '?',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");
        display_string
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_game_board() {
        let board = GameBoard::new(3, 3);
        let board_string = board.render();
        let expected = "???\n???\n???";
        assert_eq!(board_string, expected);
    }

    #[test]
    fn test_set_tiles() {
        let mut board = GameBoard::new(3, 3);
        let _ = board.set_tile(0, 0, TileState::Filled);
        let _ = board.set_tile(1, 0, TileState::Empty);
        let _ = board.set_tile(2, 0, TileState::Filled);

        let _ = board.set_tile(0, 1, TileState::Empty);
        let _ = board.set_tile(1, 1, TileState::Filled);
        let _ = board.set_tile(2, 1, TileState::Empty);

        let _ = board.set_tile(0, 2, TileState::Filled);
        let _ = board.set_tile(1, 2, TileState::Empty);
        let _ = board.set_tile(2, 2, TileState::Filled);

        assert_eq!(board.render(), "x x\n x \nx x");
    }

    #[test]
    fn test_row_iterator() {
        let mut row_iter = PicrossRowIter::new(vec![1], 3);
        assert_eq!(
            row_iter.next().unwrap(),
            GameBoardRow(vec![TileState::Filled, TileState::Empty, TileState::Empty])
        );
        assert_eq!(
            row_iter.next().unwrap(),
            GameBoardRow(vec![TileState::Empty, TileState::Filled, TileState::Empty])
        );
        assert_eq!(
            row_iter.next().unwrap(),
            GameBoardRow(vec![TileState::Empty, TileState::Empty, TileState::Filled])
        );
        assert!(row_iter.next().is_none());
    }

    #[test]
    fn test_build_row_from_empty_segments() {
        let segments: Vec<Segment> = vec![];
        let row = GameBoardRow::build_from_segments(segments, 5).unwrap();
        assert_eq!(
            row,
            GameBoardRow(vec![
                TileState::Empty,
                TileState::Empty,
                TileState::Empty,
                TileState::Empty,
                TileState::Empty,
            ])
        )
    }

    #[test]
    fn test_build_row_from_segments() {
        let segments: Vec<Segment> = vec![
            Segment {
                index: 0,
                length: 3,
            },
            Segment {
                index: 4,
                length: 1,
            },
        ];
        let row = GameBoardRow::build_from_segments(segments, 5).unwrap();
        assert_eq!(
            row,
            GameBoardRow(vec![
                TileState::Filled,
                TileState::Filled,
                TileState::Filled,
                TileState::Empty,
                TileState::Filled,
            ])
        )
    }
}
