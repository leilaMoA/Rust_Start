use rand::Rng;
use crate::cards::{Card,CardSuit, CardValue};

#[derive(Debug)]
pub struct Deck(pub Vec<Card>);

impl Deck {
        pub fn new() -> Deck {
            let values = vec![
                CardValue::Ace,
                CardValue::Two,
                CardValue::Three,
                CardValue::Four,
                CardValue::Five,
                CardValue::Six,
                CardValue::Seven,
                CardValue::Eight,
                CardValue::Nine,
                CardValue::Ten,
                CardValue::Jack,
                CardValue::Queen,
                CardValue::King,
                ];
    
            let mut deck: Vec<Card> = Vec::new();
    
            values.into_iter().for_each(|v| {
                deck.push(Card::new(CardSuit::Clubs, v));
                deck.push(Card::new(CardSuit::Diamonds, v));
                deck.push(Card::new(CardSuit::Hearts, v));
                deck.push(Card::new(CardSuit::Spades, v));
            });
    
            Deck(deck)
        }
    
        pub fn shuffle(&mut self) {
             let count = self.0.len();
    
            let mut shuffled_deck: Vec<Card> = Vec::new();
    
            loop {
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..self.0.len());
                
                shuffled_deck.push(self.0[index]);
                self.0.remove(index);
    
                if shuffled_deck.len() == count {
                    break;
                }
            }
            
            self.0 = shuffled_deck;
        }
    }