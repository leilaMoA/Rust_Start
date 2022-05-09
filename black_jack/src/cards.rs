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
    Numeral(u32),
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
            CardValue::Numeral(n) => {
                if n == 1 {
                    11
                } else {
                    n
                }
            }
            _ => 10,
        }
    }
}