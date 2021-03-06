use super::*;
use crate::ansi::*;
use core::ops::*;

pub const BOARD_LEN: usize = 10;

#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Board([[Slot; BOARD_LEN]; BOARD_LEN]);

impl Board {
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

    #[inline]
    pub fn contains(&self, pos: impl Into<Pos>) -> bool {
        let pos = pos.into();
        pos.x < BOARD_LEN as _ && pos.y < BOARD_LEN as _
    }

    /// Returns a vector of the furthest possible reachable positions
    pub fn reachable_from(&self, p: Pos) -> Vec<Pos> {
        let dirs = [0u8, 1u8, u8::MAX];

        let mut res = Vec::with_capacity(8);
        for &dir_x in &dirs {
            for &dir_y in &dirs {
                if (dir_x, dir_y) == (0, 0) {
                    continue;
                }

                let mut q = (p.x, p.y);
                let mut n;
                let mut possible = false;
                while {
                    n = (q.0.wrapping_add(dir_x), q.1.wrapping_add(dir_y));
                    self.contains(n) && self[n].is_empty()
                } {
                    q = n;
                    possible = true;
                }

                if possible {
                    res.push(q.into())
                }
            }
        }

        res
    }

    pub fn reach_count(&self, p: Pos) -> usize {
        let mut count = 0;

        macro_rules! _loop {
            ($count:ident, $p:ident, $( ( $opx:tt $valx:expr , $opy:tt $valy:expr ) ),*) => {
                let mut q;
                $(
                    q = $p;
                    while {
                        q.x = q.x $opx $valx;
                        q.y = q.y $opy $valy;
                        self.contains(q) && self[q].is_empty()
                    } {
                        $count += 1;
                    }
                )*
            }
        };

        _loop!(count, p,
            (- 1 , - 1),
            (- 1 , + 0),
            (- 1 , + 1),
            (+ 0 , - 1),
            (+ 0 , + 1),
            (+ 1 , - 1),
            (+ 1 , + 0),
            (+ 1 , + 1)
        );

        count
    }

    pub fn find_amazons(&self, player: Player) -> Vec<Pos> {
        let mut res = Vec::with_capacity(4);
        for x in 0..BOARD_LEN {
            for y in 0..BOARD_LEN {
                let (x, y) = (x as u8, y as u8);
                if self[(x, y)] == Slot::Amazon(player) {
                    res.push((x, y).into());
                }
            }
        }
        res
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
#[repr(u8)]
pub enum Slot {
    Empty,
    Arrow,
    Amazon(Player),
}

impl Slot {
    #[inline(always)]
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

#[test]
fn correct_distance() {
    let mut board = Board::default();

    use rand::prelude::*;
    let mut rng = rand::thread_rng();

    for _ in 0..50 {
        let x = rng.gen_range(0..10);
        let y = rng.gen_range(0..10);
        if board[(x, y)].is_empty() {
            board[(x, y)] = Slot::Arrow;
        }
    }

    let sum_a = board
        .find_amazons(Player::Black)
        .into_iter()
        .map(|a| {
            board
                .reachable_from(a)
                .into_iter()
                .map(|p| a.distance_to(p) as i32)
                .sum::<i32>()
        })
        .sum::<i32>();

    let sum_b = board
        .find_amazons(Player::Black)
        .into_iter()
        .map(|a| board.reach_count(a) as i32)
        .sum();
    assert_eq!(sum_a, sum_b);
}
