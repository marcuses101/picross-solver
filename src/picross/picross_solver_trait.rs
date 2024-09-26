use crate::render::PicrossFrame;

use super::PicrossGame;

pub trait PicrossSolver {
    type Iter: Iterator<Item = PicrossFrame>;
    fn iter(&self) -> Self::Iter;
    fn solve(&self) -> Result<PicrossFrame, &'static str>;
    fn from_game(game: PicrossGame) -> Self;
    fn set_game(&mut self, game: PicrossGame);
}
