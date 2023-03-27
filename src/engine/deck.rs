use super::card::{Card, Suit, Value};

pub fn build() -> Vec<Card> {
    let mut deck = Vec::new();
    for value in [
        Value::Ace,
        Value::Two,
        Value::Three,
        Value::Four,
        Value::Five,
        Value::Six,
        Value::Seven,
        Value::Eight,
        Value::Nine,
        Value::Ten,
        Value::Jack,
        Value::Queen,
        Value::King,
    ] {
        for suit in [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade] {
            deck.push(Card::new(suit, value));
        }
    }
    deck
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn it_builds_decks() {
        let deck = build();

        assert_eq!(deck.len(), 52);

        let mut unique_cards = HashSet::<Card>::new();
        for card in deck {
            unique_cards.insert(card);
        }
        assert_eq!(unique_cards.len(), 52);
    }
}
