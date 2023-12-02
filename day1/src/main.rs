use regex::Regex;
use std::{
    collections::HashMap,
    fs
};

const INPUT_FILE: &str = "input.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(INPUT_FILE)?;
    let solution1 = part1(&input)?;
    println!("First solution = {solution1}");

    let solution2 = part2(&input)?;
    println!("Second solution: {solution2}");
    Ok(())
}

fn part1(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let digit_re = Regex::new(r"(\d)")?;
    let mut total = 0;

    for line in input.lines() {
        let digits: Vec<usize> = digit_re
            .find_iter(&line)
            .map(|m| m.as_str().parse().unwrap())
            .collect();

        let first = digits[0];
        let last = digits[digits.len() - 1];

        total += first * 10 + last;
    }
    Ok(total)
}

fn part2(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut total = 0;
    let digit_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let digit_re = Regex::new(r"([0-9]|(zero|one|two|three|four|five|six|seven|eight|nine))")?;
    let begins_with_digit = Regex::new(r"^([0-9]|(zero|one|two|three|four|five|six|seven|eight|nine))")?;

    for line in input.lines() {
        let mut digits: Vec<usize> = Vec::new();

        for i in 0..line.len() {
            if let Some(m) = begins_with_digit.find(&line[i..line.len()]) {
                let digit = m.as_str();
                if digit.len() == 1 {
                    digits.push(digit.parse()?);
                } else {
                    digits.push(*digit_map.get(digit).unwrap());
                }
            }
        }

        let first = &digits[0];
        let last = &digits[digits.len() - 1];
        total += first * 10 + last;
    }
    Ok(total)
}
