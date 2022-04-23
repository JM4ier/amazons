use super::*;

pub type Value = i32;
pub const MIN_VAL: Value = -i32::MAX;
pub const MAX_VAL: Value = i32::MAX;

pub trait Heuristic {
    fn name(&self) -> String;
    fn eval(&self, player: Player, board: &Board) -> Value;
}

pub struct Reachability;

impl Heuristic for Reachability {
    fn name(&self) -> String {
        "Reachability".into()
    }
    fn eval(&self, player: Player, board: &Board) -> Value {
        fn eval_player(player: Player, board: &Board) -> Value {
            board
                .find_amazons(player)
                .into_iter()
                .map(|a| board.reach_count(a) as Value)
                .sum()
        }
        let me_val = eval_player(player, board);
        let enemy_val = eval_player(player.enemy(), board);

        if me_val == 0 {
            MIN_VAL
        } else if enemy_val == 0 {
            MAX_VAL
        } else {
            me_val - enemy_val
        }
    }
}
