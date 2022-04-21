mod ansi;
use ansi::{Color, Style, StyleElem};

#[repr(u8)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Player {
    White,
    Black,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Slot {
    Empty,
    Arrow,
    Amazon(Player),
}

impl Default for Slot {
    fn default() -> Self {
        Self::Empty
    }
}

pub const BOARD_LEN: usize = 10;
pub type Board = [[Slot; BOARD_LEN]; BOARD_LEN];

pub type Pos = (u8, u8);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Move {
    pub from: Pos,
    pub to: Pos,
    pub arrow: Pos,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Game {
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
            checker_light: Style::new().with(Color::White.bg_bright()),
            checker_dark: Style::new().with(Color::White.bg()),
            amazon_light: Style::new().with(Color::Black.fg_bright()),
            amazon_dark: Style::new().with(Color::Black.fg()).with(StyleElem::bold()),
            arrow: Style::new().with(Color::Black.fg()).with(StyleElem::bold()),
        }
    }
}

impl Game {
    pub fn new() -> Self {
        let black = Slot::Amazon(Player::Black);
        let white = Slot::Amazon(Player::White);

        let mut board = Board::default();
        board[0][3] = black;
        board[3][0] = black;
        board[6][0] = black;
        board[9][3] = black;

        board[0][6] = white;
        board[3][9] = white;
        board[6][9] = white;
        board[9][6] = white;

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
                let style = if x % 2 == y % 2 { &styles.checker_light } else { &styles.checker_dark };
                buf += &match self.board[x][y] {
                    Slot::Empty => style.fmt("  "),
                    Slot::Arrow => style.fmt("<>"),
                    Slot::Amazon(Player::White) => style.combine(&styles.amazon_light).fmt("()"),
                    Slot::Amazon(Player::Black) => style.combine(&styles.amazon_dark).fmt("[]"),
                };
            }
            buf += "\n";
        }
        buf
    }
}


fn main() {
    print!("{}", Game::new().display(&BoardStyle::default()))
}
