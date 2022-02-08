use rand::seq::SliceRandom;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Card {
    suit: Suit,
    value: i8,
}

#[derive(Clone, Copy, Debug)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

const VALUE_NAMES: [&str; 13] = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suit = match &self.suit {
            Suit::Clubs => "Clubs", Suit::Diamonds => "Diamonds", Suit::Hearts => "Hearts", Suit::Spades => "Spades", };
        let  value = VALUE_NAMES[self.value as usize - 1];

        write!(f, "{} of {}", value, suit)
    }

}

pub fn print_deck(deck: &Vec<Card>) {
    for card in deck.iter() {
        println!("{}", card);
    }
}

pub fn new_sorted_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::with_capacity(52);

    for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
        for value in 1..=13 {
            deck.push(Card {
                suit: suit,
                value:value,
            })
        }
    }

    deck
}

pub fn randomize_deck(deck: &mut Vec<Card>) {
    let mut rng = rand::thread_rng();

    deck.shuffle(&mut rng);
}

pub fn new_random_deck() -> Vec<Card> {
    let mut deck = new_sorted_deck();
    randomize_deck(&mut deck);

    deck
}