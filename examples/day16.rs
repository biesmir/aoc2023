use debug_print::debug_print;
use debug_print::debug_println;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{thread, time};

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn try_moving(
    map: &mut Vec<Vec<char>>,
    energized: &mut HashMap<usize, bool>,
    to_visit: &mut Vec<(usize, usize, Direction)>,
    visited: &mut Vec<(usize, usize, Direction)>,
    node: &(usize, usize, Direction),
    dir: &Direction,
) {
    // let delay = time::Duration::from_millis(10);
    // print_map_with_mark(&map, node.0, node.1);
    // thread::sleep(delay);
    match dir {
        Direction::East => {
            if node.1 + 1 < map[0].len() {
                let new_node = (node.0, node.1 + 1, *dir);
                if !visited.contains(&new_node) {
                    visited.push(new_node);
                    to_visit.push(new_node);
                    energized.insert((node.0) + (node.1 + 1) * 1000, true);
                    debug_println!("visiting {} {}", node.0, node.1);
                }
            }
        }
        Direction::South => {
            if node.0 + 1 < map.len() {
                let new_node = (node.0 + 1, node.1, *dir);
                if !visited.contains(&new_node) {
                    to_visit.push(new_node);
                    visited.push(new_node);
                    energized.insert((node.0 + 1) + (node.1) * 1000, true);
                    debug_println!("visiting {} {}", node.0, node.1);
                }
            }
        }
        Direction::North => {
            if node.0 > 0 {
                let new_node = (node.0 - 1, node.1, *dir);
                if !visited.contains(&new_node) {
                    visited.push(new_node);
                    to_visit.push(new_node);
                    energized.insert((node.0 - 1) + (node.1) * 1000, true);
                    debug_println!("visiting {} {}", node.0, node.1);
                }
            }
        }
        Direction::West => {
            if node.1 > 0 {
                let new_node = (node.0, node.1 - 1, *dir);
                if !visited.contains(&new_node) {
                    visited.push(new_node);
                    to_visit.push(new_node);
                    energized.insert((node.0) + (node.1 - 1) * 1000, true);
                    debug_println!("visiting {} {}", node.0, node.1);
                }
            }
        }
    }
}

fn part1(filename: &str) -> usize {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut energized: HashMap<usize, bool> = HashMap::new();
    let mut to_visit: Vec<(usize, usize, Direction)> = Vec::new();
    let mut visited: Vec<(usize, usize, Direction)> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let chars: Vec<char> = line.unwrap().chars().collect();
            map.push(chars);
        }
        print_map(&map);
        energized.insert(0, true);
        to_visit.push((0, 0, Direction::East));

        loop {
            match to_visit.pop() {
                Some(node) => match map[node.0][node.1] {
                    '\\' => match node.2 {
                        Direction::North => {
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::West,
                            );
                        }
                        Direction::East => {
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::South,
                            );
                        }

                        Direction::South => {
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::East,
                            );
                        }
                        Direction::West => {
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::North,
                            );
                        }
                    },
                    '/' => match node.2 {
                        Direction::North => {
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::East,
                            );
                        }
                        Direction::East => {
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::North,
                            );
                        }

                        Direction::South => {
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::West,
                            );
                        }
                        Direction::West => {
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::South,
                            );
                        }
                    },
                    '-' => match node.2 {
                        Direction::South | Direction::North => {
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::East,
                            );
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::West,
                            );
                        }
                        Direction::West | Direction::East => try_moving(
                            &mut map,
                            &mut energized,
                            &mut to_visit,
                            &mut visited,
                            &node,
                            &node.2,
                        ),
                    },
                    '|' => match node.2 {
                        Direction::West | Direction::East => {
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::South,
                            );
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::North,
                            );
                        }
                        Direction::South | Direction::North => try_moving(
                            &mut map,
                            &mut energized,
                            &mut to_visit,
                            &mut visited,
                            &node,
                            &node.2,
                        ),
                    },
                    '.' => try_moving(
                        &mut map,
                        &mut energized,
                        &mut to_visit,
                        &mut visited,
                        &node,
                        &node.2,
                    ),
                    _ => panic!("Unknown symbol"),
                },
                None => break,
            }
        }

        return energized.len();
    }

    panic!();
}

