mod actions;
mod card;
mod deck;
mod game;

pub use actions::{GameAction, PlayerAction};
use card::Card;
pub use game::Game;

pub enum RoundOutcome {
    PlayerWin,
    PlayerBust,
    Player21,
    HouseWin,
    HouseBust,
    House21,
    Draw,
    Continue,
}

pub enum Player {
    User,
    Dealer,
}

pub trait Pretty {
    fn obscured(&self) -> String;
    fn prettify(&self) -> String;
}

impl Pretty for Vec<Card> {
    fn obscured(&self) -> String {
        let mut iter = self.iter();
        let first = iter.next();
        match first {
            Some(_data) => {
                let shown_cards = iter
                    .map(|card| card.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("[* *] {shown_cards}")
            }
            None => String::from(""),
        }
    }

    fn prettify(&self) -> String {
        self.iter()
            .map(|card| card.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}
