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
        let mut amazons = board.find_movable_amazons();
        amazons.shuffle(&mut rng);
        let amazon = amazons[0];

        let mut board = board.board.clone();

        let mut sample_rand = |board: &Board, pos| {
            let mut targets = board
                .reachable_from(pos)
                .into_iter()
                .flat_map(|t| pos.to(t))
                .collect::<Vec<_>>();
            targets.shuffle(&mut rng);
            targets[0]
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
