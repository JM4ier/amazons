use super::*;

pub mod look_ahead;
pub mod snail;
pub mod true_random;

pub use {look_ahead::*, snail::*, true_random::*};

pub trait Strategy {
    fn name(&self) -> String;
    fn find_move(&mut self, board: &GameState) -> Move;
    fn dup(&self) -> Box<dyn Strategy>;
}
