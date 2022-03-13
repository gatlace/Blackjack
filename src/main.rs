#![allow(dead_code)]
mod base;
mod games;
mod generators;

use games::blackjack;

fn main() {
    blackjack::do_game();
}


//TODO:
// - save player data with a csv file
// - craps
// - GUI for all of this
// - I might have to rig it, I got a $1500 bank in 5 rounds