use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn get_nums(line: &str) -> Vec<u64> {
    let mut list: Vec<u64> = Vec::new();
    for number_str in line[9..].split_whitespace() {
        if let Ok(number) = number_str.parse::<u64>() {
            list.push(number);
        }
    }
    return list;
}

fn calculate_no_of_ways(time: u64, dist: u64) -> u64 {
    let mut count = 0;
    for i in 0..time {
        if i * (time - i) > dist {
            count += 1;
        }
    }
    return count;
}

fn part1(input_file: &str) -> u64 {
    if let Ok(mut lines) = read_lines(input_file) {
        let times = get_nums(&lines.next().unwrap().unwrap());
        let distances = get_nums(&lines.next().unwrap().unwrap());
        let mut solution =1;
        for i in 0..times.len(){
            solution *= calculate_no_of_ways(times[i], distances[i]);
        }
        return solution;
    }
    2137
}

fn main() {
    println!("solution {}", part1("Inputs/Day6/input"));
    println!("solution {}", calculate_no_of_ways(48938466, 261119210191063));

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
