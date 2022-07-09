use card::Card;

mod card;
mod deck;

const CARDS_IN_DECK: usize = 52;

pub fn generate_deck() -> [Card; CARDS_IN_DECK] {
    [
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
    ]
}