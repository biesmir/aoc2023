use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use atoi::atoi;
use std::cmp;

fn find_num_beginning(schematic: &Vec<Vec<u8>>, i: usize, idx: usize) -> usize {
    let mut k = idx;
    while k > 0 {
        if schematic[i][k - 1] >= b"0"[0] && schematic[i][k - 1] <= b"9"[0] {
            k -= 1;
        } else {
            break;
        }
    }
    return k;
}

fn find_gears(schematic: Vec<Vec<u8>>) -> u32 {
    let mut result = 0;
    for i in 0..schematic.len() {
        for j in 0..schematic[0].len() {
            let mut numbers_found: Vec<u32> = Vec::new();
            if schematic[i][j] == b"*"[0] {
                let left_limit;
                let right_limit;
                if j > 0 {
                    left_limit = j - 1;
                } else {
                    left_limit = 0;
                }
                if j < schematic[0].len() - 1 {
                    right_limit = j + 1;
                } else {
                    right_limit = schematic[0].len() - 1;
                }

                if j > 0 {
                    if schematic[i][left_limit] >= b"0"[0] && schematic[i][left_limit] <= b"9"[0] {
                        // find the number beginning
                        let k = find_num_beginning(&schematic, i, left_limit);
                        numbers_found.push(atoi::<u32>(&schematic[i][k..]).unwrap());
                    }
                }
                if j < schematic[0].len() + 1 {
                    if schematic[i][j + 1] >= b"0"[0] && schematic[i][j + 1] <= b"9"[0] {
                        numbers_found.push(atoi::<u32>(&schematic[i][j + 1..]).unwrap());
                    }
                }

                if i > 0 {
                    if schematic[i - 1][j] >= b"0"[0] && schematic[i - 1][j] <= b"9"[0] {
                        let num_start_idx = find_num_beginning(&schematic, i - 1, j);
                        numbers_found
                            .push(atoi::<u32>(&schematic[i - 1][num_start_idx..]).unwrap());
                    } else {
                        if schematic[i - 1][left_limit] >= b"0"[0]
                            && schematic[left_limit][j] <= b"9"[0]
                        {
                            let num_start_idx = find_num_beginning(&schematic, i - 1, left_limit);
                            numbers_found
                                .push(atoi::<u32>(&schematic[i - 1][num_start_idx..]).unwrap());
                        }
                        if schematic[i - 1][right_limit] >= b"0"[0]
                            && schematic[right_limit][j] <= b"9"[0]
                        {
                            let num_start_idx = find_num_beginning(&schematic, i - 1, right_limit);
                            numbers_found
                                .push(atoi::<u32>(&schematic[i - 1][num_start_idx..]).unwrap());
                        }
                    }
                }
                if i < schematic.len() - 1 {
                    if schematic[i + 1][j] >= b"0"[0] && schematic[i + 1][j] <= b"9"[0] {
                        let num_start_idx = find_num_beginning(&schematic, i + 1, j);
                        numbers_found
                            .push(atoi::<u32>(&schematic[i + 1][num_start_idx..]).unwrap());
                    } else {
                        if schematic[i + 1][left_limit] >= b"0"[0]
                            && schematic[left_limit][j] <= b"9"[0]
                        {
                            let num_start_idx = find_num_beginning(&schematic, i + 1, left_limit);
                            numbers_found
                                .push(atoi::<u32>(&schematic[i + 1][num_start_idx..]).unwrap());
                        }
                        if schematic[i + 1][right_limit] >= b"0"[0]
                            && schematic[right_limit][j] <= b"9"[0]
                        {
                            let num_start_idx = find_num_beginning(&schematic, i + 1, right_limit);
                            numbers_found
                                .push(atoi::<u32>(&schematic[i + 1][num_start_idx..]).unwrap());
                        }
                    }
                }
                if numbers_found.len() == 2 {
                    result += numbers_found[0] * numbers_found[1];
                }
            }
        }
    }
    result
}

fn count_parts(schematic: Vec<Vec<u8>>) -> u32 {
    let symbols = vec!['!', '#', '%', '&', '*', '+', '-', '/', '=', '@', '$'];
    let mut result = 0;
    for i in 0..schematic.len() {
        println!("result is {}", result);
        let mut last_num_idx = 0;
        let mut number: u32 = 0;
        let mut num_len: usize = 0;
        for j in 0..schematic[0].len() {
            if last_num_idx + num_len > j {
                continue;
            }
            if schematic[i][j] >= b"0"[0] && schematic[i][j] <= b"9"[0] {
                number = atoi::<u32>(&schematic[i][j..]).unwrap();
                num_len = (number.checked_ilog10().unwrap_or(0) + 1)
                    .try_into()
                    .unwrap();
                last_num_idx = j;
                'finding_symbols: for symbol in symbols.iter() {
                    if j > 0 {
                        if schematic[i][j - 1] == *symbol as u8 {
                            result += number;
                            break 'finding_symbols;
                        }
                    }
                    if j + num_len < schematic[0].len() - 1 {
                        if schematic[i][j + num_len] == *symbol as u8 {
                            result += number;
                            break 'finding_symbols;
                        }
                    }
                    let search_start;
                    if j == 0 {
                        search_start = 0;
                    } else {
                        search_start = j - 1;
                    }
                    let search_stop = cmp::min(j + num_len + 1, schematic[0].len() - 1);

                    for k in search_start..search_stop {
                        if k < schematic[i].len() - 1 {
                            if i > 0 {
                                if schematic[i - 1][k] == *symbol as u8 {
                                    result += number;
                                    break 'finding_symbols;
                                }
                            }
                            if i < schematic.len() - 1 {
                                if schematic[i + 1][k] == *symbol as u8 {
                                    result += number;
                                    break 'finding_symbols;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return result;
}


fn part1(input_file: &str) -> u32 {
    let mut schematic: Vec<Vec<u8>> = Vec::new();
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            schematic.push(line.unwrap().as_bytes().to_vec());
        }
        return count_parts(schematic);
    }
    0
}

fn part2(input_file: &str) -> u32 {
    let mut schematic: Vec<Vec<u8>> = Vec::new();
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            schematic.push(line.unwrap().as_bytes().to_vec());
        }
        return find_gears(schematic);
    }
    0
}


fn main() {
    println!("{}", part1("Inputs/Day3/input"));
    println!("{}", part2("Inputs/Day3/input"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_day3() {
        assert_eq!(part1("Inputs/Day3/example"), 4361);
    }
    #[test]
    fn test_example_day3_part2() {
        assert_eq!(part2("Inputs/Day3/example"), 467835);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
