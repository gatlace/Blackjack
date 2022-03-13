use crate::base::inputs::*;
use crate::generators::cards::*;

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

    fn place_bet(&mut self) {
        loop {
            println!("BANK: {}", self.bank);
            println!("--------");
            let bet = get_usize(&format!(
                "{}, What would you like to bet?: ",
                self.name.to_uppercase()
            ));
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

    pub fn deal_cards(&mut self, player: &mut Player) {
        player.hand.push(self.deck.pop().unwrap());
        self.hand.push(self.deck.pop().unwrap());
    }

    pub fn hand_value(&self) -> i8 {
        let mut value: i8 = 0;
        let mut first_ace = false;
        for card in self.hand.iter() {
            if card.value == 1 {
                if first_ace == false {
                    first_ace = true;
                    value += 11;
                    continue;
                } else {
                    value += card.value;
                }
            }
            value += card.value;
        }
        value
    }
}

pub struct Game {
    player: Player,
    dealer: Dealer,
    rounds: i8,
}

impl Game {
    pub fn new() -> Game {
        // TODO: hashmap that maps casino players to blackjack players
        // Have the blackjack player do all the work and pass the winnings(or losses lol)
        // on to update the player

        let player = Player::new();

        Game {
            dealer: Dealer::new(),
            player: player,
            rounds: 0,
        }
    }
    // TODO: pub display function to display multiple players' cards on the table
    pub fn do_round(&mut self) {
        if self.dealer.deck.len() < 11 {
            println!("Reshuffling Deck...");
            self.dealer.deck = new_random_deck();
        }

        self.player.place_bet();

        println!("Dealing cards...\n");
        for _ in 0..2 {
            self.dealer.deal_cards(&mut self.player);
        }

        println!(
            "{}'S HAND(value: {})",
            self.player.name,
            self.player.hand_value()
        );
        print_cards(&self.player.hand);

        self.player.third_round = get_bool(&format!(
            "{}, Would you like a third card?",
            self.player.name
        ));
        println!("");
        match self.player.third_round {
            false => {
                Game::determine_winner(&mut self.player, &mut self.dealer);
            }
            true => {
                self.dealer.deal_cards(&mut self.player);
                Game::determine_winner(&mut self.player, &mut self.dealer)
            }
        }
        self.player.hand.clear();
        self.dealer.hand.clear();
    }

    fn determine_winner(player: &mut Player, dealer: &mut Dealer) {
        let player_won = {
            if dealer.hand_value() > 21 {
                true
            }
            else if player.hand_value() > 21 {
                false
            }
            else {
                if player.hand_value() > dealer.hand_value() {
                    true
                }
                else {
                    false
                }
            }
        };

        match player_won {
            true => {
                player.bank += player.bet * 2;
                player.bet = 0;
                println!("You win!");
                println!("YOUR HAND:(value: {})", player.hand_value());
                print_cards(&player.hand);
                println!("DEALER HAND:(value: {})", dealer.hand_value());
                print_cards(&dealer.hand);
            },
            false => {
                player.bet = 0;
                println!("You lose!");
                println!("YOUR HAND:(value: {})", player.hand_value());
                print_cards(&player.hand);
                println!("DEALER HAND:(value: {})", dealer.hand_value());
                print_cards(&dealer.hand);
            }
        }
    }
}

pub fn do_game() {
    let mut game = Game::new();
    game.do_round();
    game.rounds += 1;

    loop {
        match get_bool("Would you like to quit? y/es | n/o") {
            false => game.do_round(),
            true => { 
                match game.player.bank > 500 {
                    true => { println!("Congrats! You won {} this session!", game.player.bank - 500)},
                    false => {
                        println!("Oh no! Better luck next time! You lost {} this session.", 500 - game.player.bank);
                        break;
                    }
                }
             }
        }
    }

}
