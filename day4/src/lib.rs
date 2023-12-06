use regex::Regex;
use std::{
    collections::{
        HashMap,
        HashSet,
    },
    error::Error,
    fmt,
    str::FromStr,
};

#[derive(Debug)]
pub struct CardPile {
    pub cards: HashMap<usize, ScratchCard>,
}

impl CardPile {
    pub fn from<T>(v: T) -> Self 
    where T: IntoIterator<Item = ScratchCard> {
        let mut cards = HashMap::new();
        for c in v {
            cards.insert(c.id, c);
        }
        CardPile { cards }
    }

    pub fn play(&self) -> Vec<&ScratchCard> {
        let mut card_copies = HashMap::new();
        for i in 1..=self.cards.len() {
            card_copies.insert(i, 1);
        }
        for i in 1..=self.cards.len() {
            let card = self.cards.get(&i).unwrap();
            let current_copies = card_copies.get(&i).unwrap();

            for _ in 0..*current_copies {
                for n in 1..=card.winning_numbers().len() {
                    println!("Adding copy of card {}", i + n);
                    let cs = card_copies.get(&(i + n)).unwrap();
                    card_copies.insert(i + n, cs + 1);
                }
            }
        }
        let mut winnings = Vec::new();
        for i in 1..=self.cards.len() {
            let card = self.cards.get(&i).unwrap();
            let copies = card_copies.get(&i).unwrap();
            for _ in 0..*copies {
                winnings.push(card);
            }
        }
        winnings
    }
}

#[derive(Debug)]
pub struct ScratchCard {
    id: usize,
    winners: HashSet<usize>,
    numbers: HashSet<usize>,
}

impl ScratchCard {
    pub fn score(&self) -> usize {
        let winner_count = self.winning_numbers().len();

        if winner_count > 0 {
            2_usize.pow(winner_count as u32 - 1)
        } else {
            0
        }
    }

    pub fn winning_numbers(&self) -> Vec<usize> {
        self.numbers
            .iter()
            .filter_map(|&n| {
                if self.winners.contains(&n) {
                    Some(n)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl FromStr for ScratchCard {
    type Err = ParseScratchCardError;

    fn from_str(s: &str) -> Result<ScratchCard, ParseScratchCardError> {
        let card_re = Regex::new(r"Card\s+(\d+):([\d\s]+)\|([\d\s]+)").expect("Error parsing card regex");
        let spaces_re = Regex::new(r"\s+").expect("Error parsing whitespace regex");

        if let Some(caps) = card_re.captures(s) {
            return Ok(ScratchCard {
                id: caps.get(1).unwrap().as_str().parse().unwrap(),
                winners: spaces_re.split(caps.get(2).unwrap().as_str().trim()).map(|s| s.parse().unwrap()).collect(),
                numbers: spaces_re.split(caps.get(3).unwrap().as_str().trim()).map(|s| s.parse().unwrap()).collect(),
            });
        }
        Err(ParseScratchCardError)
    }
}

#[derive(Debug)]
pub struct ParseScratchCardError;

impl fmt::Display for ParseScratchCardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing ScratchCard")
    }
}

impl Error for ParseScratchCardError {}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn example_cards() -> Vec<ScratchCard> {
        let input = "\
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        input
            .lines()
            .map(|l| l.parse().expect("Error parsing example scratch cards"))
            .collect()
    }

    #[test]
    fn solution1_test() {
        let cards = example_cards();
        assert_eq!(cards.len(), 6);
        assert_eq!(cards[0].score(), 8);
        assert_eq!(cards[1].score(), 2);
        assert_eq!(cards[2].score(), 2);
        assert_eq!(cards[3].score(), 1);
        assert_eq!(cards[4].score(), 0);
        assert_eq!(cards[5].score(), 0);

        let total_score: usize = cards.iter().map(|c| c.score()).sum();
        assert_eq!(total_score, 13);
    }

    #[test]
    fn solution2_test() {
        let pile = CardPile::from(example_cards());
        assert_eq!(pile.cards.len(), example_cards().len());

        let winnings = pile.play();
        assert_eq!(winnings.len(), 30);
    }
}

