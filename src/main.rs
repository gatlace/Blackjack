#![allow(dead_code)]
mod games;
mod generators;
mod inputs;

use games::blackjack;

fn main() {
    blackjack::do_game();
}
