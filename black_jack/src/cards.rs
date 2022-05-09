#[derive(Debug, Copy, Clone)]
pub struct Card {
    pub suit: CardSuit,
    pub value: CardValue,
}

#[derive(Debug, Copy, Clone)]
pub enum CardSuit {
    Clubs,
    Hearts,
    Diamonds,
    Spades,
}

#[derive(Debug, Copy, Clone)]
pub enum CardValue {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Card {
    pub fn new(suit: CardSuit, value: CardValue) -> Card {
        Card {
            suit: suit,
            value: value,
        }
    }

    pub fn get_score(&self) -> u32 {
        match self.value {
            CardValue::Ace => 11,
            CardValue::Two => 2,
            CardValue::Three => 3,
            CardValue::Four => 4,
            CardValue::Five => 5,
            CardValue::Six => 6,
            CardValue::Seven => 7,
            CardValue::Eight => 8,
            CardValue::Nine => 9,
            CardValue::Ten => 10,
            _ => 10,
        }
    }
}