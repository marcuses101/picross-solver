use std::collections::VecDeque;

use crate::{GameBoardRow, Segment};

#[derive(Debug, Clone)]
pub struct PicrossRowIter {
    width: usize,
    stack: VecDeque<RowIterFrame>,
}

#[derive(Debug, Clone)]
struct RowIterFrame {
    index: usize,
    current_solution: Vec<Segment>,
    remaining_chunks: Vec<usize>,
}

#[allow(dead_code)]
impl PicrossRowIter {
    pub fn new(chunks: Vec<usize>, width: usize) -> Self {
        let mut stack: VecDeque<RowIterFrame> = VecDeque::new();
        let initial_frame = RowIterFrame {
            index: 0,
            current_solution: vec![],
            remaining_chunks: chunks,
        };
        stack.push_back(initial_frame);
        Self { width, stack }
    }
}

impl Iterator for PicrossRowIter {
    type Item = GameBoardRow;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(frame) = self.stack.pop_front() {
            if frame.remaining_chunks.is_empty() {
                return Some(
                    GameBoardRow::build_from_segments(frame.current_solution, self.width).unwrap(),
                );
            }

            let current_chunk_length = frame.remaining_chunks[0];
            if current_chunk_length == 0 {
                return Some(GameBoardRow::build_from_segments(vec![], self.width).unwrap());
            }
            let others = frame.remaining_chunks[1..].to_vec();

            for i in frame.index..self.width {
                if current_chunk_length + i <= self.width {
                    let mut new_solution = frame.current_solution.clone();
                    new_solution.push(Segment {
                        index: i,
                        length: current_chunk_length,
                    });
                    let frame = RowIterFrame {
                        index: i + current_chunk_length + 1,
                        current_solution: new_solution,
                        remaining_chunks: others.clone(),
                    };
                    self.stack.push_back(frame);
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
    fn test_row_iterator_0() {
        let mut row_iter = PicrossRowIter::new(vec![0], 3);
        assert_eq!(
            row_iter.next(),
            Some(GameBoardRow(vec![Empty, Empty, Empty]))
        );
        assert_eq!(row_iter.next(), None);
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
}
