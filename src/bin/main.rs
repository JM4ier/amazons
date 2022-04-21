extern crate amazons;
use amazons::*;

fn main() {
    repeat_games(vec![Box::new(strat::RandomSnail), Box::new(strat::Snail)], 1000);
}