fn count_energized(filename: &str, initial_node: (usize, usize, Direction)) -> usize {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut energized: HashMap<usize, bool> = HashMap::new();
    let mut to_visit: Vec<(usize, usize, Direction)> = Vec::new();
    let mut visited: Vec<(usize, usize, Direction)> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let chars: Vec<char> = line.unwrap().chars().collect();
            map.push(chars);
        }
        energized.insert(initial_node.0+initial_node.1*1000, true);
        to_visit.push(initial_node);
        // visited.push(initial_node);

        loop {
            match to_visit.pop() {
                Some(node) => match map[node.0][node.1] {
                    '\\' => match node.2 {
                        Direction::North => {
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::West,
                            );
                        }
                        Direction::East => {
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::South,
                            );
                        }

                        Direction::South => {
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::East,
                            );
                        }
                        Direction::West => {
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::North,
                            );
                        }
                    },
                    '/' => match node.2 {
                        Direction::North => {
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::East,
                            );
                        }
                        Direction::East => {
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::North,
                            );
                        }

                        Direction::South => {
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::West,
                            );
                        }
                        Direction::West => {
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::South,
                            );
                        }
                    },
                    '-' => match node.2 {
                        Direction::South | Direction::North => {
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::East,
                            );
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::West,
                            );
                        }
                        Direction::West | Direction::East => try_moving(
                            &mut map,
                            &mut energized,
                            &mut to_visit,
                            &mut visited,
                            &node,
                            &node.2,
                        ),
                    },
                    '|' => match node.2 {
                        Direction::West | Direction::East => {
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::South,
                            );
                            try_moving(
                                &mut map,
                                &mut energized,
                                &mut to_visit,
                                &mut visited,
                                &node,
                                &Direction::North,
                            );
                        }
                        Direction::South | Direction::North => try_moving(
                            &mut map,
                            &mut energized,
                            &mut to_visit,
                            &mut visited,
                            &node,
                            &node.2,
                        ),
                    },
                    '.' => try_moving(
                        &mut map,
                        &mut energized,
                        &mut to_visit,
                        &mut visited,
                        &node,
                        &node.2,
                    ),
                    _ => panic!("Unknown symbol"),
                },
                None => break,
            }
        }

        return energized.len();
    }

    panic!();
}

fn part2(filename: &str) -> usize{
    let mut ener_count:Vec<usize> = Vec::new();

    ener_count.push(count_energized(filename, (0, 0, Direction::East)));
    ener_count.push(count_energized(filename, (0, 0, Direction::South)));
    ener_count.push(count_energized(filename, (109, 0, Direction::East)));
    ener_count.push(count_energized(filename, (109, 0, Direction::North)));
    ener_count.push(count_energized(filename, (109, 109, Direction::West)));
    ener_count.push(count_energized(filename, (109, 109, Direction::North)));
    ener_count.push(count_energized(filename, (0, 109, Direction::West)));
    ener_count.push(count_energized(filename, (0, 109, Direction::South)));

    //left edge
    for i in 1..109{
        ener_count.push(count_energized(filename, (i, 0, Direction::East)));
        ener_count.push(count_energized(filename, (i, 0, Direction::North)));
        ener_count.push(count_energized(filename, (i, 0, Direction::South)));
    }

    //upper edge
    for i in 1..109{
        ener_count.push(count_energized(filename, (0, i, Direction::East)));
        ener_count.push(count_energized(filename, (0, i, Direction::West)));
        ener_count.push(count_energized(filename, (0, i, Direction::South)));
    }

    //lower edge
    for i in 1..109{
        ener_count.push(count_energized(filename, (109, i, Direction::East)));
        ener_count.push(count_energized(filename, (109, i, Direction::North)));
        ener_count.push(count_energized(filename, (109, i, Direction::West)));
    }
    //right edge
    for i in 1..109{
        ener_count.push(count_energized(filename, (i, 109, Direction::West)));
        ener_count.push(count_energized(filename, (i, 109, Direction::North)));
        ener_count.push(count_energized(filename, (i, 109, Direction::South)));
    }

    return *ener_count.iter().max().unwrap();

}



fn main() {
    let args: Vec<u8> = env::args()
        .skip(1)
        .map(|arg| arg.parse::<u8>().expect("Invalid arguments"))
        .collect();

    if args.len() > 0 {
        for arg in args {
            match arg {
                1 => println!("{}", part1("Inputs/Day16/input")),
                2 => println!("{}", part2("Inputs/Day16/input")),
                _ => panic!("Invalid part"),
            }
        }
    } else {
        println!("{}", part1("Inputs/Day16/example"));
        println!("{}", part2("Inputs/Day16/input"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1("Inputs/Day16/example"), 46);
    }
    // #[test]
    // fn test_part2_example1() {
    //     assert_eq!(part2("Inputs/Day16/example"), 145);
    // }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn print_map(map: &Vec<Vec<char>>) {
    debug_println!("");
    for line in map {
        for c in line {
            debug_print!("{}", c);
        }
        debug_println!("");
    }
}

fn print_map_with_mark(map: &Vec<Vec<char>>, x: usize, y: usize) {
    debug_println!("");
    let mut map = map.clone();
    map[x][y] = 'X';
    for line in map {
        for c in line {
            debug_print!("{}", c);
        }
        debug_println!("");
    }
}
