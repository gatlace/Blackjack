use crate::generators::cards::*;
use crate::inputs::*;

pub mod blackjack {
    use super::*;

    #[derive(Debug)]
    pub struct Player {
        name: String,
        hand: Vec<Card>,
        bet: usize,
        bank: usize,
    }

    impl Player {
        pub fn new() -> Player {
            println!("NEW PLAYER\n\
                     ___________\n");

            Player {
                name: get_string("name: "),
                hand: Vec::new(),
                bet: 0,
                bank: 0,

            }
        }
    }

    struct Dealer {
        pot: usize,
        deck: Vec<Card>,
    }

    impl Dealer {
        pub fn new() -> Dealer {
            Dealer {
            pot: 0,
            deck: Vec::new(),
            }
        }
    }

    fn game_start() -> (Dealer, Vec<Player>) {
        let num_players = get_usize("how many players? ");
        let mut vec_players: Vec<Player> = Vec::with_capacity(num_players);

        for _ in 1..=num_players {
            vec_players.push(Player::new());
        }

        (Dealer::new(), vec_players)
    }

    fn round_start(dealer: &mut Dealer, players: &mut Vec<Player>) {

    }
}