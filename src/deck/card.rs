#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub rank: char,
    pub suit: char
}

impl Card {
    pub fn value(&self) -> i8 {
        match self.rank {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => {
                println!("ERROR!!!"); 1
            }
        }
    }
}