use super::*;

pub mod snail;
pub mod true_random;

pub use {snail::*, true_random::*};

pub trait Strategy {
    fn name(&self) -> String;
    fn find_move(&mut self, board: &GameState) -> Move;
    fn dup(&self) -> Box<dyn Strategy>;
}
