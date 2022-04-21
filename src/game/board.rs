use super::*;
use crate::ansi::*;
use core::ops::*;

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
                buf += &match self[(x as u8, y as u8)] {
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
