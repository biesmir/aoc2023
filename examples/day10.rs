use std::collections::HashMap;
use std::collections::HashSet;
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
                        pos = (pos.0, pos.1 - 1)
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
                        pos = (pos.0, pos.1 + 1)
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
        return solution / 2;
    }

    panic!();
}

fn part2(filename: &str) -> usize {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut path: Vec<(usize, usize)> = Vec::new();
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
        path.push(start_pos);
        let start_pos = start_pos;
        // let mut i = 1;
        let mut pos = start_pos;
        // println!("pos {} {}", pos.0 + 1, pos.1);

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
                        pos = (pos.0, pos.1 - 1)
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
                        pos = (pos.0, pos.1 + 1)
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
            path.push(pos);
        }

        let mut not_enclosed: Vec<(usize, usize)> = Vec::new();
        let mut to_visit: Vec<(usize, usize)> = Vec::new();

        for i in 0..map.len() {
            let pos = (i, 0);
            if !path.contains(&pos) {
                to_visit.push(pos);
            }
        }
        for i in 0..map.len() {
            let pos = (i, map[0].len() - 1);
            if !path.contains(&pos) {
                to_visit.push(pos);
            }
        }
        for i in 1..map[0].len() - 1 {
            let pos = (0, i);
            if !path.contains(&pos) {
                to_visit.push(pos);
            }
        }

        for i in 1..map[0].len() - 1 {
            let pos = (map.len() - 1, i);
            if !path.contains(&pos) {
                to_visit.push(pos);
            }
        }

        while to_visit.len() > 0 {
            let pos = to_visit.pop().unwrap();

            if !not_enclosed.contains(&pos) && !path.contains(&pos) {
                if pos.0 > 0 {
                    to_visit.push((pos.0 - 1, pos.1));
                }
                if pos.1 > 0 {
                    to_visit.push((pos.0, pos.1 - 1));
                    if pos.0 > 0 {
                        to_visit.push((pos.0 - 1, pos.1 - 1));
                    }
                }

                if pos.0 < map.len() - 1 {
                    to_visit.push((pos.0 + 1, pos.1));
                    if pos.1 > 0 {
                        to_visit.push((pos.0 + 1, pos.1 - 1));
                    }
                    if pos.1 < map[0].len() - 1 {
                        to_visit.push((pos.0 + 1, pos.1 + 1));
                    }
                }

                if pos.1 < map[0].len() - 1 {
                    to_visit.push((pos.0, pos.1 + 1));
                    if pos.0 > 0 {
                        to_visit.push((pos.0 - 1, pos.1 + 1));
                    }
                }

                not_enclosed.push(pos);
            }
        }

        // for i in 0..map.len() {
        //     for j in 0..map[0].len() {
        //         let pos = (i, j);
        //         if !not_enclosed.contains(&pos) && !path.contains(&pos) {
        //             let mut pipes_to_edge = (0, 0, 0, 0);
        //             for k in 0..pos.0 {
        //                 let tmp = (k, pos.1);
        //                 if path.contains(&tmp) {
        //                     pipes_to_edge.0 += 1;
        //                 }
        //             }
        //             for k in 0..pos.1 {
        //                 let tmp = (pos.0, k);
        //                 if path.contains(&tmp) {
        //                     pipes_to_edge.1 += 1;
        //                 }
        //             }
        //             for k in pos.1+1..map[0].len() {
        //                 let tmp = (pos.0, k);
        //                 if path.contains(&tmp) {
        //                     pipes_to_edge.2 += 1;
        //                 }
        //             }
        //             for k in pos.0+1..map.len() {
        //                 let tmp = (k, pos.1);
        //                 if path.contains(&tmp) {
        //                     pipes_to_edge.3 += 1;
        //                 }
        //             }
        //             if pipes_to_edge.0 % 2 == 0
        //                 && pipes_to_edge.1 % 2 == 0
        //                 && pipes_to_edge.2 % 2 == 0
        //                 && pipes_to_edge.3 % 2 == 0
        //             {
        //                 not_enclosed.push(pos);
        //             }
        //         }
        //     }
        // }

        for i in 0..map.len() {
            for j in 0..map[0].len() {
                let pos = (i, j);
                if not_enclosed.contains(&pos) {
                    print!("O");
                } else if path.contains(&pos) {
                    print!("{}", map[i][j]);
                } else {
                    print!("I");
                }
            }
            println!("");
        }

        let mut seen = HashSet::new();

        // Retain only the unique elements in the vector
        path.retain(|&x| seen.insert(x));
        not_enclosed.retain(|&x| seen.insert(x));

        return map.len() * map[0].len() - seen.len();
    }

    2137
}

fn main() {
    // println!("{}", part1("Inputs/Day10/input"));
    println!("{}", part2("Inputs/Day10/input"));
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
    #[test]
    fn test_part2_example2() {
        assert_eq!(part2("Inputs/Day10/example2"), 10);
    }
    #[test]
    fn test_part2_example1() {
        assert_eq!(part2("Inputs/Day10/example1"), 8);
    }
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
