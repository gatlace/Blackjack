#![allow(dead_code)]
mod generators;
mod games;
mod inputs;

fn main() {
    let player = games::card_games::blackjack::Player::new();
    println!("{:?}", player);
}
