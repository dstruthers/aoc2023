use regex::Regex;

pub struct Schematic {
    data: String,
}

impl Schematic {
    pub fn from(data: String) -> Self {
        Self { data } 
    }

    pub fn char_at(&self, x: usize, y: usize) -> Option<char> {
        self.data.lines().nth(y)?.chars().nth(x)
    }

    pub fn part_numbers(&self) -> Vec<usize> {
        let mut numbers = Vec::new();
        let digit_re = Regex::new(r"\d+").unwrap();
        let symbol_re = Regex::new(r"[^0-9\.]").unwrap();

        for (match_y, line) in self.data.lines().enumerate() {
            'matches_iter: for m in digit_re.find_iter(&line) {
                let match_x = m.start();
                let match_str = m.as_str();
                let start_x = if match_x > 0 { match_x - 1 } else { 0 };
                let start_y = if match_y > 0 { match_y - 1 } else { 0 };
                let end_x = match_x + match_str.len();
                let end_y = match_y + 1;

                for search_x in start_x..=end_x {
                    for search_y in start_y..=end_y {
                        if search_y == match_y && search_x >= match_x && search_x < match_x + match_str.len() {
                            continue;
                        }
                        if let Some(search_char) = self.char_at(search_x, search_y) {
                            if symbol_re.is_match(String::from(search_char).as_ref()) {
                                if let Ok(number) = match_str.parse() {
                                    numbers.push(number);
                                    continue 'matches_iter;
                                } else {
                                    panic!("Error parsing number: {match_str}");
                                }
                            }
                        }
                    }
                }
            }
        }
        numbers
    }

    pub fn gears(&self) -> Vec<(usize, usize)> {
        let mut gs = Vec::new();
        let asterisk_re = Regex::new(r"\*").unwrap();

        for (y, line) in self.data.lines().enumerate() {
            for m in asterisk_re.find_iter(&line) {
                let x = m.start();
                let ns = self.neighbors(x, y);
                if ns.len() == 2 {
                    gs.push((ns[0], ns[1]));
                }
            }
        }
        gs
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<usize> {
        let mut ns = Vec::new();
        let digit_re = Regex::new(r"\d+").unwrap();
        let search_line_range = if y > 0 { y - 1..=y + 1 } else { 0..=1 };

        for search_y in search_line_range {
            if let Some(line) = self.data.lines().nth(search_y) {
                for m in digit_re.find_iter(&line) {
                    if (m.end() >= x && m.end() <= x + 1) || (m.start() >= x - 1 && m.start() <= x + 1) {
                        let neighbor = m.as_str().parse().expect("Error parsing neighbor");
                        ns.push(neighbor);
                    }
                }
            }
        }
        ns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_schematic() -> Schematic {
        Schematic::from(
            String::from("\
                467..114..\n\
                ...*......\n\
                ..35..633.\n\
                ......#...\n\
                617*......\n\
                .....+.58.\n\
                ..592.....\n\
                ......755.\n\
                ...$.*....\n\
                .664.598.."))
    }

    #[test]
    fn solution1_test() {
        let schematic = example_schematic();
        let part_numbers = schematic.part_numbers();

        assert_eq!(part_numbers, vec![467, 35, 633, 617, 592, 755, 664, 598]);
        assert_eq!(part_numbers.iter().sum::<usize>(), 4361);
    }

    #[test]
    fn solution2_test() {
        let schematic = example_schematic();
        let gears = schematic.gears();

        assert_eq!(gears.iter().map(|(x, y)| x * y).sum::<usize>(), 467835);
    }
}
