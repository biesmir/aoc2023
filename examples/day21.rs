use debug_print::debug_print;
use debug_print::debug_println;
use std::cmp::min;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn count_possible_odd(map: &Vec<Vec<char>>) -> u64 {
    return count_possible_with_start(map, map.len() * 2 + 1, vec![(map.len() / 2, map.len() / 2)]);
}

fn count_possible_even(map: &Vec<Vec<char>>) -> u64 {
    return count_possible_with_start(map, map.len() * 2, vec![(map.len() / 2, map.len() / 2)]);
}

fn count_possible_with_start(
    map: &Vec<Vec<char>>,
    steps: usize,
    start_pos: Vec<(usize, usize)>,
) -> u64 {
    let mut to_visit: HashSet<(usize, usize)> = HashSet::new();
    for pos in start_pos {
        to_visit.insert(pos);
    }

    for _ in 0..steps {
        let mut new_to_visit: HashSet<(usize, usize)> = HashSet::new();
        for node in &to_visit {
            if node.0 > 0 {
                let up = (node.0 - 1, node.1);
                if map[up.0][up.1] != '#' {
                    new_to_visit.insert(up);
                }
            }
            if node.1 > 0 {
                let left = (node.0, node.1 - 1);
                if map[left.0][left.1] != '#' {
                    new_to_visit.insert(left);
                }
            }
            if node.0 < map.len() - 1 {
                let down = (node.0 + 1, node.1);
                if map[down.0][down.1] != '#' {
                    new_to_visit.insert(down);
                }
            }
            if node.1 < map[0].len() - 1 {
                let right = (node.0, node.1 + 1);
                if map[right.0][right.1] != '#' {
                    new_to_visit.insert(right);
                }
            }
        }
        to_visit = new_to_visit;
    }

    return to_visit.len() as u64;
}

fn part1(filename: &str, steps: usize) -> usize {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut to_visit: HashSet<(usize, usize)> = HashSet::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let chars: Vec<char> = line.unwrap().chars().collect();
            map.push(chars);
        }

        'outer: for (y, line) in map.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == 'S' {
                    to_visit.insert((y, x));
                    break 'outer;
                }
            }
        }

        for _ in 0..steps {
            let mut new_to_visit: HashSet<(usize, usize)> = HashSet::new();
            for node in &to_visit {
                if node.0 > 0 {
                    let up = (node.0 - 1, node.1);
                    if map[up.0][up.1] != '#' {
                        new_to_visit.insert(up);
                    }
                }
                if node.1 > 0 {
                    let left = (node.0, node.1 - 1);
                    if map[left.0][left.1] != '#' {
                        new_to_visit.insert(left);
                    }
                }
                if node.0 < map.len() - 1 {
                    let down = (node.0 + 1, node.1);
                    if map[down.0][down.1] != '#' {
                        new_to_visit.insert(down);
                    }
                }
                if node.1 < map[0].len() - 1 {
                    let right = (node.0, node.1 + 1);
                    if map[right.0][right.1] != '#' {
                        new_to_visit.insert(right);
                    }
                }
            }
            to_visit = new_to_visit;
        }
    }
    return to_visit.len();
}

fn plot_path(filename: &str, steps: usize) {
    let mut map: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let chars: Vec<char> = line.unwrap().chars().collect();
            map.push(chars);
        }

        let mut start_y = 65;
        for (pos, line) in map.iter().enumerate() {
            for c in line {
                if *c == 'S' {
                    start_y = pos;
                    break;
                }
            }
        }

        for i in steps - map.len()..steps + 1 {
            for (pos, c) in map[(i + start_y) % map.len()].iter().enumerate() {
                if pos == map.len() / 2 {
                    debug_print!("O");
                } else {
                    debug_print!("{}", c);
                }
            }
            debug_println!("");
        }
    }
}

fn plot_path_diag(filename: &str, steps: usize) {
    let mut map: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let chars: Vec<char> = line.unwrap().chars().collect();
            map.push(chars);
        }

        let mut start_y = 65;
        for (pos, line) in map.iter().enumerate() {
            for c in line {
                if *c == 'S' {
                    start_y = pos;
                    break;
                }
            }
        }

        let mut x = 0;
        let mut y = 0;
        let mut step = 0;
        while step < steps {
            for (pos, c) in map[(y + start_y) % map.len()].iter().enumerate() {
                if pos == x & map.len() || pos == (x + 1) & map.len() {
                    debug_print!("O");
                } else {
                    debug_print!("{}", c);
                }
            }
            debug_println!("");
            x += 1;
            y += 1;
            step += 2;
        }

        for i in steps - map.len()..steps + 1 {
            for (pos, c) in map[(i + start_y) % map.len()].iter().enumerate() {
                if pos == map.len() / 2 {
                    debug_print!("O");
                } else {
                    debug_print!("{}", c);
                }
            }
            debug_println!("");
        }
    }
}

