use super::*;

pub mod di;
pub mod heuristic;
pub mod look_ahead;
pub mod minimax;
pub mod snail;
pub mod true_random;

pub use {di::*, look_ahead::*, minimax::*, snail::*, true_random::*, heuristic::*};

pub trait Strategy {
    fn name(&self) -> String;
    fn find_move(&mut self, board: &GameState) -> Move;
    fn dup(&self) -> Box<dyn Strategy>;
}
