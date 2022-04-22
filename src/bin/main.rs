extern crate amazons;
use amazons::*;

fn main() {
    //repeat_games(
    //    vec![
    //        Box::new(strat::LookAhead::with_depth(1)),
    //        Box::new(strat::Minimax::with_depth(3)),
    //    ],
    //    1,
    //    false,
    //);
    let mut game = Game::new(
        Box::new(strat::Minimax::with_depth(3)),
        Box::new(strat::LookAhead::with_depth(1)),
    );
    show::display_game(&mut game.as_iter());
}
