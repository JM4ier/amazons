use super::*;

#[derive(Copy, Clone)]
pub struct Minimax {
    depth: usize,
}

impl Minimax {
    pub fn with_depth(depth: usize) -> Self {
        Self { depth }
    }
}

fn alpha_beta(
    state: &mut GameState,
    mut alpha: i32,
    beta: i32,
    depth: usize,
) -> (i32, Option<Move>) {
    if depth == 0 {
        return (Reachability.eval(state.turn, &state.board), None);
    }

    if state.is_finished() {
        return (-i32::MAX, None);
    }

    let mut best = None;

    for _ in 0..1000 {
        let mov = Random.find_move(state);

        state.do_move(mov);
        let score = -alpha_beta(state, -beta, -alpha, depth - 1).0;
        state.undo_move(mov);

        if score >= beta {
            return (beta, best.or(Some(mov)));
        }
        if score > alpha {
            alpha = score;
            best = Some(mov);
        }
    }
    (alpha, best)
}

impl Strategy for Minimax {
    fn name(&self) -> String {
        format!("Minimax({})", self.depth)
    }
    fn find_move(&mut self, state: &GameState) -> Move {
        alpha_beta(&mut state.clone(), -i32::MAX, i32::MAX, self.depth)
            .1
            .unwrap()
    }
    fn dup(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
