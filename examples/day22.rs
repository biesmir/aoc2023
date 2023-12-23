use debug_print::debug_print;
use debug_print::debug_println;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone)]
struct Brick {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
    z1: usize,
    z2: usize,
    supported_by: HashSet<usize>,
    supports: HashSet<usize>,
}

impl Brick {
    fn get_ocupancy(&self) -> Vec<(usize, usize, usize)> {
        let mut found: Vec<(usize, usize, usize)> = Vec::new();
        if self.x1 != self.x2 {
            for i in min(self.x1, self.x2)..max(self.x1, self.x2) + 1 {
                found.push((i, self.y1, self.z1));
            }
        } else if self.y1 != self.y2 {
            for i in min(self.y1, self.y2)..max(self.y1, self.y2) + 1 {
                found.push((self.x1, i, self.z1));
            }
        } else if self.z1 != self.z2 {
            for i in min(self.z1, self.z2)..max(self.z1, self.z2) + 1 {
                found.push((self.x1, self.y1, i));
            }
        } else {
            found.push((self.x1, self.y1, self.z1));
        }
        return found;
    }
}

fn apply_gravity(bricks: &mut Vec<Brick>) -> Vec<Brick> {
    let mut fallen_bricks: Vec<Brick> = Vec::new();

    bricks.sort_by_key(|item| item.z1);
    bricks.reverse();

    loop {
        if let Some(mut brick) = bricks.pop() {
            for i in (0..brick.z1).rev() {
                if i == 0 {
                    brick.z2 = brick.z2 - brick.z1;
                    brick.z1 = 0;
                    fallen_bricks.push(brick);
                    break;
                } else {
                    let mut found_support = false;
                    let brick_cubes = brick.get_ocupancy();
                    for (idx, stable_brick) in fallen_bricks.iter().enumerate() {
                        let sbrick_cubes = stable_brick.get_ocupancy();
                        for cube in &brick_cubes {
                            if sbrick_cubes.contains(&(cube.0, cube.1, i - 1)) {
                                found_support = true;
                                brick.z2 = brick.z2 - brick.z1 + i;
                                brick.z1 = i;
                                brick.supported_by.insert(idx);
                            }
                        }
                    }
                    if found_support {
                        fallen_bricks.push(brick);
                        break;
                    }
                }
            }
        } else {
            break;
        }
    }
    return fallen_bricks;
}

fn part1(filename: &str, steps: usize) -> usize {
    let mut bricks: Vec<Brick> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let line = line.unwrap();
            let (begin, end) = line.split_once('~').unwrap();
            let mut begin = begin.split(',');
            let x1 = begin.next().unwrap().parse::<usize>().unwrap();
            let y1 = begin.next().unwrap().parse::<usize>().unwrap();
            let z1 = begin.next().unwrap().parse::<usize>().unwrap();

            let mut end = end.split(',');
            let x2 = end.next().unwrap().parse::<usize>().unwrap();
            let y2 = end.next().unwrap().parse::<usize>().unwrap();
            let z2 = end.next().unwrap().parse::<usize>().unwrap();

            let (z1, z2) = (min(z1, z2), max(z1, z2));

            let brick = Brick {
                x1: x1,
                y1: y1,
                z1: z1,
                x2: x2,
                y2: y2,
                z2: z2,
                supported_by: HashSet::new(),
                supports: HashSet::new(),
            };
            bricks.push(brick);
        }
        let fallen = apply_gravity(&mut bricks);

        let mut support_list = vec![0; fallen.len()];
        let mut only_supporter = vec![false; fallen.len()];

        for brick in &fallen {
            for supporter in &brick.supported_by {
                support_list[*supporter] += 1;
                if brick.supported_by.len() == 1 {
                    only_supporter[*supporter] = true;
                }
            }
        }

        return only_supporter.iter().filter(|&&x| x == false).count();
    }

    todo!()
}

fn part2(filename: &str, steps: usize) -> usize {
    let mut bricks: Vec<Brick> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let line = line.unwrap();
            let (begin, end) = line.split_once('~').unwrap();
            let mut begin = begin.split(',');
            let x1 = begin.next().unwrap().parse::<usize>().unwrap();
            let y1 = begin.next().unwrap().parse::<usize>().unwrap();
            let z1 = begin.next().unwrap().parse::<usize>().unwrap();

            let mut end = end.split(',');
            let x2 = end.next().unwrap().parse::<usize>().unwrap();
            let y2 = end.next().unwrap().parse::<usize>().unwrap();
            let z2 = end.next().unwrap().parse::<usize>().unwrap();

            let (z1, z2) = (min(z1, z2), max(z1, z2));

            let brick = Brick {
                x1: x1,
                y1: y1,
                z1: z1,
                x2: x2,
                y2: y2,
                z2: z2,
                supported_by: HashSet::new(),
                supports: HashSet::new(),
            };
            bricks.push(brick);
        }
        let mut fallen = apply_gravity(&mut bricks);

        let mut support_list = vec![0; fallen.len()];
        let mut only_supporter = vec![false; fallen.len()];

        for brick in &fallen {
            for supporter in &brick.supported_by {
                support_list[*supporter] += 1;
                if brick.supported_by.len() == 1 {
                    only_supporter[*supporter] = true;
                }
            }
        }

        let mut new_fallen: Vec<Brick> = fallen.clone();

        for i in 0..fallen.len() {
            for supporter in &fallen[i].supported_by {
                new_fallen[*supporter].supports.insert(i);
            }
        }

        let mut solution = 0;

        for i in 0..fallen.len() {
            let mut count = 0;
            let mut to_check: Vec<usize> = vec![i];
            let mut is_disintegrated = vec![false; fallen.len()];
            is_disintegrated[i] = true;
            loop {
                if let Some(idx) = to_check.pop() {
                    for brick in &new_fallen[idx].supports {
                        let mut lost_support = true;
                        for b in &new_fallen[*brick].supported_by {
                            if !is_disintegrated[*b] {
                                lost_support = false;
                            }
                        }
                        if lost_support {
                            if !is_disintegrated[*brick] {
                                count += 1;
                                to_check.push(*brick);
                                is_disintegrated[*brick] = true;
                            }
                        }
                    }
                } else {
                    break;
                }
            }
            debug_println!("brick {} got {}", i, count);
            solution += count;
        }
        return solution;
    }

    todo!()
}

fn main() {
    let args: Vec<u8> = env::args()
        .skip(1)
        .map(|arg| arg.parse::<u8>().expect("Invalid arguments"))
        .collect();

    if args.len() > 0 {
        for arg in args {
            match arg {
                1 => println!("{}", part1("Inputs/Day22/input", 64)),
                2 => println!("{}", part2("Inputs/Day22/input", 26501365)),
                _ => panic!("Invalid part"),
            }
        }
    } else {
        println!("{}", part1("Inputs/Day22/input", 64));
        println!("{}", part2("Inputs/Day22/input", 26501365));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1("Inputs/Day22/example", 6), 5);
    }
    #[test]
    fn test_part2_example1() {
        assert_eq!(part2("Inputs/Day22/example", 6), 7);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
