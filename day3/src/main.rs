use std::fs;

use day3::Schematic;

const INPUT_FILE: &str = "input.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schematic = Schematic::from(fs::read_to_string(INPUT_FILE)?);
    let solution1: usize = schematic.part_numbers().iter().sum();
    println!("Solution 1: {solution1}");
    assert_eq!(solution1, 521515);

    let solution2: usize = schematic.gears().iter().map(|(x, y)| x * y).sum();
    println!("Solution 2: {solution2}");
    assert_eq!(solution2, 69527306);
    Ok(())
}


