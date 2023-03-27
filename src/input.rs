use crate::engine::{GameAction, PlayerAction};
use inquire::{error::InquireError, Select};

pub fn pre_game(prompt: &str) -> GameAction {
    let options: Vec<GameAction> = vec![GameAction::Start, GameAction::Quit];
    let ans: Result<GameAction, InquireError> = Select::new(prompt, options).prompt();
    ans.unwrap_or(GameAction::Quit)
}

pub fn player(prompt: &str) -> PlayerAction {
    let options: Vec<PlayerAction> =
        vec![PlayerAction::Hit, PlayerAction::Stay, PlayerAction::Quit];
    let ans: Result<PlayerAction, InquireError> = Select::new(prompt, options).prompt();
    ans.unwrap_or(PlayerAction::Quit)
}
