use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Suit {
    Heart,
    Spade,
    Club,
    Diamond,
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let output = match self {
            Self::Heart => "❤️",
            Self::Spade => "♠️",
            Self::Club => "♣️",
            Self::Diamond => "♦️",
        };

        write!(f, "{output}")
    }
}
