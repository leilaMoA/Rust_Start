use std::io;

const WINNING_SCORE: u32 = 21; 

fn main() {
    let mut deck = decks::Deck::new();
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

            if score == WINNING_SCORE {
                println!("BlackJack!");
            } else if score > WINNING_SCORE {
                println!("You Lost! Game Over!");
                break;
            } else {
                continue;
            }
        } 
    }
}

mod cards;
mod decks;
