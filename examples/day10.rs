use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(filename: &str) -> u64 {
    let mut solution = 1;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start_pos = (0, 0);
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let mut line_vec: Vec<char> = Vec::new();
            for c in line.unwrap().chars() {
                line_vec.push(c);
            }
            map.push(line_vec);
        }

        'outer: for i in 0..map.len() {
            for j in 0..map[0].len() {
                if map[i][j] == 'S' {
                    start_pos = (i, j);
                    break 'outer;
                }
            }
        }
        let start_pos = start_pos;
        // let mut i = 1;
        let mut pos = start_pos;
        println!("pos {} {}", pos.0 + 1, pos.1);

        if start_pos.0 > 0 {
            if map[start_pos.0 - 1][start_pos.1] == '|'
                || map[start_pos.0 - 1][start_pos.1] == '7'
                || map[start_pos.0 - 1][start_pos.1] == 'F'
            {
                pos = (start_pos.0 - 1, start_pos.1);
            }
        }
        if start_pos.1 > 0 {
            if map[start_pos.0][start_pos.1 - 1] == 'L'
                || map[start_pos.0][start_pos.1 - 1] == 'F'
                || map[start_pos.0][start_pos.1 - 1] == '-'
            {
                pos = (start_pos.0, start_pos.1 - 1);
            }
        }
        if start_pos.1 + 1 < map[0].len() {
            if map[start_pos.0][start_pos.1 + 1] == 'J'
                || map[start_pos.0][start_pos.1 + 1] == '7'
                || map[start_pos.0][start_pos.1 + 1] == '-'
            {
                pos = (start_pos.0, start_pos.1 + 1);
            }
        }
        if start_pos.0 + 1 < map.len() {
            if map[start_pos.0 + 1][start_pos.1] == '|'
                || map[start_pos.0 + 1][start_pos.1] == 'L'
                || map[start_pos.0 + 1][start_pos.1] == 'J'
            {
                pos = (start_pos.0, start_pos.1 + 1);
            }
        }
        let mut prev = start_pos;

        assert_ne!(pos, start_pos);

        loop {
            println!("pos {} {}", pos.0 + 1, pos.1);
            if pos == start_pos {
                break;
            }
            match map[pos.0][pos.1] {
                '|' => {
                    if prev == (pos.0 - 1, pos.1) {
                        prev = pos;
                        pos = (pos.0 + 1, pos.1)
                    } else if prev == (pos.0 + 1, pos.1) {
                        prev = pos;
                        pos = (pos.0 - 1, pos.1)
                    } else {
                        panic!()
                    }
                }
                '-' => {
                    if prev == (pos.0, pos.1 - 1) {
                        prev = pos;
                        pos = (pos.0, pos.1 + 1)
                    } else if prev == (pos.0, pos.1 + 1) {
                        prev = pos;
                        pos = (pos.0, pos.1 - 1)
                    } else {
                        panic!()
                    }
                }
                '7' => {
                    if prev == (pos.0, pos.1 - 1) {
                        prev = pos;
                        pos = (pos.0 + 1, pos.1)
                    } else if prev == (pos.0 + 1, pos.1) {
                        prev = pos;
                        pos = (pos.0, pos.1-1)
                    } else {
                        panic!()
                    }
                }
                'F' => {
                    if prev == (pos.0, pos.1 + 1) {
                        prev = pos;
                        pos = (pos.0 + 1, pos.1)
                    } else if prev == (pos.0 + 1, pos.1) {
                        prev = pos;
                        pos = (pos.0, pos.1+1)
                    } else {
                        panic!()
                    }
                }
                'L' => {
                    if prev == (pos.0 - 1, pos.1) {
                        prev = pos;
                        pos = (pos.0, pos.1 + 1)
                    } else if prev == (pos.0, pos.1 + 1) {
                        prev = pos;
                        pos = (pos.0 - 1, pos.1)
                    } else {
                        panic!()
                    }
                }
                'J' => {
                    if prev == (pos.0 - 1, pos.1) {
                        prev = pos;
                        pos = (pos.0, pos.1 - 1)
                    } else if prev == (pos.0, pos.1 - 1) {
                        prev = pos;
                        pos = (pos.0 - 1, pos.1)
                    } else {
                        panic!()
                    }
                }
                _ => panic!(),
            }
            solution += 1;
        }
        return solution/2;
    }

    2137
}

fn main() {
    println!("{}", part1("Inputs/Day10/input"));
    // println!("{}", part2("Inputs/Day8/input"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1("Inputs/Day10/example"), 8);
    }
    // #[test]
    // fn test_part1_example2() {
    //     assert_eq!(part1("Inputs/Day8/example2"), 6);
    // }
    // #[test]
    // fn test_part2_example1() {
    //     assert_eq!(part2("Inputs/Day8/example1"), 2);
    // }
    // #[test]
    // fn test_part2_example2() {
    //     assert_eq!(part2("Inputs/Day8/example2"), 6);
    // }

    // #[test]
    // fn test_part2_example3() {
    //     assert_eq!(part2("Inputs/Day8/example3"), 6);
    // }
    // #[test]
    // fn test_part2_my_map() {
    //     assert_eq!(part2("Inputs/Day8/example3"), 6);
    // }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
