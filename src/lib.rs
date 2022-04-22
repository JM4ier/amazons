mod ansi;
pub mod game;

pub use game::*;

pub fn repeat_games(strats: Vec<Box<dyn Strategy>>, reps: usize, self_games: bool) {
    let mut wins = vec![vec![(0, 0); strats.len()]; strats.len()];

    for (ai, a) in strats.iter().enumerate() {
        for (bi, b) in strats.iter().enumerate() {
            if bi > ai || (bi == ai && !self_games) {
                continue;
            }
            for _ in 0..reps {
                let winner0 = Game::new(a.dup(), b.dup()).finish();
                let winner1 = Game::new(b.dup(), a.dup()).finish();

                if winner0 == Player::White {
                    wins[ai][bi].0 += 1;
                } else {
                    wins[bi][ai].1 += 1;
                }
                if winner1 == Player::White {
                    wins[bi][ai].0 += 1;
                } else {
                    wins[ai][bi].1 += 1;
                }
            }
        }
    }

    for a in 0..strats.len() {
        for b in 0..strats.len() {
            if b > a || (b == a && !self_games) {
                continue;
            }
            let (w0, w1) = wins[a][b];
            let (e0, e1) = wins[b][a];
            let wt = w0 + w1;
            let et = e0 + e1;
            println!(
                "{: >20} vs {: >20}: {:0>4}-{:0>4} / {:0>4}-{:0>4} / {:0>4}-{:0>4}",
                strats[a].name(),
                strats[b].name(),
                w0,
                e0,
                w1,
                e1,
                wt,
                et
            );
        }
    }
}
