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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct GameBoard(pub Vec<GameBoardRow>);

#[allow(dead_code)]
impl GameBoard {
    pub fn new(width: usize, height: usize) -> Self {
        let board: Vec<GameBoardRow> = vec![(); height]
            .iter()
            .map(|_| GameBoardRow::new(width))
            .collect();
        Self(board)
    }

    pub fn merge_board(&self, board: Self) -> Result<Self, &'static str> {
        let merged_board: Result<Vec<GameBoardRow>, &'static str> = self
            .0
            .clone()
            .into_iter()
            .zip(board.0)
            .map(|(row_a, row_b)| {
                let merged_row: Result<Vec<TileState>, &'static str> = row_a
                    .0
                    .into_iter()
                    .zip(row_b.0)
                    .map(|tile_pair| match tile_pair {
                        (TileState::Filled, TileState::Filled) => Ok(TileState::Filled),
                        (TileState::Filled, TileState::Undetermined) => Ok(TileState::Filled),
                        (TileState::Undetermined, TileState::Filled) => Ok(TileState::Filled),
                        (TileState::Empty, TileState::Empty) => Ok(TileState::Empty),
                        (TileState::Empty, TileState::Undetermined) => Ok(TileState::Empty),
                        (TileState::Undetermined, TileState::Empty) => Ok(TileState::Empty),
                        (TileState::Undetermined, TileState::Undetermined) => {
                            Ok(TileState::Undetermined)
                        }
                        (TileState::Filled, TileState::Empty) => Err("Invalid Tile Combination"),
                        (TileState::Empty, TileState::Filled) => Err("Invalid Tile Combination"),
                    })
                    .collect();
                merged_row.map(GameBoardRow)
            })
            .collect();
        merged_board.map(GameBoard)
    }
    pub fn get_column(&self, column_index: usize) -> Vec<TileState> {
        self.0
            .iter()
            .map(|row| row.0[column_index].clone())
            .collect()
    }

    pub fn set_tile(&mut self, x: usize, y: usize, state: TileState) -> Result<(), &'static str> {
        let row = self.0.get_mut(y).unwrap();
        row.set_tile(x, state);
        Ok(())
    }
    pub fn get_column_chunks(&self, index: usize) -> Result<Vec<usize>, &'static str> {
        let mut chunks: Vec<usize> = vec![];
        let mut is_collecting = false;
        let mut count = 0;
        for row in self.0.iter() {
            let state = row.0.get(index).unwrap();
            match state {
                TileState::Filled => {
                    count += 1;
                    is_collecting = true;
                }
                TileState::Empty => {
                    if is_collecting {
                        chunks.push(count);
                        count = 0;
                        is_collecting = false;
                    }
                }
                TileState::Undetermined => return Err("Cannot process undetermined tiles"), // Undetermined not
            }
        }
        if count > 0 {
            chunks.push(count);
        }
        if chunks.is_empty() {
            chunks = vec![0]
        }
        Ok(chunks)
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
        let _ = board.set_tile(0, 0, Filled);
        let _ = board.set_tile(1, 0, Empty);
        let _ = board.set_tile(2, 0, Filled);

        let _ = board.set_tile(0, 1, Empty);
        let _ = board.set_tile(1, 1, Filled);
        let _ = board.set_tile(2, 1, Empty);

        let _ = board.set_tile(0, 2, Filled);
        let _ = board.set_tile(1, 2, Empty);
        let _ = board.set_tile(2, 2, Filled);

        assert_eq!(board.render(), "x x\n x \nx x");
    }

    #[test]
    fn test_build_row_from_empty_segments() {
        let segments: Vec<Segment> = vec![];
        let row = GameBoardRow::build_from_segments(segments, 5).unwrap();
        assert_eq!(row, GameBoardRow(vec![Empty, Empty, Empty, Empty, Empty,]))
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
            GameBoardRow(vec![Filled, Filled, Filled, Empty, Filled,])
        );
    }

    #[test]
    fn test_get_board_column_chunks() {
        let board = GameBoard(vec![
            GameBoardRow(vec![Filled, Empty, Empty]),
            GameBoardRow(vec![Empty, Empty, Filled]),
            GameBoardRow(vec![Filled, Filled, Filled]),
        ]);
        assert_eq!(board.get_column_chunks(0).unwrap(), vec![1, 1]);
        assert_eq!(board.get_column_chunks(1).unwrap(), vec![1]);
        assert_eq!(board.get_column_chunks(2).unwrap(), vec![2]);
    }
}
