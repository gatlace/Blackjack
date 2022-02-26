#![allow(dead_code)]
mod games;
mod generators;
mod inputs;

use games::blackjack::*;

fn main() {
    let mut game = Game::new();
    game.do_game();
}
