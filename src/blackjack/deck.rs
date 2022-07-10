use crate::blackjack::card::Card;
use rand::{self, seq::SliceRandom};

pub struct Deck {
    live: Vec<Card>,
    dead: Vec<Card>
}

impl Deck {
    fn shuffle(&mut self) {
        for dead_card in self.dead.iter() {
            self.live.push(*dead_card);
        }
        self.dead.clear();

        let mut rng = rand::thread_rng();
        self.live.shuffle(&mut rng);
    }

    pub fn get_card(&mut self) -> Card {
        let taken_card = match self.live.pop() {
            Some(taken_card) => taken_card,
            None => {
                self.shuffle();
                if let Some(new_card) = self.live.pop() {
                    new_card
                } else {
                    panic!("A shuffled deck was expected to have a card in it!");
                }
            }
        };

        self.dead.push(taken_card);
        taken_card
    }

    pub fn new() -> Self {
        let mut new_deck = Self {
            live: vec! {
                Card::Club(1),
                Card::Club(2),
                Card::Club(3),
                Card::Club(4),
                Card::Club(5),
                Card::Club(6),
                Card::Club(7),
                Card::Club(8),
                Card::Club(9),
                Card::Club(10),
                Card::Club(11),
                Card::Club(12),
                Card::Club(13),

                Card::Heart(1),
                Card::Heart(2),
                Card::Heart(3),
                Card::Heart(4),
                Card::Heart(5),
                Card::Heart(6),
                Card::Heart(7),
                Card::Heart(8),
                Card::Heart(9),
                Card::Heart(10),
                Card::Heart(11),
                Card::Heart(12),
                Card::Heart(13),

                Card::Spade(1),
                Card::Spade(2),
                Card::Spade(3),
                Card::Spade(4),
                Card::Spade(5),
                Card::Spade(6),
                Card::Spade(7),
                Card::Spade(8),
                Card::Spade(9),
                Card::Spade(10),
                Card::Spade(11),
                Card::Spade(12),
                Card::Spade(13),

                Card::Diamond(1),
                Card::Diamond(2),
                Card::Diamond(3),
                Card::Diamond(4),
                Card::Diamond(5),
                Card::Diamond(6),
                Card::Diamond(7),
                Card::Diamond(8),
                Card::Diamond(9),
                Card::Diamond(10),
                Card::Diamond(11),
                Card::Diamond(12),
                Card::Diamond(13)
            },
            dead: Vec::new()
        };

        let mut rng = rand::thread_rng();
        new_deck.live.shuffle(&mut rng);

        new_deck
    }
}
