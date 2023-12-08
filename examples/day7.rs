use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BinaryHeap;

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
    "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2",
];

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
                for i in pos+1..13 {
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

        for i in 0..total{
            if let Some(game) = games.pop(){
                println!("Hand {}", game.hand);
                solution += game.points * (i+1) as u64;
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
    fn test_hands_cmp1() {
        let hand1 = Game{
            hand: "32T3K".to_string(), //one pair
            points:0,
        };
        let hand2 = Game{
            hand: "KK677".to_string(), //two pair
            points:0,
        };
        assert_eq!(hand1.cmp(&hand2), Ordering::Less);
    }

    #[test]
    fn test_hands_cmp2() {
        let hand1 = Game{
            hand: "45555".to_string(), //one pair
            points:0,
        };
        let hand2 = Game{
            hand: "444T4".to_string(), //two pair
            points:0,
        };
        assert_eq!(hand1.cmp(&hand2), Ordering::Greater);
    }


    // #[test]
    // fn test_hands_cmp2() {
    //     let hand1 = Game{
    //         hand: "32T3K".to_string(), //one pair
    //         points:0,
    //     };
    //     let hand2 = Game{
    //         hand: "KK677".to_string(), //two pair
    //         points:0,
    //     };
    //     assert_eq!(hand1.cmp(&hand2), Ordering::Greater);
    // }

    #[test]
    fn test_hand_result_pair() {
        let hand = Game{
            hand: "32T3K".to_string(), //one pair
            points:0,
        };
        assert_eq!(hand.game_result(), GameResult::Pair);
    }


    #[test]
    fn test_hand_result_five() {
        let hand = Game{
            hand: "KKKKK".to_string(), //one pair
            points:0,
        };
        assert_eq!(hand.game_result(), GameResult::Five);
    }

    #[test]
    fn test_hand_result_four() {
        let hand = Game{
            hand: "KKAKK".to_string(), //one pair
            points:0,
        };
        assert_eq!(hand.game_result(), GameResult::Four);
    }

    #[test]
    fn test_hand_result_three() {
        let hand = Game{
            hand: "K6AKK".to_string(), //one pair
            points:0,
        };
        assert_eq!(hand.game_result(), GameResult::Three);
    }

    #[test]
    fn test_hand_result_full1() {
        let hand = Game{
            hand: "KKKAA".to_string(), //one pair
            points:0,
        };
        assert_eq!(hand.game_result(), GameResult::Full);
    }

    #[test]
    fn test_hand_result_full2() {
        let hand = Game{
            hand: "KKAAA".to_string(), //one pair
            points:0,
        };
        assert_eq!(hand.game_result(), GameResult::Full);
    }

#[test]
    fn test_hand_result_full3() {
        let hand = Game{
            hand: "AKAKA".to_string(), //one pair
            points:0,
        };
        assert_eq!(hand.game_result(), GameResult::Full);
    }


    #[test]
    fn test_hand_result_two_pair1() {
        let hand = Game{
            hand: "KK6AA".to_string(), //one pair
            points:0,
        };
        assert_eq!(hand.game_result(), GameResult::TwoPair);
    }

    #[test]
    fn test_hand_result_two_pair2() {
        let hand = Game{
            hand: "25353".to_string(), //one pair
            points:0,
        };
        assert_eq!(hand.game_result(), GameResult::TwoPair);
    }

    #[test]
    fn test_hand_result_pair1() {
        let hand = Game{
            hand: "ATA45".to_string(), //one pair
            points:0,
        };
        assert_eq!(hand.game_result(), GameResult::Pair);
    }

    #[test]
    fn test_hand_result_pair2() {
        let hand = Game{
            hand: "85678".to_string(), //one pair
            points:0,
        };
        assert_eq!(hand.game_result(), GameResult::Pair);
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
