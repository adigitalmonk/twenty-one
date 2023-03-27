use std::fmt::{Display, Formatter, Result};

#[derive(Eq, PartialEq)]
pub enum PlayerAction {
    Hit,
    Stay,
    Quit,
}

impl Display for PlayerAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let output = match self {
            Self::Hit => "Hit",
            Self::Stay => "Stay",
            Self::Quit => "Quit",
        };
        write!(f, "{output}")
    }
}

pub enum GameAction {
    Start,
    Quit,
}

impl Display for GameAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let output = match self {
            Self::Start => "Start",
            Self::Quit => "Quit",
        };
        write!(f, "{output}")
    }
}
