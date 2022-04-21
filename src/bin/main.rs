extern crate amazons;
use amazons::*;

fn main() {
    repeat_games(vec![Box::new(strat::RandomSnail), Box::new(strat::Snail)], 1000);
    let mut game = Game::new(Box::new(strat::RandomSnail), Box::new(strat::Snail));
    show::display_game(&mut game.as_iter());
}