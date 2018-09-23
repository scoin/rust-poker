extern crate rand;

mod card;
mod hand;

pub use self::card::Card;
pub use self::hand::Hand;
pub use self::hand::HandResult;

use self::rand::Rng;

pub fn generate() -> Vec<String> {
    let mut deck = Vec::new();
    for r in vec!['A','2','3','4','5','6','7','8','9','T','J','Q','K'] {
        for s in vec!['S', 'C', 'D', 'H'] {
            deck.push(format!("{}{}", &r, &s))
        }
    }
    deck
}

pub fn shuffle(deck: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    rng.shuffle(deck);
}