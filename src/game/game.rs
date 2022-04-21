use super::*;

pub type BStrat = Box<dyn Strategy>;

pub struct Game {
    state: GameState,
    white: BStrat,
    black: BStrat,
}

impl Game {
    pub fn new(white: BStrat, black: BStrat) -> Self {
        Self {
            state: GameState::new(),
            white,
            black,
        }
    }
    pub fn next_move(&mut self) {
        let player = match self.state.turn {
            Player::White => &mut self.white,
            Player::Black => &mut self.black,
        };
        let mov = player.find_move(&self.state);
        if self.state.legal_move(mov) {
            self.state.do_move(mov);
        } else {
            panic!(
                "Player {:?} with strategy {} tried to do the illegal move {}.",
                self.state.turn,
                player.name(),
                mov
            );
        }
    }
    /// plays the current game to the end and returns the winner
    pub fn finish(&mut self) -> Player {
        while !self.is_finished() {
            self.next_move();
        }
        self.state.turn.enemy()
    }
    pub fn is_finished(&self) -> bool {
        self.state.find_amazons().into_iter().all(|p| self.state.board.is_trapped(p))
    }
}

