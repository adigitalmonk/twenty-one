#![deny(clippy::pedantic)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::suspicious)]
#![deny(clippy::complexity)]
#![deny(clippy::style)]

use twenty_one::run_game;

fn main() {
    run_game();

    println!("Goodbye!");
}
