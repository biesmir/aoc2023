use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(filename: &str) -> u64 {
    let mut map: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let mut line_vec: Vec<char> = Vec::new();
            for c in line.unwrap().chars() {
                line_vec.push(c);
            }
            map.push(line_vec);
        }

        let mut empty_columns: Vec<usize> = Vec::new();
        let mut empty_rows: Vec<usize> = Vec::new();
        for row in 0..map.len() {
            let mut empty = true;
            for column in 0..map[0].len() {
                if map[row][column] == '#' {
                    empty = false;
                    break;
                }
            }
            if empty {
                empty_rows.push(row);
            }
        }

        for column in 0..map[0].len() {
            let mut empty = true;
            for row in 0..map.len() {
                if map[row][column] == '#' {
                    empty = false;
                    break;
                }
            }
            if empty {
                empty_columns.push(column);
            }
        }

        for &row in empty_rows.iter().rev() {
            let new_row: Vec<char> = map[row].clone();
            map.insert(row, new_row);
        }

        for &column in empty_columns.iter().rev() {
            for i in 0..map.len() {
                map[i].insert(column, '.');
            }
        }

        for i in 0..map.len() {
            for j in 0..map[0].len() {
                print!("{}", map[i][j]);
            }
            println!("");
        }
        let mut stars: Vec<(usize, usize)> = Vec::new();
        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if map[i][j] == '#' {
                    stars.push((i, j));
                }
            }
        }
        let mut solution = 0;
        for i in 0..stars.len() {
            for j in i + 1..stars.len() {
                solution += calculate_len(stars[i], stars[j]);
            }
        }
        return solution;
    }

    panic!();
}

fn calculate_len(p1: (usize, usize), p2: (usize, usize)) -> u64 {
    let x_distance: u64;
    let y_distance: u64;
    if p1.0 > p2.0 {
        x_distance = (p1.0 - p2.0).try_into().unwrap();
    } else {
        x_distance = (p2.0 - p1.0).try_into().unwrap();
    }
    if p1.1 > p2.1 {
        y_distance = (p1.1 - p2.1).try_into().unwrap();
    } else {
        y_distance = (p2.1 - p1.1).try_into().unwrap();
    }

    return x_distance + y_distance;
}

fn part2(filename: &str) -> u64 {
    let mut map: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let mut line_vec: Vec<char> = Vec::new();
            for c in line.unwrap().chars() {
                line_vec.push(c);
            }
            map.push(line_vec);
        }

        let mut empty_columns: Vec<usize> = Vec::new();
        let mut empty_rows: Vec<usize> = Vec::new();
        for row in 0..map.len() {
            let mut empty = true;
            for column in 0..map[0].len() {
                if map[row][column] == '#' {
                    empty = false;
                    break;
                }
            }
            if empty {
                empty_rows.push(row);
            }
        }

        for column in 0..map[0].len() {
            let mut empty = true;
            for row in 0..map.len() {
                if map[row][column] == '#' {
                    empty = false;
                    break;
                }
            }
            if empty {
                empty_columns.push(column);
            }
        }

        // for &row in empty_rows.iter().rev() {
        //     let new_row: Vec<char> = map[row].clone();
        //     map.insert(row, new_row);
        // }

        // for &column in empty_columns.iter().rev() {
        //     for i in 0..map.len() {
        //         map[i].insert(column, '.');
        //     }
        // }

        for i in 0..map.len() {
            for j in 0..map[0].len() {
                print!("{}", map[i][j]);
            }
            println!("");
        }
        let mut stars: Vec<(usize, usize)> = Vec::new();
        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if map[i][j] == '#' {
                    stars.push((i, j));
                }
            }
        }
        let mut solution = 0;
        for i in 0..stars.len() {
            for j in i + 1..stars.len() {
                solution += calculate_len(stars[i], stars[j]);
                if stars[i].0 <= stars[j].0 {
                    for k in stars[i].0..stars[j].0 {
                        if empty_rows.contains(&k) {
                            solution += 1000000 - 1;
                        }
                    }
                } else {
                    for k in stars[j].0..stars[i].0 {
                        if empty_rows.contains(&k) {
                            solution += 1000000 - 1;
                        }
                    }
                }
                if stars[i].1 <= stars[j].1 {
                    for k in stars[i].1..stars[j].1 {
                        if empty_columns.contains(&k) {
                            solution += 1000000 - 1;
                        }
                    }
                } else {
                    for k in stars[j].1..stars[i].1 {
                        if empty_columns.contains(&k) {
                            solution += 1000000 - 1;
                        }
                    }
                }
            }
        }
        return solution;
    }

    panic!();
}

fn main() {
    println!("{}", part1("Inputs/Day11/input"));
    println!("{}", part2("Inputs/Day11/input"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1("Inputs/Day11/example"), 374);
    }
    #[test]
    fn test_part2_example1() {
        assert_eq!(part2("Inputs/Day11/example"), 8410);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
