use rand::Rng;
use std::io;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    let mut score: u32 = 0;

    let mut play = || -> u32 {
        let top_card = deck.0.pop();

        if let Some(card) = top_card {
            println!("{:?}", card);
            card.get_score()
        }
        else {
            0
        }
    };

    println!("Your Cards are: ");
    score += play();
    score += play();
    println!("Your score: {}", score);

    loop {
        println!("\nAnother card? [Y/N] ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input.to_uppercase() == "N" {
            break;
        } else if input.to_uppercase() == "Y" {
            score += play();
            println!("Your score: {}", score);

            if score == 21 {
                println!("BlackJack!");
            } else if score > 21 {
                println!("You Lost! Game Over!");
                break;
            } else {
                continue;
            }
        } 
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Card {
    pub suit: CardSuit,
    pub value: CardValue,
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

#[derive(Debug)]
pub struct Deck(Vec<Card>);

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

