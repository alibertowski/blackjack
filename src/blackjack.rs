use hand::Hand;

mod card;
mod hand;
pub mod deck;

struct Game {
    dealer: Hand,
    players: Vec<Hand>
}

impl Game {
    pub fn new() -> Self {
        Self {
            dealer: Hand{},
            players: Vec::new()
        }
    }

    pub fn set_dealer(&mut self, dealer: Hand) {
        self.dealer = dealer;
    }
    
    pub fn add_player(&mut self, player: Hand) {
        self.players.push(player);
    }
}