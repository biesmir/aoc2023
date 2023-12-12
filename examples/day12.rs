use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn is_correct(springs: &Vec<char>, groups: &Vec<u64>) -> bool {
    let mut on_series = false;
    let mut current_series = 0;
    let mut i: i64 = -1;
    for c in springs {
        if i >= groups.len().try_into().unwrap() {
            return false;
        }

        match c {
            '#' => {
                if !on_series {
                    i += 1;
                }
                on_series = true;
                current_series += 1;
            }
            '.' => {
                if on_series {
                    if groups[<i64 as TryInto<usize>>::try_into(i).unwrap()] != current_series {
                        return false;
                    }
                    on_series = false;
                    current_series = 0;
                }
            }
            _ => panic!(),
        }
    }

    if i < groups.len().try_into().unwrap() {
        if on_series {
            if groups[<i64 as TryInto<usize>>::try_into(i).unwrap()] != current_series {
                return false;
            }
        }
    }
    if i>=0{
    if <i64 as TryInto<usize>>::try_into(i).unwrap() == groups.len() - 1 {
        return true;
    }
    }

    return false;
}

fn get_possible(line: &str) -> u64 {
    let mut parsed = line.split_whitespace();
    let springs: Vec<char> = parsed.nth(0).unwrap().chars().collect();
    let numbers: Vec<u64> = parsed
        .nth(0)
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut qmarks: Vec<usize> = Vec::new();

    for (pos, c) in springs.iter().enumerate() {
        if *c == '?' {
            qmarks.push(pos);
        }
    }
    let qmarks = qmarks;
    let mut permutation = 0x00;

    let mut possibilities = 0;
    let mut springs_variant = springs.clone();
    while (permutation & (0x1 << qmarks.len())) == 0 {
        for (i, j) in qmarks.iter().enumerate() {
            if (permutation & (0x1 << i)) > 0 {
                springs_variant[*j] = '#'
            } else {
                springs_variant[*j] = '.'
            }
        }

        if is_correct(&springs_variant, &numbers) {
            possibilities += 1;
            for c in &springs_variant {
                print!("{}", *c);
            }

            println!(" ");
        } //  else {
          //     // print!("in");
          // }
          // println!("valid");
        permutation += 1;
    }
    println!("possibilities {}", possibilities);
    return possibilities;
}

fn part1(filename: &str) -> u64 {
    if let Ok(lines) = read_lines(filename) {
        return lines
            .map(|x| get_possible(&x.unwrap()))
            .collect::<Vec<u64>>()
            .into_iter()
            .sum();
    }
    panic!();
}

fn part2(filename: &str) -> u64 {
    todo!()
}

fn main() {
    println!("{}", part1("Inputs/Day12/input"));
    println!("{}", part2("Inputs/Day12/input"));
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part1_example1() {
    //     assert_eq!(part1("Inputs/Day12/example"), 21);
    // }
    // #[test]
    // fn test_part2_example1() {
    //     assert_eq!(part2("Inputs/Day11/example"), 8410);
    // }
    #[test]
    fn test_possibilities1() {
        let line = ".??..??...?##. 1,1,3";
        assert_eq!(get_possible(&line), 4);
    }

    #[test]
    fn test_possibilities2() {
        let line = "?###???????? 3,2,1";
        assert_eq!(get_possible(&line), 10);
    }

    #[test]
    fn test_valid_line1() {
        let springs = vec!['.', '.', '.', '.', '#'];
        let groups = vec![1];
        assert!(is_correct(&springs, &groups));
    }

    #[test]
    fn test_valid_line2() {
        let springs = vec!['.', '#', '#', '.', '#'];
        let groups = vec![2, 1];
        assert!(is_correct(&springs, &groups));
    }

    #[test]
    fn test_valid_line3() {
        let springs = vec!['.', '#', '#', '.', '#', '#', '#', '#', '.', '#'];
        let groups = vec![2, 4, 1];
        assert!(is_correct(&springs, &groups));
    }

    #[test]
    fn test_invalid_line1() {
        let springs = vec!['.', '#', '#', '.', '#', '.', '#', '#', '.', '#'];
        let groups = vec![2, 4, 1];
        assert!(!is_correct(&springs, &groups));
    }

    #[test]
    fn test_invalid_line2() {
        let springs = vec!['.', '#', '#', '.', '#', '#', '#', '#', '.', '#', '.', '#'];
        let groups = vec![2, 4, 1];
        assert!(!is_correct(&springs, &groups));
    }

    #[test]
    fn test_invalid_line3() {
        let springs = vec!['.', '#', '#', '.', '#', '.', '#', '#', '.', '#'];
        let groups = vec![2, 4, 1, 5];
        assert!(!is_correct(&springs, &groups));
    }

    #[test]
    fn test_invalid_line4() {
        let springs = vec!['.', '#', '#', '#', '.', '#', '#', '.', '#', '#', '#', '#'];
        let groups = vec![3, 2, 1];
        assert!(!is_correct(&springs, &groups));
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