// fn translate_cords(cords: (isize, isize), map: &Vec<Vec<char>>) -> (usize, usize) {
//     debug_println!("in {} {}", cords.0, cords.1);
//     let x: usize;
//     let y: usize;
//     if cords.0 >= 0 {
//         y = (cords.0 % map.len() as isize) as usize;
//     } else {
//         y = (map.len() as isize - (-cords.0 % map.len() as isize)) as usize;
//     }
//     if cords.1 >= 0 {
//         x = (cords.1 % map[0].len() as isize) as usize;
//     } else {
//         x = (map[0].len() as isize - (-cords.1 % map[0].len() as isize)) as usize;
//     }
//     debug_println!("out {} {}", y, x);
//     return (y % map.len(), x % map[0].len());
// }

fn get_edge_rd(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    // for i in (map.len() / 2..map.len()).step_by(2) {
    //     ret.push((i, map.len() - 1));
    //     ret.push((map.len() - 1, i));
    // }
    ret = vec![(0, 0)];
    return ret;
}

fn get_edge_lu(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    // for i in (0..map.len() / 2 + 1).step_by(2) {
    //     ret.push((i, 0));
    //     ret.push((0, i));
    // }
    ret = vec![(map.len() - 1, map.len() - 1)];
    return ret;
}

fn get_edge_ld(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    // for i in (0..map.len() / 2 + 1).step_by(2) {
    //     ret.push((i, map.len() - 1));
    // }

    // for i in map.len() / 2..map.len() {
    //     ret.push((0, i));
    // }
    ret = vec![(0, map.len() - 1)];
    return ret;
}

fn get_edge_ru(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    // for i in (0..map.len() / 2 + 1).step_by(2) {
    //     ret.push((0, i));
    // }

    // for i in map.len() / 2..map.len() {
    //     ret.push((i, map.len() - 1));
    // }
    ret = vec![(map.len() - 1, 0)];
    return ret;
}

fn part2(filename: &str, steps: usize) -> usize {
    let mut map: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let chars: Vec<char> = line.unwrap().chars().collect();
            map.push(chars);
        }

        assert_eq!(map.len(), map[0].len());

        let even_superblock_possible_count: u64 = count_possible_even(&map) as u64;
        let odd_superblock_possible_count: u64 = count_possible_odd(&map) as u64;

        debug_println!(
            "even_superblock_possible_count {}",
            even_superblock_possible_count
        );
        debug_println!(
            "odd_superblock_possible_count {}",
            odd_superblock_possible_count
        );

        let megastructure_radius = (steps - (map.len() / 2)) / map.len();

        let mut even_superblock_count: u64 = 0;
        let mut odd_superblock_count: u64 = 1;

        let mut prev_delta = 0;
        for i in 0..megastructure_radius {
            if i % 2 == 0 {
                odd_superblock_count += prev_delta;
                // debug_println!("odd");
            } else {
                even_superblock_count += prev_delta;
                // debug_println!("even");
            }
            prev_delta += 4;
        }

        debug_println!("odd superblock count {}", odd_superblock_count);
        debug_println!("even superblock count {}", even_superblock_count);

        debug_println!("prev delta {}", prev_delta);
        let edge_len = (prev_delta - 4) / 4;
        debug_println!("edge len {}", edge_len);

        let lu_edge_permutation =
            count_possible_with_start(&map, map.len() + map.len() / 2 - 1, get_edge_lu(&map));
        let ld_edge_permutation =
            count_possible_with_start(&map, map.len() + map.len() / 2 - 1, get_edge_ld(&map));
        let ru_edge_permutation =
            count_possible_with_start(&map, map.len() + map.len() / 2 - 1, get_edge_ru(&map));
        let rd_edge_permutation =
            count_possible_with_start(&map, map.len() + map.len() / 2 - 1, get_edge_rd(&map));

        let lu_fill_permutation =
            count_possible_with_start(&map, (map.len() - 3) / 2, get_edge_lu(&map));
        let ld_fill_permutation =
            count_possible_with_start(&map, (map.len() - 3) / 2, get_edge_ld(&map));
        let ru_fill_permutation =
            count_possible_with_start(&map, (map.len() - 3) / 2, get_edge_ru(&map));
        let rd_fill_permutation =
            count_possible_with_start(&map, (map.len() - 3) / 2, get_edge_rd(&map));

        debug_println!("lu {}", lu_edge_permutation);
        debug_println!("rd {}", rd_edge_permutation);
        debug_println!("ld {}", ld_edge_permutation);
        debug_println!("ru {}", ru_edge_permutation);

        let left_peak =
            count_possible_with_start(&map, map.len() - 1, vec![(map.len() / 2, map.len() - 1); 1]);
        let bottom_peak =
            count_possible_with_start(&map, map.len() - 1, vec![(0, map.len() / 2); 1]);
        let top_peak =
            count_possible_with_start(&map, map.len() - 1, vec![(map.len() - 1, map.len() / 2); 1]);
        let right_peak =
            count_possible_with_start(&map, map.len() - 1, vec![(map.len() / 2, 0); 1]);

        debug_println!("l peak {}", left_peak);
        debug_println!("r peak {}", right_peak);
        debug_println!("u peak {}", top_peak);
        debug_println!("d peak {}", bottom_peak);

        return (even_superblock_possible_count * even_superblock_count
            + odd_superblock_possible_count * odd_superblock_count
            + left_peak
            + right_peak
            + top_peak
            + bottom_peak
            + (lu_edge_permutation * edge_len)
            + (ru_edge_permutation * edge_len)
            + (ld_edge_permutation * edge_len)
            + (rd_edge_permutation * edge_len)
            + (lu_fill_permutation * (edge_len + 1))
            + (ru_fill_permutation * (edge_len + 1))
            + (ld_fill_permutation * (edge_len + 1))
            + (rd_fill_permutation * (edge_len + 1)))
            .try_into()
            .unwrap();
    }
    panic!()
}

