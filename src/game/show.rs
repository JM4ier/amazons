use super::*;
use std::{time, thread};

pub fn display_game(moves: &mut Iterator<Move>) {
    display_end_game(GameState::new(), moves)
}

pub fn display_end_game(mut config: GameState, moves: &mut Iterator<Move>) {
    for mov in moves {
        config.do_move(mov);
        config.display();
        thread::sleep(time::Duration::from_secs_f32(0.2));
    }
}