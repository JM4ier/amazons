use rand::prelude::*;

use super::*;
use std::collections::*;

#[derive(Copy, Clone)]
pub struct DeepIdiot {
    width: usize,
    depth: usize,
    memory: usize,
}

impl DeepIdiot {
    pub fn with_depth(depth: usize) -> Self {
        Self {
            depth,
            memory: 20,
            width: 1000,
        }
    }
}

#[derive(Copy, Clone)]
struct Valued<T> {
    val: i32,
    data: T,
}

impl<T> PartialEq for Valued<T> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}
impl<T> Eq for Valued<T> {}
impl<T> std::cmp::PartialOrd for Valued<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.val.cmp(&other.val))
    }
}
impl<T> std::cmp::Ord for Valued<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

impl Strategy for DeepIdiot {
    fn name(&self) -> String {
        format!("DeepIdiot({}, {}, {})", self.depth, self.width, self.memory)
    }
    fn find_move(&mut self, state: &GameState) -> Move {
        let me = state.turn;

        let mut rng = rand::thread_rng();
        let rng = &mut rng;

        let mut states = vec![vec![(0, state.clone())]];

        let mut initial_moves = vec![];

        for idx in 0..self.depth {
            let mut best_vals = BinaryHeap::new();

            for _ in 0..self.width {
                let states = states.last_mut().unwrap();

                let state_idx = rng.gen_range(0..states.len());
                let state = &mut states[state_idx].1;

                let mov = if state.is_finished() {
                    let z = (0, 0).into();
                    Move {
                        from: z,
                        to: z,
                        arrow: z,
                    }
                } else {
                    Random.find_move(&state)
                };

                state.do_move(mov);
                let mut val = Reachability.eval(state.turn, &state.board);
                state.undo_move(mov);

                if idx % 2 == 1 {
                    val = -val;
                }

                best_vals.push(Valued {
                    val,
                    data: (state_idx, mov),
                });
                if best_vals.len() > self.memory {
                    best_vals.pop();
                }
            }

            let new_states = best_vals
                .iter()
                .map(|v| {
                    let (si, mov) = v.data;
                    let mut state = states.last().unwrap()[si].1.clone();
                    state.do_move(mov);
                    (si, state)
                })
                .collect::<Vec<_>>();
            states.push(new_states);

            if idx == 0 {
                initial_moves = best_vals.iter().map(|v| v.data.1).collect();
            }
        }

        // finding the best outcome
        let mut best_val = MIN_VAL;
        let mut best_last = 0;
        for i in 0..self.memory {
            let val = Reachability.eval(me, &states.last().unwrap()[i].1.board);
            if val > best_val {
                best_val = val;
                best_last = i;
            }
        }

        // backtracking
        let mut mov_idx = best_last;
        for i in (2..self.depth).rev() {
            mov_idx = states[i][mov_idx].0;
        }

        initial_moves[mov_idx]
    }
    fn dup(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
