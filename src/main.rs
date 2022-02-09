#![allow(dead_code)]
mod games;
mod generators;
mod inputs;

use games::card_games::blackjack::*;

fn main() {
    let mut game = Game::new();
    game.do_round();
}
