use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(filename: &str) -> u64 {
    let mut solution = 1;
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    if let Ok(mut lines) = read_lines(filename) {
        let moves = lines.next().unwrap().unwrap();
        println!("moves {}", moves);
        lines.next();

        let line = lines.next().unwrap().unwrap();
        let words: Vec<&str> = line.split_whitespace().collect();
        let entry = words[0].to_string();
        let left = words[2][1..4].to_string();
        let right = words[3][..3].to_string();
        println!("{}", entry);
        map.insert(entry, (left.clone(), right.clone()));

        let mut current = (left, right);

        for line in lines {
            if let Ok(line) = line {
                let words: Vec<&str> = line.split_whitespace().collect();
                let entry = words[0].to_string();
                let left = words[2][1..4].to_string();
                let right = words[3][..3].to_string();
                println!("{}", entry);
                map.insert(entry, (left, right));
            }
        }

        assert_eq!("ZZZ", "ZZZ".to_string());

        let mut i: usize = 0;

        loop {
            let mv = moves.chars().nth(i).unwrap();
            // println!("current {} {}", current.0, current.1);
            if mv == 'L' {
                // println!("current0 {}", current.0);
                if current.0 == "ZZZ" {
                    return solution;
                }
                current = map.get(&current.0).unwrap().clone();
            } else if mv == 'R' {
                // println!("current1 {}", current.1);
                if current.1 == "ZZZ" {
                    return solution;
                }
                current = map.get(&current.1).unwrap().clone();
            } else {
                panic!()
            }
            // println!("sol {}", solution);
            solution += 1;
            i = (i + 1) % moves.len();
        }
    }

    2137
}

fn part2(filename: &str) -> u64 {
    let mut solution = 0;
    let mut map: HashMap<String, (String, String,String)> = HashMap::new();
    let mut current_nodes:Vec<(String,String, String)> = Vec::new();
    if let Ok(mut lines) = read_lines(filename) {
        let moves = lines.next().unwrap().unwrap();
        println!("moves {}", moves);
        lines.next();
        loop {
            let line = lines.next().unwrap().unwrap();
            let words: Vec<&str> = line.split_whitespace().collect();
            let entry = words[0].to_string();
            let left = words[2][1..4].to_string();
            let right = words[3][..3].to_string();
            println!("{}", entry);
            map.insert(entry.clone(), (entry.clone(), left.clone(), right.clone()));

            if entry.chars().nth(2).unwrap() != 'A' {
                println!("entry nodes {}", current_nodes.len());
                break;
            }

            current_nodes.push((entry.to_string(), left.to_string(), right.to_string()));

        }

        for line in lines {
            if let Ok(line) = line {
                let words: Vec<&str> = line.split_whitespace().collect();
                let entry = words[0].to_string();
                let left = words[2][1..4].to_string();
                let right = words[3][..3].to_string();
                println!("{}", entry);
                map.insert(entry.clone(), (entry.clone(), left, right));
            }
        }

        let mut i: usize = 0;

        'outer: loop {
            let mv = moves.chars().nth(i).unwrap();
            // println!("current {} {}", current.0, current.1);
            if mv == 'L' {
                for current in & mut current_nodes{
                    *current = map.get(&current.1).unwrap().clone();
                }

            } else if mv == 'R' {
                for current in &mut current_nodes{
                    *current = map.get(&current.2).unwrap().clone();
                }
            } else {
                panic!()
            }
            // println!("sol {}", solution);

            solution += 1;
            i = (i + 1) % moves.len();

            if solution%100000 == 0{
                    println!("so far{}", solution);

            }

            for current in &current_nodes{
                if current.0.chars().nth(2).unwrap() != 'Z'{
                    continue 'outer;
                }
                else {
                    // println!("pos {}", pos);
                }
            }
            for current in &current_nodes{
                    println!("{} ends with Z", current.0);
            }

            return solution;
        }
    }

    2137
}

fn main() {
    // println!("{}", part1("Inputs/Day8/input"));
    println!("{}", part2("Inputs/Day8/input"));
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
    #[test]
    fn test_part2_example1() {
        assert_eq!(part2("Inputs/Day8/example1"), 2);
    }
    #[test]
    fn test_part2_example2() {
        assert_eq!(part2("Inputs/Day8/example2"), 6);
    }

    #[test]
    fn test_part2_example3() {
        assert_eq!(part2("Inputs/Day8/example3"), 6);
    }
    #[test]
    fn test_part2_my_map() {
        assert_eq!(part2("Inputs/Day8/example3"), 6);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
