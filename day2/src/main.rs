use std::{
    collections::HashMap,
    error::Error,
    fmt,
    fs,
    str::FromStr,
};

const INPUT_FILE: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(INPUT_FILE)?;
    let mut games: Vec<Game> = Vec::new();

    for line in input.lines() {
        let game: Game = line.parse()?;
        games.push(line.parse()?);
    }

    let requirements = vec![(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)];
    let solution1: usize = games
        .iter()
        .filter(|g| g.is_possible(&requirements))
        .map(|g| g.id)
        .sum();
    println!("Solution 1: {solution1}");
    assert_eq!(solution1, 2176);

    let solution2: usize = games
        .iter()
        .map(|g| g.minimum_cubes())
        .map(|mins| mins.into_values().product::<usize>())
        .sum();

    println!("Solution 2: {solution2}");
    assert_eq!(solution2, 63700);
    Ok(())
}

#[derive(Debug)]
struct Game {
    id: usize,
    reveals: Vec<Vec<(Color, usize)>>,
}

impl Game {
    fn is_possible(&self, requirements: &Vec<(Color, usize)>) -> bool {
        for (req_color, req_count) in requirements {
            for reveals in &self.reveals {
                for (rev_color, rev_count) in reveals {
                    if rev_color == req_color && rev_count > req_count {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn minimum_cubes(&self) -> HashMap<Color, usize> {
        let mut minimums: HashMap<Color, usize> = HashMap::new();

        for reveals in &self.reveals {
            for (rev_color, rev_count) in reveals {
                if let Some(ref min) = minimums.get(&rev_color) {
                    if rev_count > min {
                        minimums.insert(*rev_color, *rev_count);
                    }
                } else {
                    minimums.insert(*rev_color, *rev_count);
                }
            }
        }
        minimums
    }
}

#[derive(Debug)]
struct ParseGameError {
    input: String,
}

impl Error for ParseGameError {}

impl fmt::Display for ParseGameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to parse Game from input: {}", self.input)
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((tag, data)) = s.split_once(':') else {
            return Err(ParseGameError { input: s.to_owned() });
        };
        let Some((_, game_id)) = tag.split_once(' ') else {
            return Err(ParseGameError { input: s.to_owned() });
        };
        let Ok(game_id) = game_id.parse() else {
            return Err(ParseGameError { input: s.to_owned() });
        };

        let reveal_data = data.split(';');
        let mut reveals = Vec::new();

        for reveal in reveal_data {
            let mut rev = Vec::new();

            for reveal_color in reveal.trim().split(',') {
                let Some((count, color)) = reveal_color.trim().split_once(' ') else {
                    return Err(ParseGameError { input: s.to_owned() });
                };
                let Ok(count) = count.parse() else {
                    return Err(ParseGameError { input: s.to_owned() });
                };
                let Ok(color) = color.parse() else {
                    return Err(ParseGameError { input: s.to_owned() });
                };
                rev.push((color, count));
            }
            reveals.push(rev);
        }
        Ok(Game { id: game_id, reveals })
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct ParseColorError {
    input: String,
}

impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to parse Color from input: {}", self.input)
    }
}

impl Error for ParseColorError { }

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match String::from(s).to_lowercase().as_str() {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            other => Err(ParseColorError { input: other.to_owned() }),
        }
    }
}
