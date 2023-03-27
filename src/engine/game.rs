use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::cmp::Ordering;

use super::card::{Card, Scoreable};
use super::deck::build;
use super::{Player, Pretty, RoundOutcome};

use super::actions::PlayerAction;

#[derive(Default)]
pub struct Game {
    player: Vec<Card>,
    dealer: Vec<Card>,
    player_last_action: Option<PlayerAction>,
    dealer_last_action: Option<PlayerAction>,
    deck: Vec<Card>,
    rng: ThreadRng,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Vec::new(),
            dealer: Vec::new(),
            player_last_action: None,
            dealer_last_action: None,
            deck: build(),
            rng: rand::thread_rng(),
        }
    }

    pub fn did_bust(&mut self, player: Player) -> bool {
        self.check_score(player) > 21
    }

    fn deal(&mut self) -> Vec<Card> {
        self.deck.shuffle(&mut self.rng);
        let card1 = self.deck.pop().expect("ran out of cards");
        let card2 = self.deck.pop().expect("ran out of cards");

        vec![card1, card2]
    }

    pub fn stayed(&mut self, player: Player) {
        match player {
            Player::User => self.player_last_action = Some(PlayerAction::Stay),
            Player::Dealer => self.dealer_last_action = Some(PlayerAction::Stay),
        }
    }

    pub fn has_stayed(&self, player: Player) -> bool {
        let action = match player {
            Player::User => &self.player_last_action,
            Player::Dealer => &self.dealer_last_action,
        };

        matches!(action, Some(PlayerAction::Stay))
    }

    fn hand_score(hand: &Vec<Card>) -> u8 {
        let (low, high) = hand.score();
        if low == 21 || high == 21 {
            21
        } else if low > high {
            high
        } else {
            low
        }
    }

    pub fn check_score(&self, player: Player) -> u8 {
        let hand = match player {
            Player::User => &self.player,
            Player::Dealer => &self.dealer,
        };
        Self::hand_score(hand)
    }

    fn pretty_hands(&self, show_results: bool) -> (String, String) {
        let pretty_player = self.player.prettify();
        let pretty_dealer = if show_results {
            self.dealer.prettify()
        } else {
            self.dealer.obscured()
        };

        (pretty_player, pretty_dealer)
    }

    pub fn show_hands(&self, show_results: bool) -> (String, String) {
        let (player_hand, dealer_hand) = self.pretty_hands(show_results);
        let player_score = self.check_score(Player::User);
        let player_hand = format!("Your hand [{player_score}]: {player_hand}");
        let dealer_hand = if show_results {
            let dealer_score = self.check_score(Player::Dealer);
            // Shouldn't be building strings in here
            // should return some structure that the game loop can format however it wants
            format!("Dealer's hand [{dealer_score}]: {dealer_hand}")
        } else {
            format!("Dealer's hand [??]: {dealer_hand}")
        };
        (player_hand, dealer_hand)
    }

    pub fn hit(&mut self, player: Player) {
        self.deck.shuffle(&mut self.rng);
        let hit = self.deck.pop().expect("ran out of cards");

        match player {
            Player::User => {
                self.player.push(hit);
                self.player_last_action = Some(PlayerAction::Hit);
            }
            Player::Dealer => {
                self.dealer.push(hit);
                self.dealer_last_action = Some(PlayerAction::Hit);
            }
        }
    }

    pub fn dealer_action(&self) -> PlayerAction {
        if self.check_score(Player::Dealer) >= 18 {
            PlayerAction::Stay
        } else {
            PlayerAction::Hit
        }
    }

    pub fn setup(&mut self) {
        self.player = self.deal();
        self.dealer = self.deal();
    }

    pub fn start_round(&mut self) -> bool {
        self.check_score(Player::User) != 21 && self.check_score(Player::Dealer) != 21
    }

    pub fn end_round(&mut self) -> RoundOutcome {
        let player_score = self.check_score(Player::User);
        let dealer_score = self.check_score(Player::Dealer);
        let both_stayed = self.has_stayed(Player::User) && self.has_stayed(Player::Dealer);

        if both_stayed {
            match player_score.cmp(&dealer_score) {
                Ordering::Greater => RoundOutcome::PlayerWin,
                Ordering::Less => RoundOutcome::HouseWin,
                Ordering::Equal => RoundOutcome::Draw,
            }
        } else if player_score > 21 {
            RoundOutcome::PlayerBust
        } else if player_score == 21 {
            RoundOutcome::Player21
        } else if dealer_score > 21 {
            RoundOutcome::HouseBust
        } else if dealer_score == 21 {
            RoundOutcome::House21
        } else {
            RoundOutcome::Continue
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::card::{Suit, Value};
    use super::*;

    #[test]
    fn it_scores_21() {
        let mut game = Game::new();
        game.player = vec![
            Card::new(Suit::Heart, Value::Ace),
            Card::new(Suit::Heart, Value::Ten),
        ];

        game.dealer = vec![
            Card::new(Suit::Heart, Value::Ace),
            Card::new(Suit::Heart, Value::Ten),
        ];

        let score = Game::hand_score(&game.player);
        assert_eq!(score, 21);
        let score = Game::hand_score(&game.dealer);
        assert_eq!(score, 21);
    }

    #[test]
    fn it_deals_cards() {
        let mut game = Game::new();
        let hand = game.deal();

        assert_eq!(hand.len(), 2);
        assert_eq!(game.deck.len(), 50);
    }

    #[test]
    fn it_sets_up_the_game() {
        let mut game = Game::new();
        game.setup();

        assert_eq!(game.player.len(), 2);
        assert_eq!(game.dealer.len(), 2);
        assert_eq!(game.deck.len(), 48);
    }
}
