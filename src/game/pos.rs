use super::*;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pos {
    pub x: u8,
    pub y: u8,
}

impl Pos {
    #[inline]
    pub fn distance_to(self, other: Self) -> u8 {
        self.x.abs_diff(other.x).max(self.y.abs_diff(other.y))
    }

    #[inline]
    pub fn is_reachable(self, other: Self) -> bool {
        let xd = self.x.abs_diff(other.x);
        let yd = self.y.abs_diff(other.y);
        xd == yd || xd == 0 || yd == 0
    }

    #[inline]
    pub fn towards(self, goal: Self) -> Self {
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
        (towards(self.x, goal.x), towards(self.y, goal.y)).into()
    }

    /// Returns an iterator of positions that go towards `goal` until it is reached.
    /// Starting position (`self`) *is not* included, end position *is* included.
    pub fn to(self, goal: Self) -> PosIter {
        PosIter {
            from: self,
            to: goal,
        }
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

impl From<(u8, u8)> for Pos {
    fn from((x, y): (u8, u8)) -> Self {
        Self { x, y }
    }
}

pub struct PosIter {
    from: Pos,
    to: Pos,
}

impl Iterator for PosIter {
    type Item = Pos;
    fn next(&mut self) -> Option<Pos> {
        if self.from == self.to {
            None
        } else {
            self.from = self.from.towards(self.to);
            Some(self.from)
        }
    }
}
