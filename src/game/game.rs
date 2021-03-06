use super::*;

pub type BStrat = Box<dyn Strategy>;

pub struct Game {
    state: GameState,
    log: Vec<Move>,
    enable_log: bool,
    white: BStrat,
    black: BStrat,
}

impl Game {
    pub fn new(white: BStrat, black: BStrat) -> Self {
        Self {
            state: GameState::new(),
            white,
            black,
            log: Vec::new(),
            enable_log: false,
        }
    }
    pub fn enable_log(&mut self) {
        self.enable_log = true;
    }
    pub fn log(&self) -> &[Move] {
        &self.log
    }
    #[inline]
    fn do_move(&mut self, mov: Move) {
        self.state.do_move(mov);
        if self.enable_log {
            self.log.push(mov);
        }
    }
    pub fn next_move(&mut self) -> Move {
        let player = match self.state.turn {
            Player::White => &mut self.white,
            Player::Black => &mut self.black,
        };
        let mov = player.find_move(&self.state);
        if self.state.legal_move(mov) {
            self.do_move(mov);
            mov
        } else {
            eprintln!("{}", self.state.board.display(&BoardStyle::default()));
            panic!(
                "Player {:?} with strategy '{}' tried to do the illegal move {}.",
                self.state.turn,
                player.name(),
                mov
            );
        }
    }
    /// plays the current game to the end and returns the winner
    pub fn finish(&mut self) -> Player {
        while !self.state.is_finished() {
            self.next_move();
        }
        self.state.turn.enemy()
    }

    /// an iterator representing the moves of the remaining game
    pub fn as_iter(&mut self) -> GameIter {
        GameIter { game: self }
    }
}

pub struct GameIter<'a> {
    game: &'a mut Game,
}

impl Iterator for GameIter<'_> {
    type Item = Move;
    fn next(&mut self) -> Option<Move> {
        if self.game.state.is_finished() {
            None
        } else {
            Some(self.game.next_move())
        }
    }
}
