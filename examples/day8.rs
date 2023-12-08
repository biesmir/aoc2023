use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(filename: &str) -> u64 {
    let mut solution = 0;
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    if let Ok(mut lines) = read_lines(filename) {
        let moves = lines.next().unwrap().unwrap();
        println!("{}", moves);
        lines.next();

        let words: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
                let entry = words[0].to_string();
                let left = words[2][1..].to_string();
                let right = words[3][..3].to_string();
                println!("{}", entry);
                map.insert(entry, (left, right));

        for line in lines {
            if let Ok(line) = line {
                let words: Vec<&str> = line.split_whitespace().collect();
                let entry = words[0].to_string();
                let left = words[2][1..].to_string();
                let right = words[3][..3].to_string();
                println!("{}", entry);
                map.insert(entry, (left, right));
            }
        }

        let mut current = ("PTN".to_string(), "MPT".to_string());

        for mv in moves.chars(){
            if current.0 == "ZZZ"{
                return solution;
            }
            if mv == 'L'{
                current = map.get(&current.0).unwrap().clone();
            } else{
                current = map.get(&current.1).unwrap().clone();
            }
            println!("sol {}", solution);
            solution += 1;

        }

    }

    2137
}

fn main() {
    println!("{}", part1("Inputs/Day8/input"));
    //println!("{}", part2("Inputs/Day7/input"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1("Inputs/Day8/example1"), 2);
    }
    #[test]
    fn test_part1_example2() {
        assert_eq!(part1("Inputs/Day8/example2"), 6);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
