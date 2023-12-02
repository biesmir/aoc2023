use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use atoi::atoi;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
enum Color{
    Red,
    Green,
    Blue,
}

fn game_valid(line: &str, limit: &HashMap<Color, u16>)->u16{
    let mut game: HashMap<Color, u16> = HashMap::from([(Color::Red, 0), (Color::Green, 0), (Color::Blue, 0)]);
    let reds: Vec<_> = line.match_indices("red").collect();
    let greens: Vec<_> = line.match_indices("green").collect();
    let blues: Vec<_> = line.match_indices("blue").collect();

    let game_id = atoi::<u16>(line[5..].as_bytes()).unwrap();

    for red_idx in reds{
        let color_count = line[red_idx.0-3..red_idx.0].trim().parse::<u16>().unwrap();
        let old_count = game.get(&Color::Red).unwrap();
        if *old_count < color_count{
           game.insert(Color::Red, color_count);
        }
    }

    for blue_idx in blues{
        let color_count = line[blue_idx.0-3..blue_idx.0].trim().parse::<u16>().unwrap();
        let old_count = game.get(&Color::Blue).unwrap();
        if *old_count < color_count{
           game.insert(Color::Blue, color_count);
        }
    }

        for green_idx in greens{
        let color_count = line[green_idx.0-3..green_idx.0].trim().parse::<u16>().unwrap();
        let old_count = game.get(&Color::Green).unwrap();
        if *old_count < color_count{
           game.insert(Color::Green, color_count);
        }
    }

    for color in Color::iter(){
        let color_count = game.get(&color).unwrap();
        let color_limit = limit.get(&color).unwrap();
        if color_count > color_limit{
            return 0;
        }
    }

    return game_id;
}

fn power_of_game(line: &str)->u32{
    let mut game: HashMap<Color, u32> = HashMap::from([(Color::Red, 0), (Color::Green, 0), (Color::Blue, 0)]);
    let reds: Vec<_> = line.match_indices("red").collect();
    let greens: Vec<_> = line.match_indices("green").collect();
    let blues: Vec<_> = line.match_indices("blue").collect();


    for red_idx in reds{
        let color_count = line[red_idx.0-3..red_idx.0].trim().parse::<u32>().unwrap();
        let old_count = game.get(&Color::Red).unwrap();
        if *old_count < color_count{
           game.insert(Color::Red, color_count);
        }
    }

    for blue_idx in blues{
        let color_count = line[blue_idx.0-3..blue_idx.0].trim().parse::<u32>().unwrap();
        let old_count = game.get(&Color::Blue).unwrap();
        if *old_count < color_count{
           game.insert(Color::Blue, color_count);
        }
    }

        for green_idx in greens{
        let color_count = line[green_idx.0-3..green_idx.0].trim().parse::<u32>().unwrap();
        let old_count = game.get(&Color::Green).unwrap();
        if *old_count < color_count{
           game.insert(Color::Green, color_count);
        }
    }

    return game.get(&Color::Red).unwrap() * game.get(&Color::Green).unwrap() * game.get(&Color::Blue).unwrap();
}


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
    // println!("{}", ret);
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

    let mut idx1 = 10000;
    let mut idx2 = 0;

    if numbers.len() == 1{
        idx1 = numbers_idx[0];
        digit1 = numbers[0];
        idx2 = numbers_idx[0];
        digit2 = numbers[0];
    }
    if numbers.len() > 1{
        idx1 = numbers_idx[0];
        idx2 = numbers_idx[numbers_idx.len()-1];
        digit1 = numbers[0];
        digit2 = numbers[numbers_idx.len()-1];
    }
    let number_strings = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        for (pos, number_str) in number_strings.iter().enumerate(){
            let v: Vec<_> = line.match_indices(number_str).collect();
            if v.len() > 0 {
                let idx_low = v[0].0;
                let idx_high = v[v.len()-1].0;
                if idx_low < idx1{
                    digit1 = pos as u32;
                    idx1 = idx_low;
                }
                if idx_high > idx2{
                    digit2 = pos as u32;
                    idx2 = idx_high;
                }
            }
        }

    // println!("{} {}", idx1, idx2);
    // assert_ne!(idx1, idx2);
    // assert!(idx2>idx1);
    //
    let ret;
    // if idx1 == idx2{
    //     ret = digit1;
    // } else if digit2==0{
    //     ret = digit1;
    // } else{
        ret = 10* digit1 + digit2;
    // }
    // println!("{}", ret);
    ret

}

fn day2_part1(input_file: &str)-> u16{
    let limit: HashMap<Color, u16> = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);
    if let Ok(lines) = read_lines(input_file) {
        return lines.map(|x| game_valid(&x.unwrap(), &limit)).collect::<Vec<u16>>().into_iter().sum()
    }
    panic!("file not found")
}

fn day2_part2(input_file: &str)-> u32{
    if let Ok(lines) = read_lines(input_file) {
        return lines.map(|x| power_of_game(&x.unwrap())).collect::<Vec<u32>>().into_iter().sum()
    }
    panic!("file not found")
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
    println!("{}", part1("Inputs/Day1/input1"));
    println!("{}", part2("Inputs/Day1/input1"));
    println!("{}", day2_part1("Inputs/Day2/input1"));
    println!("{}", day2_part2("Inputs/Day2/input1"));

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

    #[test]
    fn test_line1() {
        assert_eq!(line_num2("twogpfttmhp8two13"), 23);
    }
    #[test]
    fn test_line2() {
        assert_eq!(line_num2("b8"), 88);
    }
    #[test]
    fn test_line3() {
        assert_eq!(line_num2("634jgvbvr"), 64);
    }
    #[test]
    fn test_line4() {
        assert_eq!(line_num2("ninesixmxvdcqgxcmskl115lskkp"), 95);
    }
    #[test]
    fn test_line5() {
        assert_eq!(line_num2("eighteigheightteight"), 88);
    }
    #[test]
    fn test_line6() {
        assert_eq!(line_num2("8888882137888888888"), 88);
    }
    #[test]
    fn test_line7() {
        assert_eq!(line_num2("7one6874"), 74);
    }
    #[test]
    fn test_line8() {
        assert_eq!(line_num2("eighjpiigmdt1"), 11);
    }

    #[test]
    fn test_example_day2() {
        assert_eq!(day2_part1("Inputs/Day2/example"), 8);
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
