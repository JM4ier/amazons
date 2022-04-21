use super::*;

#[derive(PartialEq, Eq, Clone)]
pub struct GameState {
    pub board: Board,
    pub turn: Player,
}

impl GameState {
    pub fn new() -> Self {
        let black = Slot::Amazon(Player::Black);
        let white = Slot::Amazon(Player::White);

        let mut board = Board::default();
        board[(0, 3)] = black;
        board[(3, 0)] = black;
        board[(6, 0)] = black;
        board[(9, 3)] = black;

        board[(0, 6)] = white;
        board[(3, 9)] = white;
        board[(6, 9)] = white;
        board[(9, 6)] = white;

        Self {
            board,
            turn: Player::White,
        }
    }
    pub fn legal_move(&self, mov: Move) -> bool {
        if self.board[mov.from] != Slot::Amazon(self.turn) {
            return false;
        }
        if !mov.from.is_reachable(mov.to) || !mov.to.is_reachable(mov.arrow) {
            return false;
        }
        let mut board = self.board.clone();
        board[mov.from] = Slot::Empty;
        board.is_all_empty(mov.from, mov.to) && board.is_all_empty(mov.to, mov.arrow)
    }
    pub fn do_move(&mut self, mov: Move) {
        let source = self.board[mov.from];
        self.board[mov.from] = Slot::Empty;
        self.board[mov.to] = source;
        self.board[mov.arrow] = Slot::Arrow;
        self.turn = self.turn.enemy();
    }
    pub fn find_amazons(&self) -> Vec<Pos> {
        let mut res = Vec::with_capacity(4);
        for x in 0..BOARD_LEN {
            for y in 0..BOARD_LEN {
                let (x, y) = (x as u8, y as u8);
                if self.board[(x, y)] == Slot::Amazon(self.turn) {
                    res.push((x, y).into());
                }
            }
        }
        res
    }
}
