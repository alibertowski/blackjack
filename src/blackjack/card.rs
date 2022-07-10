#[derive(Copy, Clone, Debug)]
pub enum Card {
    Heart(u8),
    Spade(u8),
    Diamond(u8),
    Club(u8),
}