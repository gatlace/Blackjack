use crate::generators::cards::*;
use crate::base::inputs::*;

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
            let bet = get_usize(&format!("{}, What would you like to bet?: ", self.name.to_uppercase()));
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
    /// returns true if dealer's hand <= player's hand <= 21 or dealer's hand > 21
    pub fn eval_cards(&mut self, dealer: &Dealer) -> bool {
        if dealer.hand_value() > 21 {
            true
        }
        else if self.hand_value() > 21 {
            false
        }
        else {
            if self.hand_value() > dealer.hand_value() {
                true
            }
            else {
                false
            }
        }
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

        let rounds = get_usize("How many rounds? ") as i8;
        println!("");

        Game {
            dealer: Dealer::new(),
            player: player,
            rounds: rounds,
        }
    }
    // TODO: one player games, pub display function
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

        for player in self.players.iter() {
            println!("{}'S HAND(value: {})", player.name.to_uppercase(), player.hand_value());
            print_cards(&player.hand);
        }

        let mut r3: Vec<Player> = Vec::new();
        for player in self.players.iter_mut() {
            player.third_round = get_bool(&format!("{}, Would you like a third card?", player.name.to_uppercase()));
            println!("");
            if player.third_round == true {
                r3.push(player.clone())
            } else {
                if player.eval_cards(&self.dealer) == true {
                    player.bank += player.bet * 2;
                    player.bet = 0;
                    println!("You win!");
                    println!("YOUR HAND:(value: {})", player.hand_value());
                    print_cards(&player.hand);
                    println!("DEALER HAND:(value: {})", self.dealer.hand_value());
                    print_cards(&self.dealer.hand);
                }
                else {
                    player.bet = 0;
                    println!("You lose!");
                    println!("YOUR HAND:(value: {})", player.hand_value());
                    print_cards(&player.hand);
                    println!("DEALER HAND:(value: {})", self.dealer.hand_value());
                    print_cards(&self.dealer.hand);
                }
            }
        }

        self.dealer.deal_cards(&mut r3);
        for player in r3.iter_mut() {
            if player.eval_cards(&self.dealer) == true {
                player.bank += player.bet * 2;
                player.bet = 0;
                println!("You win!");
                println!("YOUR HAND:(value: {})", player.hand_value());
                print_cards(&player.hand);
                println!("DEALER HAND:(value: {})", self.dealer.hand_value());
                print_cards(&self.dealer.hand);
            }
            else { 
                player.bet = 0;
                println!("You lose!");
                println!("YOUR HAND:(value: {})", player.hand_value());
                print_cards(&player.hand);
                println!("DEALER HAND:(value: {})", self.dealer.hand_value());
                print_cards(&self.dealer.hand);
            }
        }

        for player in self.players.iter_mut() {
            player.hand.clear();
        }
        self.dealer.hand.clear();
    }
}

pub fn do_game() {
    let mut game = Game::new();
    for _ in 0..game.rounds { game.do_round() }
    for player in game.players.iter() {
        println!("{}'S BANK: ${}", player.name.to_uppercase(), player.bank);
    }
}
