#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum TileState {
    Filled,
    Empty,
    Undetermined,
}

#[derive(Debug, Clone)]
pub struct Segment {
    pub length: usize,
    pub index: usize,
}

#[derive(Debug, PartialEq)]
pub struct GameBoardRow(pub Vec<TileState>);

impl GameBoardRow {
    fn new(width: usize) -> Self {
        Self(vec![TileState::Undetermined; width])
    }
    fn set_tile(&mut self, index: usize, tile: TileState) {
        self.0[index] = tile;
    }
    pub fn build_from_segments(segments: Vec<Segment>, width: usize) -> Result<Self, &'static str> {
        let row: Vec<TileState> = vec![(); width]
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let is_filled = segments
                    .iter()
                    .any(|seg| i >= seg.index && i < seg.index + seg.length);
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
pub struct GameBoard(Vec<GameBoardRow>);

#[allow(dead_code)]
impl GameBoard {
    pub fn new(width: usize, height: usize) -> Self {
        let board: Vec<GameBoardRow> = vec![(); height]
            .iter()
            .map(|_| GameBoardRow::new(width))
            .collect();
        Self(board)
    }

    pub fn set_tile(&mut self, x: usize, y: usize, state: TileState) -> Result<(), String> {
        let row = self.0.get_mut(y).unwrap();
        row.set_tile(x, state);
        Ok(())
    }
}

#[allow(dead_code)]
impl GameBoard {
    pub fn render(&self) -> String {
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

#[cfg(test)]
mod tests {
    use crate::iterators::PicrossRowIter;

    use super::TileState::*;
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
            GameBoardRow(vec![Filled, Empty, Empty])
        );
        assert_eq!(
            row_iter.next().unwrap(),
            GameBoardRow(vec![Empty, Filled, Empty])
        );
        assert_eq!(
            row_iter.next().unwrap(),
            GameBoardRow(vec![Empty, Empty, Filled])
        );
        assert!(row_iter.next().is_none());
    }

    #[test]
    fn test_row_iterator_two_chunk() {
        let mut row_iter = PicrossRowIter::new(vec![2], 3);
        assert_eq!(
            row_iter.next().unwrap(),
            GameBoardRow(vec![Filled, Filled, Empty])
        );
        assert_eq!(
            row_iter.next().unwrap(),
            GameBoardRow(vec![Empty, Filled, Filled])
        );
        assert!(row_iter.next().is_none());
    }

    #[test]
    fn test_row_iterator_complex() {
        let mut row_iter = PicrossRowIter::new(vec![2, 1], 5);

        assert_eq!(
            row_iter.next().unwrap(),
            GameBoardRow(vec![Filled, Filled, Empty, Filled, Empty])
        );
        assert_eq!(
            row_iter.next().unwrap(),
            GameBoardRow(vec![Filled, Filled, Empty, Empty, Filled])
        );
        assert_eq!(
            row_iter.next().unwrap(),
            GameBoardRow(vec![Empty, Filled, Filled, Empty, Filled])
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
