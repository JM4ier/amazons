use super::*;
use std::{thread, time};

pub fn display_game(moves: &mut impl Iterator<Item = Move>) {
    display_end_game(GameState::new(), moves)
}

pub fn display_end_game(mut config: GameState, moves: &mut impl Iterator<Item = Move>) {
    let style = BoardStyle::default();
    println!("{}", config.board.display(&style));
    for mov in moves {
        thread::sleep(time::Duration::from_secs_f32(0.1));
        config.do_move(mov);
        println!("{}", config.board.display(&style));
    }
    println!(
        "Oopsie Woopsie, Player {:?} is stucky-wucky >_<",
        config.turn
    );
}
