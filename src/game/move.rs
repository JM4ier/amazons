use super::*;
use std::fmt;

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
