use std::cmp::min;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn check_reflection_column(map: &Vec<Vec<char>>, column: usize) -> bool {
    let limit = min(map[0].len() - column - 1, column + 1);

    for row in map {
        for i in 0..limit {
            if row[column + i + 1] != row[column - i] {
                return false;
            }
        }
    }

    return true;
}

fn check_reflection_row(map: &Vec<Vec<char>>, row: usize) -> bool {
    let limit = min(map.len() - row - 1, row + 1);

    for i in 0..map[0].len() {
        for j in 0..limit {
            if map[row + j + 1][i] != map[row - j][i] {
                return false;
            }
        }
    }

    return true;
}

fn get_score(map: Vec<Vec<char>>) -> usize {
    for i in 0..map[0].len() - 1 {
        if check_reflection_column(&map, i) {
            return i + 1;
        }
    }

    for i in 0..map.len() - 1 {
        if check_reflection_row(&map, i) {
            return (i + 1) * 100;
        }
    }

    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }

    panic!("no reflection found");
}

fn part1(filename: &str) -> usize {
    if let Ok(mut lines) = read_lines(filename) {
        let mut res = 0;
        'outer: loop {
            let mut map: Vec<Vec<char>> = Vec::new();
            loop {
                if let Some(line) = lines.next() {
                    let line = line.unwrap();
                    if line.len() > 0 {
                        map.push(line.chars().collect());
                    } else {
                        res += get_score(map);
                        break;
                    }
                } else {
                    res += get_score(map);
                    break 'outer;
                }
            }
        }
        return res;
    }
    panic!();
}

fn check_reflection_column_smudged(map: &Vec<Vec<char>>, column: usize) -> bool {
    let limit = min(map[0].len() - column - 1, column + 1);
    let mut incorrect_count = 0;

    for row in map {
        for i in 0..limit {
            if row[column + i + 1] != row[column - i] {
                incorrect_count += 1;
                if incorrect_count > 1 {
                    return false;
                }
            }
        }
    }

    if incorrect_count == 1 {
        return true;
    }
    return false;
}

fn check_reflection_row_smudged(map: &Vec<Vec<char>>, row: usize) -> bool {
    let limit = min(map.len() - row - 1, row + 1);
    let mut incorrect_count = 0;

    for i in 0..map[0].len() {
        for j in 0..limit {
            if map[row + j + 1][i] != map[row - j][i] {
                incorrect_count += 1;
                if incorrect_count > 1 {
                    return false;
                }
            }
        }
    }

    if incorrect_count == 1 {
        return true;
    }
    return false;
}

fn get_score_smudged(map: Vec<Vec<char>>) -> usize {
    for i in 0..map[0].len() - 1 {
        if check_reflection_column_smudged(&map, i) {
            return i + 1;
        }
    }

    for i in 0..map.len() - 1 {
        if check_reflection_row_smudged(&map, i) {
            return (i + 1) * 100;
        }
    }

    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }

    panic!("no reflection found");
}

fn part2(filename: &str) -> usize {
    if let Ok(mut lines) = read_lines(filename) {
        let mut res = 0;
        'outer: loop {
            let mut map: Vec<Vec<char>> = Vec::new();
            loop {
                if let Some(line) = lines.next() {
                    let line = line.unwrap();
                    if line.len() > 0 {
                        map.push(line.chars().collect());
                    } else {
                        res += get_score_smudged(map);
                        break;
                    }
                } else {
                    res += get_score_smudged(map);
                    break 'outer;
                }
            }
        }
        return res;
    }
    panic!();
}

fn main() {
    println!("{}", part1("Inputs/Day13/input"));
    println!("{}", part2("Inputs/Day13/input"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1("Inputs/Day13/example"), 405);
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(part2("Inputs/Day13/example"), 400);
    }

    #[test]
    fn test_part1_horizontal() {
        let mut map: Vec<Vec<char>> = Vec::new();
        map.push(vec!['#', '.', '#', '#', '.', '.', '#', '#', '.']);
        map.push(vec!['.', '.', '#', '.', '#', '#', '.', '#', '.']);
        assert_eq!(get_score(map), 5);
    }
    #[test]
    fn test_part1_vertical() {
        let mut map: Vec<Vec<char>> = Vec::new();
        map.push(vec!['#', '.', '.', '.', '.', '.', '.', '.', '#']);
        map.push(vec!['.', '.', '.', '.', '.', '.', '.', '.', '#']);
        map.push(vec!['.', '.', '.', '.', '.', '.', '.', '.', '#']);
        map.push(vec!['#', '.', '.', '.', '.', '.', '.', '.', '#']);
        assert_eq!(get_score(map), 200);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
