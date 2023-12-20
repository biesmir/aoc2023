use debug_print::debug_print;
use debug_print::debug_println;
use std::cmp::min;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Map {
    map: Vec<Vec<bool>>,
    pos_x: usize,
    pos_y: usize,
}

impl Map {
    fn dig(&mut self, dir: Direction) {
        match dir {
            Direction::North => {
                if self.pos_y > 0 {
                    self.pos_y -= 1;
                    self.map[self.pos_y][self.pos_x] = true;
                } else {
                    let mut line = vec![false; self.map[0].len()];
                    line[self.pos_x] = true;
                    self.map.insert(0, line);
                }
            }
            Direction::West => {
                if self.pos_x > 0 {
                    self.pos_x -= 1;
                    self.map[self.pos_y][self.pos_x] = true;
                } else {
                    for line in &mut self.map {
                        line.insert(0, false);
                    }
                    self.map[self.pos_y][self.pos_x] = true;
                }
            }
            Direction::South => {
                self.pos_y += 1;
                if self.pos_y < self.map.len() {
                    self.map[self.pos_y][self.pos_x] = true;
                } else {
                    let mut line = vec![false; self.map[0].len()];
                    line[self.pos_x] = true;
                    self.map.push(line);
                }
            }
            Direction::East => {
                self.pos_x += 1;
                if self.pos_x < self.map[0].len() {
                    self.map[self.pos_y][self.pos_x] = true;
                } else {
                    for line in &mut self.map {
                        line.push(false);
                    }
                    self.map[self.pos_y][self.pos_x] = true;
                }
            }
        }
    }
}

fn part1(filename: &str) -> usize {
    let mut map = Map {
        pos_x: 0,
        pos_y: 0,
        map: vec![vec![true; 1]; 1],
    };
    let mut last: char = 'F';
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let line = line.unwrap();
            let fields: Vec<&str> = line.split_whitespace().collect();
            let dir: Direction;
            match fields[0] {
                "U" => dir = Direction::North,
                "R" => dir = Direction::East,
                "D" => dir = Direction::South,
                "L" => dir = Direction::West,
                _ => panic!(),
            }
            last = fields[0].chars().nth(0).unwrap();
            for _ in 0..fields[1].parse::<usize>().unwrap() {
                map.dig(dir);
            }
            print_map(&map.map);
        }

        let mut to_visit: Vec<(usize, usize)> = Vec::new();
        if last == 'U' {
            if !map.map[map.pos_y + 1][map.pos_x + 1] {
                to_visit.push((map.pos_y + 1, map.pos_x + 1));
                loop {
                    if let Some(current) = to_visit.pop() {
                        map.map[current.0][current.1] = true;
                        if !map.map[current.0 + 1][current.1] {
                            to_visit.push((current.0 + 1, current.1));
                        }
                        if !map.map[current.0 - 1][current.1] {
                            to_visit.push((current.0 - 1, current.1));
                        }
                        if !map.map[current.0][current.1 - 1] {
                            to_visit.push((current.0, current.1 - 1));
                        }
                        if !map.map[current.0][current.1 + 1] {
                            to_visit.push((current.0, current.1 + 1));
                        }
                    } else {
                        break;
                    }
                }
            } else {
                todo!();
            }
        } else {
            todo!();
        }
        let mut solution = 0;
        for line in &map.map {
            for p in line {
                if *p {
                    solution += 1;
                }
            }
        }
        return solution;
    }
    todo!()
}

fn part2(filename: &str) -> usize {
    todo!();
}

fn main() {
    let args: Vec<u8> = env::args()
        .skip(1)
        .map(|arg| arg.parse::<u8>().expect("Invalid arguments"))
        .collect();

    if args.len() > 0 {
        for arg in args {
            match arg {
                1 => println!("{}", part1("Inputs/Day18/input")),
                2 => println!("{}", part2("Inputs/Day18/input")),
                _ => panic!("Invalid part"),
            }
        }
    } else {
        println!("{}", part1("Inputs/Day18/input"));
        println!("{}", part2("Inputs/Day18/input"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1("Inputs/Day18/example"), 62);
    }
    // #[test]
    // fn test_part2_example1() {
    //     assert_eq!(part2("Inputs/Day18/example"), 94);
    // }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn print_map(map: &Vec<Vec<bool>>) {
    debug_println!("");
    for line in map {
        for c in line {
            if *c {
                debug_print!("#");
            } else {
                debug_print!(" ");
            }
        }
        debug_println!("");
    }
}
