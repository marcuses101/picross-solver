use std::usize;

use crate::{GameBoardRow, Segment, TileState};

#[derive(Debug, Clone)]
pub struct PicrossRowIter<'a> {
    width: usize,
    stack: Vec<RowIterFrame<'a>>,
}

#[derive(Debug, Clone)]
struct RowIterFrame<'a> {
    index: usize,
    current_solution: Vec<Segment>,
    remaining_chunks: &'a [usize],
}

impl<'a> PicrossRowIter<'a> {
    pub fn new(chunks: &'a [usize], width: usize) -> Self {
        Self {
            width,
            stack: vec![RowIterFrame {
                index: 0,
                current_solution: vec![],
                remaining_chunks: chunks,
            }],
        }
    }
    pub fn get_partially_solved_row(
        &mut self,
        known_row: Option<GameBoardRow>,
    ) -> Result<GameBoardRow, &'static str> {
        let compare_row = match known_row {
            Some(row) => row,
            None => GameBoardRow(vec![TileState::Undetermined; self.width]),
        };
        self.filter(|row| {
            row.0.iter().zip(&compare_row.0).all(|pair| {
                !matches!(
                    pair,
                    (TileState::Filled, TileState::Empty) | (TileState::Empty, TileState::Filled)
                )
            })
        })
        .reduce(|acc, cur| {
            let row: Vec<TileState> = acc
                .0
                .into_iter()
                .zip(cur.0)
                .map(|pair| match pair {
                    (TileState::Filled, TileState::Filled) => TileState::Filled,
                    (TileState::Empty, TileState::Empty) => TileState::Empty,
                    _ => TileState::Undetermined,
                })
                .collect();
            GameBoardRow(row)
        })
        .ok_or("no valid configurations")
    }
}

impl Iterator for PicrossRowIter<'_> {
    type Item = GameBoardRow;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(frame) = self.stack.pop() {
            if frame.remaining_chunks.is_empty() {
                return Some(
                    GameBoardRow::build_from_segments(frame.current_solution, self.width).unwrap(),
                );
            }

            let current_chunk_length = frame.remaining_chunks[0];
            if current_chunk_length == 0 {
                return Some(GameBoardRow::build_from_segments(vec![], self.width).unwrap());
            }
            let others = &frame.remaining_chunks[1..];

            for i in (frame.index..self.width).rev() {
                if current_chunk_length + i <= self.width {
                    let mut new_solution = frame.current_solution.clone();
                    new_solution.push(Segment {
                        index: i,
                        length: current_chunk_length,
                    });
                    let frame = RowIterFrame {
                        index: i + current_chunk_length + 1,
                        current_solution: new_solution,
                        remaining_chunks: others,
                    };
                    self.stack.push(frame);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::TileState::*;

    #[test]
    fn test_row_iterator() {
        let mut row_iter = PicrossRowIter::new(&[1], 3);
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
    fn test_row_iterator_0() {
        let mut row_iter = PicrossRowIter::new(&[0], 3);
        assert_eq!(
            row_iter.next(),
            Some(GameBoardRow(vec![Empty, Empty, Empty]))
        );
        assert_eq!(row_iter.next(), None);
    }

    #[test]
    fn test_row_iterator_two_chunk() {
        let mut row_iter = PicrossRowIter::new(&[2], 3);
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
        let mut row_iter = PicrossRowIter::new(&[2, 1], 5);

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
    fn test_get_partially_solved_row() {
        let width = 10;
        let mut row_iter = PicrossRowIter::new(&[6], width);
        assert_eq!(
            row_iter.get_partially_solved_row(None),
            Ok(GameBoardRow(vec![
                Undetermined,
                Undetermined,
                Undetermined,
                Undetermined,
                Filled,
                Filled,
                Undetermined,
                Undetermined,
                Undetermined,
                Undetermined,
            ]))
        );

        let mut row_iter = PicrossRowIter::new(&[0], width);
        assert_eq!(
            row_iter.get_partially_solved_row(None),
            Ok(GameBoardRow(vec![
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            ]))
        )
    }
}
