mod engine;
mod input;

use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

use engine::{Game, GameAction, Player, RoundOutcome};
use input::{player, pre_game};

use crate::engine::PlayerAction;

pub fn show_hands(game: &Game, show_results: bool) {
    let (player_hand, dealer_hand) = game.show_hands(show_results);
    println!("{player_hand}");
    println!("{dealer_hand}");
}

pub fn run_game() {
    while matches!(pre_game("Wanna play a round of 21?"), GameAction::Start) {
        println!("let's play a game!");

        let mut game = Game::new();
        game.setup();

        loop {
            show_hands(&game, false);
            let round_outcome: RoundOutcome = {
                if game.start_round() {
                    let player_action = if game.has_stayed(Player::User) {
                        println!("> Player stayed.");
                        PlayerAction::Stay
                    } else {
                        player("Choose your action.")
                    };

                    if player_action == PlayerAction::Quit {
                        break;
                    }

                    let dealer_action = game.dealer_action();

                    if player_action == PlayerAction::Hit {
                        println!("> You hit.");
                        game.hit(Player::User);
                    } else {
                        println!("> You stay.");
                        game.stayed(Player::User);
                    }

                    if !game.did_bust(Player::User) {
                        println!("... Dealer mulls it over.");
                        for _ in 0..3 {
                            sleep(Duration::from_millis(500));
                            print!(".");
                            let _ = io::stdout().flush();
                        }

                        sleep(Duration::from_millis(500));
                        println!();

                        if dealer_action == PlayerAction::Hit {
                            println!("> Dealer hits.");
                            game.hit(Player::Dealer);
                        } else {
                            println!("> Dealer stays.");
                            game.stayed(Player::Dealer);
                        }
                    }
                }

                game.end_round()
            };

            if !matches!(round_outcome, RoundOutcome::Continue) {
                println!("<><><><><><><><><>");
                show_hands(&game, true);

                match round_outcome {
                    RoundOutcome::Player21 => println!("=> 21! You win!"),
                    RoundOutcome::PlayerWin => println!("=> You win!"),
                    RoundOutcome::PlayerBust => println!("=> Bust! You lose!"),
                    RoundOutcome::HouseWin => println!("=> You lose!"),
                    RoundOutcome::HouseBust => println!("=> House busts! You win!"),
                    RoundOutcome::House21 => println!("=> House has 21! You lose!"),
                    RoundOutcome::Draw => println!("=> It's a draw!"),
                    _ => {}
                }
                break;
            }
            println!("________");
        }

        println!("** Game over! **");
    }
}
