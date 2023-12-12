use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum PermutationCorrectness {
    Correct,
    Incorrect,
    Possible,
}

fn is_correct(springs: &Vec<char>, groups: &Vec<u64>) -> bool {
    return get_broken_spring(&springs) == *groups;
}

fn check_permutation(
    springs: &Vec<char>,
    groups: &Vec<u64>,
    qmarks: &Vec<usize>,
) -> PermutationCorrectness {
    let (so_far, leftover) = get_broken_springs_so_far(&springs);
    if so_far == *groups {
        return PermutationCorrectness::Correct;
    }
    for (idx, count) in so_far.iter().enumerate() {
        if idx >= groups.len() {
            return PermutationCorrectness::Incorrect;
        }
        if count > &groups[idx] {
            return PermutationCorrectness::Incorrect;
        }
    }

    let count_so_far: usize = so_far.iter().sum::<u64>().try_into().unwrap();
    let qmarks_left: usize = qmarks.iter().len();
    let goal_count: usize = groups.iter().sum::<u64>().try_into().unwrap();
    if count_so_far + leftover + qmarks_left < goal_count {
        // for c in springs{
        //     print!("{}", c);
        // }
        // println!("");
        // println!("{} + {} + {} < {}", count_so_far, qmarks_left, leftover, goal_count);
        return PermutationCorrectness::Incorrect;
    }

    if count_so_far + leftover > goal_count {
        // for c in springs{
        //     print!("{}", c);
        // }
        // println!("");
        // println!("{} + {} + {} > {}", count_so_far, qmarks_left, leftover, goal_count);
        return PermutationCorrectness::Incorrect;
    }


    return PermutationCorrectness::Possible;
}

fn get_broken_springs_so_far(springs: &Vec<char>) -> (Vec<u64>, usize) {
    let mut nums: Vec<u64> = Vec::new();
    let mut on_series = false;
    let mut current_series = 0;
    let mut i: i64 = -1;

    for j in 0..springs.len() {
        let c = springs[j];
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
                    nums.push(current_series);
                    on_series = false;
                    current_series = 0;
                }
            }
            _ => {
                let mut leftover = 0;
                    if on_series {
                        nums.push(current_series);
                    }

                for k in j+1..springs.len() {
                    if springs[k] == '#' {
                        leftover += 1;
                    }
                }
                return (nums, leftover);
            }
        }
    }
    if on_series {
        nums.push(current_series);
    }

    return (nums, 0);
}

fn get_broken_spring(springs: &Vec<char>) -> Vec<u64> {
    let mut nums: Vec<u64> = Vec::new();
    let mut on_series = false;
    let mut current_series = 0;
    let mut i: i64 = -1;
    for c in springs {
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
                    nums.push(current_series);
                    on_series = false;
                    current_series = 0;
                }
            }
            _ => panic!(),
        }
    }
    if on_series {
        nums.push(current_series);
    }

    return nums;
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

fn recursive_search(springs: &Vec<char>, groups: &Vec<u64>, qmarks: &Vec<usize>) -> u64 {
    match check_permutation(&springs, &groups, &qmarks) {
        PermutationCorrectness::Incorrect => {
            return 0;
        }
        PermutationCorrectness::Correct => {
            return 1;
        }
        PermutationCorrectness::Possible => {
            if qmarks.len() == 0 {
                return 0;
            }
            let mut springs1 = springs.clone();
            let mut springs2 = springs.clone();
            let mut qmarks_list: Vec<usize> = qmarks.clone();
            let idx = qmarks_list.pop().unwrap();
            springs1[idx] = '#';
            springs2[idx] = '.';
            return recursive_search(&springs1, &groups, &qmarks_list)
                + recursive_search(&springs2, &groups, &qmarks_list);
        }
    }
}

fn get_possible2(line: &str) -> u64 {
    let mut parsed = line.split_whitespace();
    let springs: Vec<char> = parsed.nth(0).unwrap().chars().collect();
    let numbers: Vec<u64> = parsed
        .nth(0)
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    // if springs[0] == '.' || springs[springs.len()-1] == '.'{
    //     let tmp = get_possible(&line);
    //     return tmp*tmp*tmp*tmp;
    // }

    let mut unfloded_springs: Vec<char> = springs.clone();
    let mut unfloded_numbers: Vec<u64> = numbers.clone();

    for _ in 0..4 {
        unfloded_numbers.append(&mut numbers.clone());
        unfloded_springs.push('?');
        unfloded_springs.append(&mut springs.clone());
    }
    let springs = unfloded_springs;
    let numbers = unfloded_numbers;

    for c in &springs {
        print!("{}", *c);
    }
    println!("");

    let mut qmarks: Vec<usize> = Vec::new();

    for (pos, c) in springs.iter().enumerate() {
        if *c == '?' {
            qmarks.push(pos);
        }
    }
    qmarks.reverse();
    let qmarks = qmarks;

    let res = recursive_search(&springs, &numbers, &qmarks);
    println!("result {}", res);
    return res;
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
    if let Ok(lines) = read_lines(filename) {
        return lines
            .map(|x| get_possible2(&x.unwrap()))
            .collect::<Vec<u64>>()
            .into_iter()
            .sum();
    }
    panic!();
}

fn main() {
    // println!("{}", part1("Inputs/Day12/input"));
    println!("{}", part2("Inputs/Day12/input"));
    // println!("{}", part2("Inputs/Day12/example"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1("Inputs/Day12/example"), 21);
    }
    #[test]
    fn test_part2_example1() {
        assert_eq!(part2("Inputs/Day12/example"), 525152);
    }
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

    #[test]
    fn test_possibilities_recursive1() {
        let springs = vec!['.', '#', '#', '#', '.', '#', '#', '.', '#'];
        let groups = vec![3, 2, 1];
        let qmarks: Vec<usize> = Vec::new();
        assert_eq!(recursive_search(&springs, &groups, &qmarks), 1);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
