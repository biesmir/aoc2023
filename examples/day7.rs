use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Eq)]
struct Game {
    hand: String,
    points: u64,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum GameResult {
    Five,
    Four,
    Full,
    Three,
    TwoPair,
    Pair,
    High,
}

static CARDS_LIST: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "A", "K", "Q", "J", "T",
];

fn better_than(first: char, second: char) -> bool {
    let mut value1: usize = 0;
    let mut value2: usize = 0;
    for i in 1..13 {
        if CARDS_LIST[i].as_bytes()[0] == first as u8 {
            value1 = i;
            break;
        }
    }
    for i in 1..13 {
        if CARDS_LIST[i].as_bytes()[0] == second as u8 {
            value2 = i;
            break;
        }
    }
    return value1 > value2;
}

impl Ord for Game {
    fn cmp(&self, another: &Game) -> Ordering {
        if self.game_result() > another.game_result() {
            return true;
        }
        if self.game_result() < another.game_result() {
            return false;
        }
        for i in 0..5 {
            if self.hand.chars().nth(i).unwrap() == another.hand.chars().nth(i).unwrap() {
                continue;
            }
            if better_than(
                self.hand.chars().nth(i).unwrap(),
                another.hand.chars().nth(i).unwrap(),
            ) {
                return true;
            } else {
                return false;
            }
        }
        panic!()
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, another: &Game) -> Option<Ordering> {
        return Some(self.cmp(&another));
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        if self.game_result() == other.game_result() {
            return true;
        }
        return false;
    }
}

impl Game {
    fn game_result(&self) -> GameResult {
        for (pos, card) in CARDS_LIST.into_iter().enumerate() {
            let count1 = self.hand.matches(card).collect::<Vec<&str>>().len();
            if count1 == 5 {
                return GameResult::Five;
            }
            if count1 == 4 {
                return GameResult::Four;
            }
            if count1 == 3 {
                for i in pos..13 {
                    let count2 = self
                        .hand
                        .matches(CARDS_LIST[i])
                        .collect::<Vec<&str>>()
                        .len();
                    if count2 == 2 {
                        return GameResult::Full;
                    }
                }
                return GameResult::Three;
            }
            if count1 == 2 {
                for i in pos..13 {
                    let count2 = self
                        .hand
                        .matches(CARDS_LIST[i])
                        .collect::<Vec<&str>>()
                        .len();
                    if count2 == 2 {
                        return GameResult::TwoPair;
                    }
                }
                return GameResult::Pair;
            }
        }

        return GameResult::High;
    }
}

fn part1(filename: &str) -> u64 {
    let mut games: Vec<Game> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                let words: Vec<&str> = line.split_whitespace().collect();
                let game = Game {
                    hand: words[0].to_string(),
                    points: words[1].parse().unwrap(),
                };
                games.push(game);
            }
        }
    }

    2137
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(part1("Inputs/Day7/example"), 6440);
    }
    // #[test]
    // fn test_example_part2() {
    //     assert_eq!(part2("Inputs/Day7/example"), 30);
    // }
}

fn main() {
    println!("{}", part1("Inputs/Day7/input"));
    // println!("{}", part2("Inputs/Day7/input"));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