fn main() {
    // plot_path("Inputs/Day21/input", 26501365);
    // plot_path_diag("Inputs/Day21/input", 26501365);
    let args: Vec<u8> = env::args()
        .skip(1)
        .map(|arg| arg.parse::<u8>().expect("Invalid arguments"))
        .collect();

    if args.len() > 0 {
        for arg in args {
            match arg {
                1 => println!("{}", part1("Inputs/Day21/input", 64)),
                2 => println!("{}", part2("Inputs/Day21/input", 26501365)),
                _ => panic!("Invalid part"),
            }
        }
    } else {
        println!("{}", part1("Inputs/Day21/input", 64));
        println!("{}", part2("Inputs/Day21/input", 26501365));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1("Inputs/Day21/example", 6), 16);
    }

    #[test]
    fn test_part1_example_by_part2() {
        let mut map: Vec<Vec<char>> = Vec::new();

        if let Ok(lines) = read_lines("Inputs/Day21/example") {
            for line in lines {
                let chars: Vec<char> = line.unwrap().chars().collect();
                map.push(chars);
            }
        }
        assert_eq!(
            count_possible_with_start(&map, 6, vec![(map.len() / 2, map.len() / 2)]),
            16
        );
    }

    #[test]
    fn test_part1_foo() {
        assert_eq!(part1("Inputs/Day21/input", 132), 7257);
    }
    #[test]
    fn test_part1_bar() {
        assert_eq!(part1("Inputs/Day21/input", 131), 7226);
    }

    // #[test]
    // fn test_part2_example1() {
    //     assert_eq!(part2("Inputs/Day21/example", 6), 16);
    // }
    // #[test]
    // fn test_part2_example2() {
    //     assert_eq!(part2("Inputs/Day21/example", 10), 50);
    // }
    // #[test]
    // fn test_part2_example3() {
    //     assert_eq!(part2("Inputs/Day21/example", 50), 1594);
    // }
    // #[test]
    // fn test_part2_example4() {
    //     assert_eq!(part2("Inputs/Day21/example", 100), 6536);
    // }
    // #[test]
    // fn test_part2_example5() {
    //     assert_eq!(part2("Inputs/Day21/example", 500), 167004);
    // }
    // #[test]
    // fn test_part2_example6() {
    //     assert_eq!(part2("Inputs/Day21/example", 1000), 668697);
    // }
    // #[test]
    // fn test_part2_example7() {
    //     assert_eq!(part2("Inputs/Day21/example", 5000), 16733044);
    // }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
