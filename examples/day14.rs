use debug_print::debug_print;
use debug_print::debug_println;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn print_map(map: &Vec<Vec<char>>) {
    debug_println!("");
    for line in map {
        for c in line {
            debug_print!("{}", c);
        }
        debug_println!("");
    }
}

fn apply_gravity(map: &mut Vec<Vec<char>>) {
    for i in 1..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'O' {
                for k in (0..i).rev() {
                    if map[k][j] != '.' {
                        map[i][j] = '.';
                        map[k + 1][j] = 'O';
                        break;
                    }
                    if k == 0 {
                        map[i][j] = '.';
                        map[0][j] = 'O';
                    }
                }
            }
        }
    }
}

fn tilt_north(map: &mut Vec<Vec<char>>) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'O' {
                for k in (0..i).rev() {
                    if map[k][j] != '.' {
                        map[i][j] = '.';
                        map[k + 1][j] = 'O';
                        break;
                    }
                    if k == 0 {
                        map[i][j] = '.';
                        map[0][j] = 'O';
                    }
                }
            }
        }
    }
}

fn tilt_west(map: &mut Vec<Vec<char>>) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'O' {
                for k in (0..j).rev() {
                    if map[i][k] != '.' {
                        map[i][j] = '.';
                        map[i][k + 1] = 'O';
                        break;
                    }
                    if k == 0 {
                        map[i][j] = '.';
                        map[i][0] = 'O';
                    }
                }
            }
        }
    }
}

fn tilt_east(map: &mut Vec<Vec<char>>) {
    let width = map[0].len();
    for i in 0..map.len() {
        for j in (0..width).rev() {
            if map[i][j] == 'O' {
                for k in j + 1..width {
                    if map[i][k] != '.' {
                        map[i][j] = '.';
                        map[i][k - 1] = 'O';
                        break;
                    }
                    if k == width - 1 {
                        map[i][j] = '.';
                        map[i][width - 1] = 'O';
                    }
                }
            }
        }
    }
}

fn tilt_south(map: &mut Vec<Vec<char>>) {
    let height = map.len();
    let width = map[0].len();
    for i in (0..height).rev() {
        for j in 0..width {
            if map[i][j] == 'O' {
                for k in i + 1..height {
                    if map[k][j] != '.' {
                        map[i][j] = '.';
                        map[k - 1][j] = 'O';
                        break;
                    }
                    if k == height - 1 {
                        map[i][j] = '.';
                        map[height - 1][j] = 'O';
                    }
                }
            }
        }
    }
}

fn part1(filename: &str) -> usize {
    let mut solution = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            map.push(line.unwrap().chars().collect());
        }
        apply_gravity(&mut map);
        for (pos, line) in map.iter().enumerate() {
            for c in line {
                debug_print!("{}", c);
                if *c == 'O' {
                    solution += map.len() - pos;
                }
            }
            debug_println!("");
        }
        return solution;
    }
    panic!();
}

fn part2(filename: &str) -> usize {
    let mut solution = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            map.push(line.unwrap().chars().collect());
        }
        let mut maps = vec![map.clone()];
        let mut cycle_len = 0;
        let mut cycle_start = 0;
        for i in 0..1000000000 {
            tilt_north(&mut map);
            // print_map(&map);
            tilt_west(&mut map);
            // print_map(&map);
            tilt_south(&mut map);
            // print_map(&map);
            tilt_east(&mut map);
            print_map(&map);

            match maps.iter().position(|x| *x == map) {
                Some(idx) => {
                    cycle_start = idx;
                    cycle_len = i - cycle_start + 1;
                }
                None => (),
            }
            if maps.contains(&map) {
                break;
            }

            maps.push(map.clone());
        }
        debug_println!("cycle_start {}", cycle_start);
        debug_println!("cycle_len {}", cycle_len);
        let foo = 1000000000 - cycle_start;
        let bar = foo % cycle_len;

        let idx = cycle_start + bar;
        debug_println!("idx {}", idx);
        map = maps[idx].clone();

        for (pos, line) in map.iter().enumerate() {
            for c in line {
                if *c == 'O' {
                    solution += map.len() - pos;
                }
            }
        }
        return solution;
    }
    panic!();
}

fn main() {
    let args: Vec<u8> = env::args()
        .skip(1 )
        .map(|arg| arg.parse::<u8>().expect("Invalid arguments"))
        .collect();

    if args.len() > 0 {
        for arg in args {
            match arg {
                1 => println!("{}", part1("Inputs/Day14/input")),
                2 => println!("{}", part2("Inputs/Day14/input")),
                _ => panic!("Invalid part"),
            }
        }
    } else {
        println!("{}", part1("Inputs/Day14/input"));
        println!("{}", part2("Inputs/Day14/input"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1("Inputs/Day14/example"), 136);
    }
    #[test]
    fn test_part2_example1() {
        assert_eq!(part2("Inputs/Day14/example"), 64);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
