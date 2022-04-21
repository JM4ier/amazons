use core::fmt::Display;

const CSI: &str = "\x1B[";

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[allow(unused)]
pub enum Color {
    Reset = 0,
    Black = 30,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

pub struct StyleElem(String);

impl StyleElem {
    pub fn bold() -> Self {
        Self("22".into())
    }
}

pub struct ColorCode(u8);

impl From<ColorCode> for StyleElem {
    fn from(cc: ColorCode) -> Self {
        Self(format!("{}", cc.0))
    }
}

macro_rules! __color_code__ {
    ($($fn:ident $offset:expr),*) => {
        impl Color {$(
            pub fn $fn(self) -> ColorCode {
                ColorCode(if self == Self::Reset {
                    0
                } else {
                    self as u8 + $offset
                })
            }
        )*}
    }
}
__color_code__!(fg 0, bg 10, fg_bright 60, bg_bright 70);

#[derive(Clone)]
pub struct Style(String, bool);

impl<T: Into<StyleElem>> From<T> for Style {
    fn from(style_elem: T) -> Self {
        Self::new().with(style_elem)
    }
}

impl Style {
    pub fn new() -> Self {
        Self(CSI.into(), true)
    }
    fn add(self, effect: impl Display) -> Self {
        if self.1 {
            Self(format!("{}{}", self.0, effect), false)
        } else {
            Self(format!("{};{}", self.0, effect), false)
        }
    }
    pub fn with(self, style_elem: impl Into<StyleElem>) -> Self {
        self.add(style_elem.into().0)
    }
    pub fn combine(&self, other: &Self) -> Self {
        Self(format!("{}m{}", self.0, other.0), other.1)
    }
    pub fn fmt(&self, thing: impl Display) -> String {
        format!("{}m{}{}0m", self.0, thing, CSI)
    }
}
