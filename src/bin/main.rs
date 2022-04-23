extern crate amazons;
use amazons::*;

fn main() {
    repeat_games(
        vec![
            Box::new(strat::LookAhead),
            Box::new(strat::DeepIdiot::with_depth(5)),
        ],
        10,
        false,
    );
    let mut game = Game::new(
        Box::new(strat::Minimax::with_depth(2)),
        Box::new(strat::DeepIdiot::with_depth(4)),
    );
    //show::display_game(&mut game.as_iter());
}
