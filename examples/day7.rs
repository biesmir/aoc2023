use std::cmp::max;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Eq)]
struct Game {
    hand: String,
    points: u64,
}

#[derive(Eq)]
struct GameJokered {
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
    "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2",
];

static CARDS_LIST_JOKERED: [&str; 13] = [
    "A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2", "J",
];

fn better_than_jokered(first: char, second: char) -> Ordering {
    let mut value1: usize = 0;
    let mut value2: usize = 0;
    for i in 1..13 {
        if CARDS_LIST_JOKERED[i].as_bytes()[0] == first as u8 {
            value1 = i;
            break;
        }
    }
    for i in 1..13 {
        if CARDS_LIST_JOKERED[i].as_bytes()[0] == second as u8 {
            value2 = i;
            break;
        }
    }
    if value1 == value2 {
        return Ordering::Equal;
    }
    if value1 > value2 {
        return Ordering::Greater;
    }
    return Ordering::Less;
}

fn better_than(first: char, second: char) -> Ordering {
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
    if value1 == value2 {
        return Ordering::Equal;
    }
    if value1 > value2 {
        return Ordering::Greater;
    }
    return Ordering::Less;
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
                for i in pos + 1..13 {
                    let count2 = self
                        .hand
                        .matches(CARDS_LIST[i])
                        .collect::<Vec<&str>>()
                        .len();
                    if count2 == 3 {
                        return GameResult::Full;
                    }
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

impl Ord for Game {
    fn cmp(&self, another: &Game) -> Ordering {
        if self.game_result() > another.game_result() {
            return Ordering::Greater;
        }
        if self.game_result() < another.game_result() {
            return Ordering::Less;
        }
        for i in 0..5 {
            if self.hand.chars().nth(i).unwrap() == another.hand.chars().nth(i).unwrap() {
                continue;
            }
            return better_than(
                self.hand.chars().nth(i).unwrap(),
                another.hand.chars().nth(i).unwrap(),
            );
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
        return self.cmp(&other) == Ordering::Equal;
    }
}

impl Ord for GameJokered {
    fn cmp(&self, another: &GameJokered) -> Ordering {
        if self.game_result() > another.game_result() {
            return Ordering::Greater;
        }
        if self.game_result() < another.game_result() {
            return Ordering::Less;
        }
        for i in 0..5 {
            if self.hand.chars().nth(i).unwrap() == another.hand.chars().nth(i).unwrap() {
                continue;
            }
            return better_than_jokered(
                self.hand.chars().nth(i).unwrap(),
                another.hand.chars().nth(i).unwrap(),
            );
        }
        panic!()
    }
}

impl PartialOrd for GameJokered {
    fn partial_cmp(&self, another: &GameJokered) -> Option<Ordering> {
        return Some(self.cmp(&another));
    }
}

impl PartialEq for GameJokered {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(&other) == Ordering::Equal;
    }
}

impl GameJokered {
    fn game_result(&self) -> GameResult {
        let jokers_count = self.hand.matches("J").collect::<Vec<&str>>().len();
        if jokers_count >= 4 {
            return GameResult::Five;
        }

        let mut counts: Vec<usize> = Vec::new();
        for card in CARDS_LIST_JOKERED[0..12].into_iter() {
            let count = self.hand.matches(card).collect::<Vec<&str>>().len();
            counts.push(count);
        }
        counts.sort();
        let count1 = counts[counts.len() - 1];
        let count2 = counts[counts.len() - 2];

        if count1 == 5 {
            return GameResult::Five;
        }
        if count1 == 4 {
            if jokers_count == 1 {
                return GameResult::Five;
            } else {
                return GameResult::Four;
            }
        }

        if count1 == 3 {
            if jokers_count == 2 {
                return GameResult::Five;
            }
            if jokers_count == 1 {
                return GameResult::Four;
            }
            if count2 == 2 {
                return GameResult::Full;
            }
            if count2 == 1 {
                if jokers_count == 1 {
                    return GameResult::Full;
                }
            }
            return GameResult::Three;
        }

        if count1 == 2 {
            if jokers_count == 3 {
                return GameResult::Five;
            }
            if jokers_count == 2 {
                return GameResult::Four;
            }
            if jokers_count == 1 {
                if count2 == 2 {
                    return GameResult::Full;
                }
                return GameResult::Three;
            }
            if count2 == 2 {
                return GameResult::TwoPair;
            }
            return GameResult::Pair;
        }

        if count1 == 1 {
            if jokers_count == 4 {
                return GameResult::Five;
            }
            if jokers_count == 3 {
                return GameResult::Four;
            }
            if jokers_count == 2 {
                return GameResult::Three;
            }
            if jokers_count == 1 {
                return GameResult::Pair;
            }
        }

        return GameResult::High;
    }
}

fn part1(filename: &str) -> u64 {
    let mut games = BinaryHeap::new();
    let mut solution = 0;
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

        let total = games.len();

        for i in 0..total {
            if let Some(game) = games.pop() {
                println!("Hand {}", game.hand);
                solution += game.points * (i + 1) as u64;
            }
        }
        return solution;
    }

    2137
}

fn part2(filename: &str) -> u64 {
    let mut games = BinaryHeap::new();
    let mut solution = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                let words: Vec<&str> = line.split_whitespace().collect();
                let game = GameJokered {
                    hand: words[0].to_string(),
                    points: words[1].parse().unwrap(),
                };
                games.push(game);
            }
        }

        let total = games.len();

        for i in 0..total {
            if let Some(game) = games.pop() {
                println!("Hand {}", game.hand);
                solution += game.points * (i + 1) as u64;
            }
        }
        return solution;
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

    #[test]
    fn test_hand_result_pair() {
        let hand = Game {
            hand: "32T3K".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Pair);
    }

    #[test]
    fn test_hand_result_five() {
        let hand = Game {
            hand: "KKKKK".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Five);
    }

    #[test]
    fn test_hand_result_four() {
        let hand = Game {
            hand: "KKAKK".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Four);
    }

    #[test]
    fn test_hand_result_three() {
        let hand = Game {
            hand: "K6AKK".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Three);
    }

    #[test]
    fn test_hand_result_full1() {
        let hand = Game {
            hand: "KKKAA".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Full);
    }

    #[test]
    fn test_hand_result_full2() {
        let hand = Game {
            hand: "KKAAA".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Full);
    }

    #[test]
    fn test_hand_result_full3() {
        let hand = Game {
            hand: "AKAKA".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Full);
    }

    #[test]
    fn test_hand_result_two_pair1() {
        let hand = Game {
            hand: "KK6AA".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::TwoPair);
    }

    #[test]
    fn test_hand_result_two_pair2() {
        let hand = Game {
            hand: "25353".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::TwoPair);
    }

    #[test]
    fn test_hand_result_pair1() {
        let hand = Game {
            hand: "ATA45".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Pair);
    }

    #[test]
    fn test_hand_result_pair2() {
        let hand = Game {
            hand: "85678".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Pair);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2("Inputs/Day7/example"), 5905);
    }

    #[test]
    fn test_jokered_hand_result_pair() {
        let hand = GameJokered {
            hand: "32TJK".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Pair);
    }

    #[test]
    fn test_jokered_hand_result_five1() {
        let hand = GameJokered {
            hand: "KKJJJ".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Five);
    }

    #[test]
    fn test_jokered_hand_result_five2() {
        let hand = GameJokered {
            hand: "JTTJT".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Five);
    }
    #[test]
    fn test_jokered_hand_result_five3() {
        let hand = GameJokered {
            hand: "TTTJT".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Five);
    }

    #[test]
    fn test_jokered_hand_result_four() {
        let hand = GameJokered {
            hand: "KKAJK".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Four);
    }

    #[test]
    fn test_jokered_hand_result_three1() {
        let hand = GameJokered {
            hand: "K6AJK".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Three);
    }

    #[test]
    fn test_jokered_hand_result_three2() {
        let hand = GameJokered {
            hand: "22A29".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Three);
    }

    #[test]
    fn test_jokered_hand_result_full1() {
        let hand = GameJokered {
            hand: "KKJAA".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Full);
    }

    #[test]
    fn test_jokered_hand_result_full2() {
        let hand = GameJokered {
            hand: "KKAAA".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Full);
    }

    #[test]
    fn test_jokered_hand_result_full3() {
        let hand = GameJokered {
            hand: "AKAKJ".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Full);
    }

    #[test]
    fn test_jokered_hand_result_two_pair1() {
        let hand = GameJokered {
            hand: "KK6AA".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::TwoPair);
    }

    #[test]
    fn test_jokered_hand_result_two_pair2() {
        let hand = GameJokered {
            hand: "25253".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::TwoPair);
    }

    #[test]
    fn test_jokered_hand_result_pair1() {
        let hand = GameJokered {
            hand: "ATA45".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Pair);
    }

    #[test]
    fn test_jokered_hand_result_pair2() {
        let hand = GameJokered {
            hand: "8567J".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::Pair);
    }

    #[test]
    fn test_jokered_hand_result_high() {
        let hand = GameJokered {
            hand: "2673Q".to_string(),
            points: 0,
        };
        assert_eq!(hand.game_result(), GameResult::High);
    }
}

fn main() {
    println!("{}", part1("Inputs/Day7/input"));
    println!("{}", part2("Inputs/Day7/input"));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
