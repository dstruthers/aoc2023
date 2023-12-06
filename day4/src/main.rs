use std::fs;

use day4::{CardPile, ScratchCard};

const INPUT_FILE: &str = "input.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cards: Vec<ScratchCard> = fs::read_to_string(INPUT_FILE)?
        .lines()
        .map(|l| l.parse())
        .filter(|r| match r {
            Ok(_) => true,
            _ => false,
        })
        .map(|r| r.unwrap())
        .collect();

    let solution1: usize = cards.iter().map(|c| c.score()).sum();
    println!("Solution 1: {solution1}");
    assert_eq!(solution1, 23941);

    let solution2 = CardPile::from(cards).play().len();
    println!("Solution 2: {solution2}");
    assert_eq!(solution2, 5571760);
    Ok(())
}
