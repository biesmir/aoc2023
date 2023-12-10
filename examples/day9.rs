use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn make_next_sequence(old: &Vec<i64>) -> Vec<i64> {
    let mut new: Vec<i64> = Vec::new();

    for i in 0..old.len() - 1 {
        new.push(old[i + 1] - old[i]);
    }
    return new;
}

fn extrapolate_line(line: &mut Vec<i64>, lower_line: &Vec<i64>) {
    line.push(line[line.len() - 1] + lower_line[lower_line.len() - 1])
}

fn read_line_to_vec(line: &str) -> Vec<i64> {
    let new: Vec<i64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    return new;
}

fn part1(filename: &str) -> i64 {
    let mut solution = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let mut numbers_lines: Vec<Vec<i64>> = Vec::new();
            let line_nums = read_line_to_vec(&line.unwrap());
            numbers_lines.push(line_nums);
            loop {
                let next = make_next_sequence(&numbers_lines[numbers_lines.len() - 1]);
                numbers_lines.push(next);
                if numbers_lines[numbers_lines.len() - 1].iter().sum::<i64>() == 0 {
                    break;
                }
            }

            println!("numbers_lines.len() {}", numbers_lines.len());
            for i in (0..numbers_lines.len()-1).rev(){
                let tmp = numbers_lines[i+1][numbers_lines[i+1].len() - 1] + numbers_lines[i][numbers_lines[i].len() - 1];
                numbers_lines[i].push(tmp);
                println!("doopa");
            }

            let tmp = numbers_lines[0][numbers_lines[0].len()-1];
            println!("tmp {}", tmp);
            solution += tmp;
        }
        return solution;
    }
    panic!();
}

fn part2(filename: &str) -> i64 {
    let mut solution = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let mut numbers_lines: Vec<Vec<i64>> = Vec::new();
            let line_nums = read_line_to_vec(&line.unwrap());
            numbers_lines.push(line_nums);
            loop {
                let next = make_next_sequence(&numbers_lines[numbers_lines.len() - 1]);
                numbers_lines.push(next);
                if numbers_lines[numbers_lines.len() - 1].iter().sum::<i64>() == 0 {
                    break;
                }
            }

            println!("numbers_lines.len() {}", numbers_lines.len());
            for i in (0..numbers_lines.len()-1).rev(){
                let tmp = numbers_lines[i][0] - numbers_lines[i+1][0];
                numbers_lines[i].insert(0, tmp);
            }

            let tmp = numbers_lines[0][0];
            println!("tmp {}", tmp);
            solution += tmp;
        }
        return solution;
    }
    panic!();
}


fn main() {
    println!("{}", part1("Inputs/Day9/input"));
    println!("{}", part2("Inputs/Day9/input"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1("Inputs/Day9/example"), 114);
    }
    #[test]
    fn test_part2_example1() {
        assert_eq!(part2("Inputs/Day9/example"), 2);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
