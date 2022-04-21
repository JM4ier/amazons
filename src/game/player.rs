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
