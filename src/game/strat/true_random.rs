use super::*;
use rand::prelude::*;

#[derive(Copy, Clone)]
pub struct Random;

impl Strategy for Random {
    fn name(&self) -> String {
        "True Random".into()
    }
    fn find_move(&mut self, board: &GameState) -> Move {
        let mut rng = rand::thread_rng();
        let amazons = board.find_movable_amazons();
        let amazon = amazons[rng.gen_range(0..amazons.len())];

        let mut board = board.board.clone();

        let mut sample_rand = |board: &Board, pos| {
            let targets = board
                .reachable_from(pos)
                .into_iter()
                .flat_map(|t| pos.to(t))
                .collect::<Vec<_>>();
            targets[rng.gen_range(0..targets.len())]
        };

        let target = sample_rand(&board, amazon);
        board[amazon] = Slot::Empty;
        let arrow = sample_rand(&board, target);

        Move {
            from: amazon,
            to: target,
            arrow,
        }
    }
    fn dup(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
