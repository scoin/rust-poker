use deck::Card;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Hand {
    pub own: Vec<Card>,
    pub common: Vec<Card>
}

#[derive(Debug)]
pub enum HandResult {
    None,
    HighCard(Card),
    Pair(Vec<Card>),
    TwoPair(Vec<Card>, Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    Straight(Vec<Card>),
    Flush(Vec<Card>),
    FullHouse(Vec<Card>, Vec<Card>),
    FourOfAKind(Vec<Card>),
    StraightFlush(Vec<Card>),
    RoyalFlush(Vec<Card>)
}


impl Hand {
    pub fn insert_own(& mut self, cards: &[String]) {
        for i in cards {
            let c: Card = Card {
                rank: i.chars().next().unwrap(),
                suit: i.chars().nth(1).unwrap()
            };
            self.own.push(c);
        }
    }

    pub fn insert_common(& mut self, cards: &[String]) {
        for i in cards {
            let c: Card = Card {
                rank: i.chars().next().unwrap(),
                suit: i.chars().nth(1).unwrap()
            };
            self.common.push(c);
        }
    }

    pub fn get_cards(&self) -> Vec<Card> {
        let mut newcards: Vec<Card> = self.own.to_vec();
        newcards.extend(&self.common);
        newcards
    }

    fn sort_values(&self) -> Vec<Card> {
        let mut newcards = self.get_cards();
        newcards.sort_by(|a,b| {
            b.value().cmp(&a.value())
        });
        newcards
    }

    fn sort_suits(&self) -> Vec<Card> {
        let mut newcards = self.get_cards();
        newcards.sort_by(|a,b| {
            let o = b.suit.cmp(&a.suit);
            match o {
                Ordering::Equal => b.value().cmp(&a.value()),
                _ => o
            }
        });
        newcards
    }

    pub fn get_result(&self) -> HandResult {

        match (self.is_flush(), self.is_straight(), self.matching_cards()) {
            (Some(_), Some(cards), _) => {
                if cards[0].rank == 'A' {
                    HandResult::RoyalFlush(cards)
                } else {
                    HandResult::StraightFlush(cards)
                }
            },
            (_, _, HandResult::FourOfAKind(c)) => HandResult::FourOfAKind(c),
            (_, _, HandResult::FullHouse(c, d)) => HandResult::FullHouse(c, d),
            (Some(cards), None, _) => HandResult::Flush(cards),
            (None, Some(cards), _) => HandResult::Straight(cards),
            (None, None, HandResult::ThreeOfAKind(c)) => HandResult::ThreeOfAKind(c),
            (None, None, HandResult::TwoPair(c,d)) => HandResult::TwoPair(c,d),
            (None, None, HandResult::Pair(c)) => HandResult::Pair(c),
            _ => HandResult::HighCard(self.sort_values()[0])
        }
    }

    pub fn matching_cards(&self) -> HandResult {
        let cards: Vec<Card> = self.sort_values();
        let mut total: Vec<Vec<Card>> = Vec::new();
        let mut i: usize = 0;
        let mut j: usize = 1;
        let mut pair: Vec<Card> = Vec::new();
        loop {
            if j == cards.len() {
                if pair.len() > 0 {
                    total.push(pair);
                }
                break
            }
            if cards[i].rank == cards[j].rank && pair.len() == 0 {
                pair.push(cards[i]);
                pair.push(cards[j]);
            } else if cards[i].rank == cards[j].rank {
                pair.push(cards[j]);
            } else if pair.len() > 0 {
                total.push(pair);
                pair = Vec::new();
            }
            i += 1;
            j += 1;
        }
        match total.len() {
            1 => {
                match total[0].len() {
                    4 => HandResult::FourOfAKind(total[0].to_vec()),
                    3 => HandResult::ThreeOfAKind(total[0].to_vec()),
                    _ => HandResult::Pair(total[0].to_vec())
                }
            },
            2...4 => {
                total.sort_by(|a,b| {
                    b.len().cmp(&a.len())
                });
                

                match total[0].len() {
                    4 => HandResult::FourOfAKind(total[0].to_vec()),
                    3 => HandResult::FullHouse(total[0].to_vec(), total[1][0..2].to_vec()),
                    _ => HandResult::TwoPair(total[0].to_vec(), total[1].to_vec())
                }
            },
            _ => HandResult::None
        }
    }

    pub fn is_straight(&self) -> Option<Vec<Card>> {
        let cards: Vec<Card> = self.sort_values();
        let mut result: Vec<Card> = Vec::new();
        let mut i: usize = 0;
        let mut j: usize = 1;
        
        loop {
            if j == cards.len() || result.len() == 5 {
                break
            }

            if cards[i].value() - cards[j].value() == 1 && result.len() == 0{
                result.push(cards[i]);
                result.push(cards[j]);
            }
            else if cards[i].value() - cards[j].value() == 1 {
                result.push(cards[j]);
            } 
            else if cards[i].value() - cards[j].value() > 1 {
                result = Vec::new();
            }

            i += 1;
            j += 1;
        }
        match result.len() {
            5 => Some(result),
            _ => None
        }
    }

    pub fn is_flush(&self) -> Option<Vec<Card>> {
        let cards: Vec<Card> = self.sort_suits();
        let mut result: Vec<Card> = Vec::new();
        let mut i: usize = 0;
        let mut j: usize = 1;
        loop {
            if j == cards.len() || result.len() == 5 {
                break
            }

            if cards[i].suit == cards[j].suit && result.len() == 0 {
                result.push(cards[i]);
                result.push(cards[j]);
            } else if cards[i].suit == cards[j].suit {
                result.push(cards[j]);
            } else {
                result = Vec::new();
            }

            i += 1;
            j += 1;
        }
        match result.len() {
            5 => Some(result),
            _ => None
        }
    }
}
