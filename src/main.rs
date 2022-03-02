#![allow(dead_code)]
mod games;
mod generators;
mod base;

use games::blackjack;

fn main() {
    blackjack::do_game();
}
