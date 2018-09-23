
mod deck;

use deck::Hand;


fn main() {
    let mut d: Vec<String> = deck::generate();
    deck::shuffle(&mut d);
    // println!("{:?}", d);

    let mut h: Hand = Hand {
        own: Vec::new(),
        common: Vec::new()
    };
    h.insert_own(&d[0..2]);
    h.insert_common(&d[2..7]);

    // h.insert_own(&vec![String::from("AD"), String::from("QD"), String::from("JH"), String::from("JD"), String::from("TH"), String::from("9H"), String::from("8H")]);
    // h.insert_own(&vec![String::from("AD"), String::from("QD"), String::from("AH"), String::from("QS"), String::from("TH"), String::from("9H"), String::from("8H")]);
    // h.insert_own(&vec![String::from("AD"), String::from("QD"), String::from("KD"), String::from("JD"), String::from("TD"), String::from("9D"), String::from("8D")]);
    // h.insert_own(&vec![String::from("2D"), String::from("4D"), String::from("6D"), String::from("JD"), String::from("TD"), String::from("AH"), String::from("KH")]);
    // h.insert_own(&vec![String::from("2D"), String::from("2C"), String::from("2H"), String::from("3H"), String::from("3D"), String::from("AH"), String::from("KH")]);
    // h.insert_own(&vec![String::from("2D"), String::from("3C"), String::from("4H"), String::from("5H"), String::from("6D"), String::from("TH"), String::from("9H")]);

    println!("HAND {:?}", h.get_cards());
    
    let r = h.get_result();

    println!("{:?}", r);
}



// fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
//     assert!(a < b);
//     if a == 0 {
//         return (b, 0, 1);
//     }
//     else {
//         let (g, x, y) = egcd(b % a, a);
//         return (g, y - (b / a) * x, x);
//     }
// }