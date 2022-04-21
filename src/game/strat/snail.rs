use super::*;
use rand::prelude::*;

macro_rules! __snail__ {
    ($name:ident, $shuffle:expr) => {
        #[derive(Copy, Clone)]
        pub struct $name;

        impl Strategy for $name {
            fn name(&self) -> String {
                stringify!($name).into()
            }
            fn find_move(&mut self, state: &GameState) -> Move {
                let mut amzs = state.find_amazons();
                $shuffle(&mut amzs);

                for a in amzs {
                    let mut moves = a
                        .neighbors()
                        .into_iter()
                        .filter(|&m| state.board[m].is_empty())
                        .collect::<Vec<_>>();

                    $shuffle(&mut moves);
                    if moves.len() > 0 {
                        return Move {
                            from: a,
                            to: moves[0],
                            arrow: a,
                        };
                    }
                }

                panic!("find_move has been called on a finished amazons game")
            }
            fn dup(&self) -> Box<dyn Strategy> {
                Box::new(self.clone())
            }
        }
    }
}

__snail__!(RandomSnail, shuffle);
__snail__!(Snail, no_shuffle);

#[inline]
fn shuffle<T>(v: &mut Vec<T>) {
    v.shuffle(&mut rand::thread_rng());
}
#[inline]
fn no_shuffle<T>(_: &mut Vec<T>) {}
