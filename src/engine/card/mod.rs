use std::fmt::{Display, Formatter, Result};

mod suit;
mod value;

pub use suit::Suit;
pub use value::Value;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Card {
    suit: Suit,
    value: Value,
}

impl Card {
    pub const fn new(suit: Suit, value: Value) -> Self {
        Self { suit, value }
    }

    pub fn score(&self, aces_high: bool) -> u8 {
        self.value.score(aces_high)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{} {}]", self.value, self.suit)
    }
}

pub trait Scoreable {
    fn score(&self) -> (u8, u8);
}

impl Scoreable for Vec<Card> {
    fn score(&self) -> (u8, u8) {
        self.iter().fold((0, 0), |(mut low, mut high), card| {
            low += card.score(false);
            high += card.score(true);
            (low, high)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_a_value() {
        assert_eq!(Card::new(Suit::Heart, Value::Ten).score(false), 10);
        assert_eq!(Card::new(Suit::Diamond, Value::Ace).score(false), 1);
        assert_eq!(Card::new(Suit::Diamond, Value::Ace).score(true), 11);
        assert_eq!(Card::new(Suit::Club, Value::King).score(false), 10);
        assert_eq!(Card::new(Suit::Spade, Value::Eight).score(false), 8);
        assert_eq!(Card::new(Suit::Heart, Value::Six).score(false), 6);
    }

    #[test]
    fn it_can_score_a_hand() {
        let hand = vec![Card::new(Suit::Heart, Value::Ace)];

        assert_eq!(hand.score(), (1, 11));

        let hand = vec![
            Card::new(Suit::Heart, Value::Ace),
            Card::new(Suit::Heart, Value::Nine),
        ];

        assert_eq!(hand.score(), (10, 20));

        let hand = vec![
            Card::new(Suit::Heart, Value::Ace),
            Card::new(Suit::Heart, Value::Six),
            Card::new(Suit::Heart, Value::Five),
        ];

        assert_eq!(hand.score(), (12, 22));
    }
}
