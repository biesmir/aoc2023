use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn line_num(line: &str)->u32{
    const RADIX: u32 = 10;
    let mut numbers:Vec<u32> = Vec::new();
    for c in line.chars(){
        match c.to_digit(RADIX){
            Some(num) => numbers.push(num),
            None => (),
        }
    }
    let ret = 10* numbers[0] + numbers[numbers.len()-1];
    println!("{}", ret);
    ret

}

fn line_num2(line: &str)->u32{
    const RADIX: u32 = 10;
    let mut numbers:Vec<u32> = Vec::new();
    let mut numbers_idx:Vec<usize> = Vec::new();
    for (pos, c) in line.chars().enumerate(){
        match c.to_digit(RADIX){
            Some(num) => {numbers.push(num);
            numbers_idx.push(pos)},
            None => (),
        }
    }

    let mut digit1 = 0;
    let mut digit2 = 0;

    let mut idx1=10000;
    let mut idx2=0;

    if numbers.len() > 0{
        let mut idx1=numbers_idx[0];
        let mut idx2=numbers_idx[numbers_idx.len()-1];

    let number_strings = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        for (pos, number_str) in number_strings.iter().enumerate(){
            let idx = line.find(number_str);
            match idx{
                Some(idx) => {
                    if idx <= idx1{
                        digit1 = pos as u32;
                        idx1 = idx;
                    }
                    if idx >= idx2{
                        numbers_idx.push(pos);
                        digit2 = pos as u32;
                        idx2 = idx;
                    }
                },
                None => (),
            }
        }
        if digit1 == 0{ digit1 = numbers[0]; }
        if digit2 == 0{ digit2 = numbers[numbers.len()-1]; }
    }
        else{
            let number_strings = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
            for (pos, number_str) in number_strings.iter().enumerate(){
                let idx = line.find(number_str);
                match idx{
                    Some(idx) => {
                        if idx <= idx1{
                            digit1 = pos as u32;
                            idx1 = idx;
                        }
                        if idx >= idx2{
                            digit2 = pos as u32;
                            idx2 = idx;
                        }
                    },
                    None => (),
                }

            }
        }
    assert_ne!(idx1, idx2);
        let ret = 10* digit1 + digit2;
        println!("{}", ret);
        ret

}


fn part1(input_file: &str) -> u32{
    if let Ok(lines) = read_lines(input_file) {
        return lines.map(|x| line_num(&x.unwrap())).collect::<Vec<u32>>().into_iter().sum()
    }
    panic!("file not found")
}

fn part2(input_file: &str) -> u32{
    if let Ok(lines) = read_lines(input_file) {
        return lines.map(|x| line_num2(&x.unwrap())).collect::<Vec<u32>>().into_iter().sum()
    }
    panic!("file not found")
}


fn main() {
    // println!("{}", part1("Inputs/Day1/input1"));
    println!("{}", part2("Inputs/Day1/input1"));
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(part1("Inputs/Day1/example"), 142);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2("Inputs/Day1/example2"), 281);
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
