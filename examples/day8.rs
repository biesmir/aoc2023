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

fn calculate_cycle_len(
    start_node: u64,
    moves: &Vec<char>,
    map: &HashMap<u64, (u64, u64, u64)>,
) -> (u64, u64) {
    let mut res = 1;
    let mut i = 0;
    let mut node = start_node;
    let mut z_pos = 0;

    let mut visited: Vec<(usize, u64)> = Vec::new();

    println!("start node {}", start_node);
    loop {
        // let mv = moves.chars().nth(i).unwrap();
        // println!("current {}", currentz);
        match moves[i] {
            'L' => {
                node = map[&node].1;
            }
            'R' => {
                node = map[&node].2;
            }
            _ => panic!(),
        }
        // println!("node {}", node);

        if (node & 0xFF) == 'Z'.into() {
            z_pos = res;
        } //  else{
          //     println!("{:X} this doesnt end with {:X}", node, 'Z' as u8);
          // }

        if visited.contains(&(i, node)) {
            break;
        } else {
            visited.push((i, node));
        }
        res += 1;
        i = (i + 1) % moves.len();

    }
    // assert_ne!(z_pos, 0);
    let index = visited.iter().position(|&x| x == (i, node));
    println!("z_pos {}", z_pos);
    return (z_pos, (visited.len()-index.unwrap()).try_into().unwrap());
}

fn part2(filename: &str) -> u64 {
    let mut solution = 0;
    let mut map: HashMap<u64, (u64, u64, u64)> = HashMap::new();
    // let mut map: Vec<(u64, u64, u64)> = vec![(0, 0, 0); 1_000_000_000];
    let mut entry_nodes: Vec<u64> = Vec::new();
    if let Ok(mut lines) = read_lines(filename) {
        let moves = lines.next().unwrap().unwrap();
        println!("moves {}", moves);
        lines.next();
        loop {
            let line = lines.next().unwrap().unwrap();
            let words: Vec<&str> = line.split_whitespace().collect();
            let letter1: u64 = (*(words[0].as_bytes().get(0).unwrap())).into();
            let letter2: u64 = (*(words[0].as_bytes().get(1).unwrap())).into();
            let letter3: u64 = (*(words[0].as_bytes().get(2).unwrap())).into();
            let entry: u64 = (letter1 << 16) + (letter2 << 8) + letter3;
            print!("{} {} ", words[0], entry);

            let letter1: u64 = (*(words[2].as_bytes().get(1).unwrap())).into();
            let letter2: u64 = (*(words[2].as_bytes().get(2).unwrap())).into();
            let letter3: u64 = (*(words[2].as_bytes().get(3).unwrap())).into();
            let left: u64 = (letter1 << 16) + (letter2 << 8) + letter3;
            print!("{} {} ", words[2], left);

            let letter1: u64 = (*(words[3].as_bytes().get(0).unwrap())).into();
            let letter2: u64 = (*(words[3].as_bytes().get(1).unwrap())).into();
            let letter3: u64 = (*(words[3].as_bytes().get(2).unwrap())).into();
            let right: u64 = (letter1 << 16) + (letter2 << 8) + letter3;
            println!("{} {}", words[3], right);

            map.insert(entry, (entry, left, right));

            if (entry & 0xFF) != b"A"[0].into() {
                println!("number of entry nodes {}", entry_nodes.len());
                break;
            }

            entry_nodes.push(entry);
        }

        for line in lines {
            if let Ok(line) = line {
                let words: Vec<&str> = line.split_whitespace().collect();
                let letter1: u64 = (*(words[0].as_bytes().get(0).unwrap())).into();
                let letter2: u64 = (*(words[0].as_bytes().get(1).unwrap())).into();
                let letter3: u64 = (*(words[0].as_bytes().get(2).unwrap())).into();
                let entry: u64 = (letter1 << 16) + (letter2 << 8) + letter3;
                print!("{} {} ", words[0], entry);

                let letter1: u64 = (*(words[2].as_bytes().get(1).unwrap())).into();
                let letter2: u64 = (*(words[2].as_bytes().get(2).unwrap())).into();
                let letter3: u64 = (*(words[2].as_bytes().get(3).unwrap())).into();
                let left: u64 = (letter1 << 16) + (letter2 << 8) + letter3;
                print!("{} {} ", words[2], left);

                let letter1: u64 = (*(words[3].as_bytes().get(0).unwrap())).into();
                let letter2: u64 = (*(words[3].as_bytes().get(1).unwrap())).into();
                let letter3: u64 = (*(words[3].as_bytes().get(2).unwrap())).into();
                let right: u64 = (letter1 << 16) + (letter2 << 8) + letter3;
                println!("{} {}", words[3], right);

                map.insert(entry, (entry, left, right));
            }
        }

        let moves: Vec<char> = moves.chars().collect();
        let mut cycles: Vec<(u64, u64)> = Vec::new();
        for node in entry_nodes {
            cycles.push(calculate_cycle_len(node, &moves, &map));
        }

        let mut ends: Vec<u64> = Vec::new();

        for cycle in &cycles {
            println!("cycle {} {}", cycle.0, cycle.1);
            ends.push(cycle.0 + cycle.1);
        }

        loop {
            if ends.iter().all(|&x| x == ends[0]) {
                return ends[0];
            }

            let mut min_element = std::u64::MAX;
            let mut min_index = None;
            for (index, &element) in ends.iter().enumerate() {
                if element < min_element {
                    min_element = element;
                    min_index = Some(index);
                }
            }
            ends[min_index.unwrap()] += cycles[min_index.unwrap()].1;
        }

        // let mut i: usize = 0;
        // let moves: Vec<char> = moves.chars().collect();
        // 'outer: loop {
        //     // let mv = moves.chars().nth(i).unwrap();
        //     // println!("current {}", currentz);
        //     match moves[i] {
        //         'L' => {
        //             for current in &mut current_nodes {
        //                 *current = map[current].1;
        //             }
        //         }
        //         'R' => {
        //             for current in &mut current_nodes {
        //                 *current = map[current].2;
        //             }
        //         }
        //         _ => panic!(),
        //     }

        //     solution += 1;
        //     i = (i + 1) % moves.len();

        //     // if solution % 100000000 == 0 {
        //     //     println!("so far{}", solution);
        //     // }

        //     for current in &current_nodes {
        //         if (current & 0xFF) != 'Z'.into() {
        //             continue 'outer;
        //         }
        //     }
        //     for current in &current_nodes {
        //         println!("{} ends with Z", current);
        //     }

        //     return solution;
        // }
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
