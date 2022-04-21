mod ansi;
use ansi::{Color, Style, StyleElem};

use std::{ops::*, fmt, thread, time};
use rand::prelude::*;

#[repr(u8)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn enemy(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Slot {
    Empty,
    Arrow,
    Amazon(Player),
}

impl Slot {
    #[inline]
    pub fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }
}

impl Default for Slot {
    fn default() -> Self {
        Self::Empty
    }
}

pub const BOARD_LEN: usize = 10;
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Board([[Slot; BOARD_LEN]; BOARD_LEN]);

impl Board {
    pub fn is_all_empty(&self, mut a: Pos, b: Pos) -> bool {
        while {
            if !self[a].is_empty() {
                return false;
            }
            a = a.towards(b);
            a != b
        } {}
        true
    }
    pub fn is_trapped(&self, p: Pos) -> bool {
        p.neighbors().into_iter().all(|p| !self[p].is_empty())
    }
}

impl From<(u8, u8)> for Pos {
    fn from((x, y): (u8, u8)) -> Self {
        Self { x, y }
    }
}

impl<T: Into<Pos>> Index<T> for Board {
    type Output = Slot;
    fn index(&self, pos: T) -> &Slot {
        let pos = pos.into();
        &self.0[pos.x as usize][pos.y as usize]
    }
}

impl<T: Into<Pos>> IndexMut<T> for Board {
    fn index_mut(&mut self, pos: T) -> &mut Slot {
        let pos = pos.into();
        &mut self.0[pos.x as usize][pos.y as usize]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pos {
    pub x: u8,
    pub y: u8,
}

#[inline]
fn towards(a: u8, b: u8) -> u8 {
    if a < b {
        a + 1
    } else if a > b {
        a - 1
    } else {
        a
    }
}

impl Pos {
    #[inline]
    pub fn is_reachable(self, other: Self) -> bool {
        let xd = self.x.abs_diff(other.x);
        let yd = self.y.abs_diff(other.y);
        xd == yd || xd == 0 || yd == 0
    }
    #[inline]
    pub fn towards(self, goal: Self) -> Self {
        (towards(self.x, goal.x), towards(self.y, goal.y)).into()
    }
    #[inline]
    pub fn neighbors(self) -> Vec<Self> {
        let mut res = Vec::with_capacity(8);
        for x in self.x.saturating_sub(1)..(self.x + 2).min(BOARD_LEN as u8) {
            for y in self.y.saturating_sub(1)..(self.y + 2).min(BOARD_LEN as u8) {
                let p = (x, y).into();
                if p != self {
                    res.push(p);
                }
            }
        }
        res
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", (self.x + 0x61u8) as char, self.y)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Move {
    pub from: Pos,
    pub to: Pos,
    pub arrow: Pos,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}/{}", self.from, self.to, self.arrow)
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct GameState {
    pub board: Board,
    pub turn: Player,
}

pub struct BoardStyle {
    pub checker_light: Style,
    pub checker_dark: Style,
    pub amazon_light: Style,
    pub amazon_dark: Style,
    pub arrow: Style,
}

impl Default for BoardStyle {
    fn default() -> Self {
        Self {
            checker_light: Style::from(Color::White.bg_bright()),
            checker_dark: Style::from(Color::White.bg()),
            amazon_light: Style::from(Color::Black.fg_bright()),
            amazon_dark: Style::from(Color::Black.fg()).with(StyleElem::bold()),
            arrow: Style::from(Color::Black.fg()).with(StyleElem::bold()),
        }
    }
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
    pub fn display(&self, styles: &BoardStyle) -> String {
        let mut buf = "  ".to_string();
        for x in 0..BOARD_LEN as u8 {
            buf += &format!("{} ", (0x61u8 + x) as char);
        }
        buf += "\n";
        for y in 0..BOARD_LEN {
            buf += &format!("{} ", y);
            for x in 0..BOARD_LEN {
                let style = if x % 2 == y % 2 {
                    &styles.checker_light
                } else {
                    &styles.checker_dark
                };
                buf += &match self.board[(x as u8, y as u8)] {
                    Slot::Empty => style.fmt("  "),
                    Slot::Arrow => style.combine(&styles.arrow).fmt("::"),
                    Slot::Amazon(Player::White) => style.combine(&styles.amazon_light).fmt("()"),
                    Slot::Amazon(Player::Black) => style.combine(&styles.amazon_dark).fmt("[]"),
                };
            }
            buf += "\n";
        }
        buf
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

pub struct Game {
    state: GameState,
    white: Box<dyn Strategy>,
    black: Box<dyn Strategy>,
}

impl Game {
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
    pub fn is_finished(&self) -> bool {
        self.state.find_amazons().into_iter().all(|p| self.state.board.is_trapped(p))
    }
}

pub trait Strategy {
    fn name(&self) -> String;
    fn find_move(&mut self, board: &GameState) -> Move;
}

pub struct RandomSnail;

impl Strategy for RandomSnail {
    fn name(&self) -> String {
        "Random_Snail".into()
    }
    fn find_move(&mut self, state: &GameState) -> Move {
        let mut rng = rand::thread_rng();
        let mut amzs = state.find_amazons();

        amzs.shuffle(&mut rng);

        for a in amzs {
            let mut moves = a
                .neighbors()
                .into_iter()
                .filter(|&m| state.board[m].is_empty())
                .collect::<Vec<_>>();

            moves.shuffle(&mut rng);
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
}

fn main() {
    let style = BoardStyle::default();
    let mut game = Game {
        state: GameState::new(),
        white: Box::new(RandomSnail),
        black: Box::new(RandomSnail),
    };
    while !game.is_finished() {
        //ansi::clear_screen();
        game.next_move();
        println!("{}", game.state.display(&style));
        thread::sleep(time::Duration::from_secs_f64(0.1));
    }

    println!("Oopsie Woopsie, Player {:?} is stuck :((", game.state.turn);
}
