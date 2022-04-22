use super::*;

#[derive(Copy, Clone)]
pub struct LookAhead {
    depth: usize,
}

impl LookAhead {
    pub fn with_depth(depth: usize) -> Self {
        Self { depth }
    }
}

fn value(player: Player, board: &Board) -> u32 {
    board
        .find_amazons(player)
        .into_iter()
        .map(|a| {
            board
                .reachable_from(a)
                .into_iter()
                .map(|p| a.distance_to(p) as u32)
                .sum::<u32>()
        })
        .sum()
}

#[inline]
fn total_value(player: Player, board: &Board) -> u32 {
    1000000 + value(player, board) - value(player.enemy(), board)
}

impl Strategy for LookAhead {
    fn name(&self) -> String {
        "LookAhead".into()
    }
    fn find_move(&mut self, state: &GameState) -> Move {
        let mut state = state.clone();

        let mut best_value = 0;
        let mut best_move = Random.find_move(&state);

        let me = state.turn;

        for _ in 0..1000 {
            let mov = Random.find_move(&state);
            state.do_move(mov);
            let value = total_value(me, &state.board);
            state.undo_move(mov);
            if value > best_value {
                best_value = value;
                best_move = mov;
            }
        }

        best_move
    }
    fn dup(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
