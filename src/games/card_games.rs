use crate::generators::cards::*;
use crate::inputs::*;

pub mod blackjack {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct Player {
        name: String,
        hand: Vec<Card>,
        bet: usize,
        bank: usize,
        third_round: bool,
    }

    impl Player {
        pub fn new() -> Player {
            println!("NEW PLAYER");
            println!("__________");

            Player {
                name: get_string("name: "),
                hand: Vec::new(),
                bet: 0,
                bank: 500,
                third_round: false,
            }
        }

        pub fn place_bet(&mut self) {
            loop {
                println!("BANK: {}", self.bank);
                println!("--------");
                let bet = get_usize("What would you like to bet?: ");
                if bet > self.bank {
                    println!("Error: your bet cannot be bigger than your bank");
                    continue;
                }
                self.bet = bet;
                self.bank -= bet;
                println!("");
                break;
            }
        }

        pub fn hand_value(&self) -> i8 {
            let mut value: i8 = 0;

            for card in self.hand.iter() {
                value += card.value;
            }

            value
        }
    }

    pub struct Dealer {
        hand: Vec<Card>,
        deck: Vec<Card>,
    }

    impl Dealer {
        pub fn new() -> Dealer {
            Dealer {
                hand: Vec::new(),
                deck: new_random_deck(),
            }
        }

        pub fn deal_cards(&mut self, players: &mut Vec<Player>) {
            for player in players.iter_mut() {
                player.hand.push(self.deck.pop().unwrap());
                self.hand.push(self.deck.pop().unwrap());
            }
        }
    }

    pub struct Game {
        players: Vec<Player>,
        dealer: Dealer,
        rounds: i8,
    }

    impl Game {
        pub fn new() -> Game {
            let num_players = get_usize("how many players? ");
            let mut vec_players: Vec<Player> = Vec::with_capacity(num_players);
            println!("");

            for _ in 1..=num_players {
                vec_players.push(Player::new());
                println!("");
            }

            let rounds = get_usize("How many rounds? ") as i8;
            println!("");
            
            Game {
                dealer: Dealer::new(),
                players: vec_players,
                rounds: rounds,
            }
        }

        pub fn do_round(&mut self) {
            if self.dealer.deck.len() < 11 {
                println!("Reshuffling Deck...");
                self.dealer.deck = new_random_deck();
            }

            for player in &mut self.players {
                player.place_bet();
            }

            println!("Dealing cards...\n");
            for _ in 0..2 {
                self.dealer.deal_cards(&mut self.players);
            }

            for player in self.players.iter_mut() {
                print_cards(&player.hand);
            }
        }
    }
}
