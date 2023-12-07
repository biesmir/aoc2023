use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn scratch_line_result(line: &str) -> u32 {
    let wining_begin_idx = line.find(':').unwrap();
    let my_numbers_begin_idx = line.find('|').unwrap();

    let mut winning: Vec<u32> = Vec::new();
    let mut my_numbers: Vec<u32> = Vec::new();

    for i in (wining_begin_idx + 1..my_numbers_begin_idx - 1).step_by(3) {
        // println!("{} {}", i, line);
        let num: u32 = line[i..i + 3].trim().parse().unwrap();
        // println!("{}", num);
        winning.push(num);
    }
    for i in (my_numbers_begin_idx + 1..line.len() - 1).step_by(3) {
        // println!("dd{} {}", i, line);
        let num: u32 = line[i..i + 3].trim().parse().unwrap();
        my_numbers.push(num);
    }

    let mut win_count = 0;

    for winning_number in &winning {
        for my_number in &my_numbers {
            if winning_number == my_number {
                win_count += 1;
            }
        }
    }

    if win_count > 0 {
        let base: u32 = 2;
        return base.pow(win_count - 1);
    } else {
        return 0;
    }
}

fn scratch_line_result_count(line: &str) -> usize {
    let wining_begin_idx = line.find(':').unwrap();
    let my_numbers_begin_idx = line.find('|').unwrap();

    let mut winning: Vec<u32> = Vec::new();
    let mut my_numbers: Vec<u32> = Vec::new();

    for i in (wining_begin_idx + 1..my_numbers_begin_idx - 1).step_by(3) {
        // println!("{} {}", i, line);
        let num: u32 = line[i..i + 3].trim().parse().unwrap();
        // println!("{}", num);
        winning.push(num);
    }
    for i in (my_numbers_begin_idx + 1..line.len() - 1).step_by(3) {
        // println!("dd{} {}", i, line);
        let num: u32 = line[i..i + 3].trim().parse().unwrap();
        my_numbers.push(num);
    }

    let mut win_count = 0;

    for winning_number in &winning {
        for my_number in &my_numbers {
            if winning_number == my_number {
                win_count += 1;
            }
        }
    }

    return win_count as usize;
}

fn part2(input_file: &str) -> u32 {
    let mut limit = 0;
    if let Ok(lines) = read_lines(input_file) {
        let mut card_count: Vec<u32> = vec![1; 1000];
        for (i, line) in lines.enumerate() {
            // println!("{}", card_count[i]);
            limit = i;
            for j in i + 1..i + scratch_line_result_count(&line.unwrap()) + 1 {
                card_count[j] += card_count[i];
            }
        }

        return card_count[..limit + 1].into_iter().sum();
    }
    panic!("file not found");
}

fn part1(input_file: &str) -> u32 {
    if let Ok(lines) = read_lines(input_file) {
        return lines
            .map(|x| scratch_line_result(&x.unwrap()))
            .collect::<Vec<u32>>()
            .into_iter()
            .sum();
    }
    panic!("file not found");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(part1("Inputs/Day4/example"), 13);
    }
    #[test]
    fn test_example_part2() {
        assert_eq!(part2("Inputs/Day4/example"), 30);
    }

}

fn main() {
    println!("{}", part1("Inputs/Day4/input"));
    println!("{}", part2("Inputs/Day4/input"));

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
