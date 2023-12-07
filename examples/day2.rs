use atoi::atoi;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
enum Color {
    Red,
    Green,
    Blue,
}

fn game_valid(line: &str, limit: &HashMap<Color, u16>) -> u16 {
    let mut game: HashMap<Color, u16> =
        HashMap::from([(Color::Red, 0), (Color::Green, 0), (Color::Blue, 0)]);
    let reds: Vec<_> = line.match_indices("red").collect();
    let greens: Vec<_> = line.match_indices("green").collect();
    let blues: Vec<_> = line.match_indices("blue").collect();

    let game_id = atoi::<u16>(line[5..].as_bytes()).unwrap();

    for red_idx in reds {
        let color_count = line[red_idx.0 - 3..red_idx.0]
            .trim()
            .parse::<u16>()
            .unwrap();
        let old_count = game.get(&Color::Red).unwrap();
        if *old_count < color_count {
            game.insert(Color::Red, color_count);
        }
    }

    for blue_idx in blues {
        let color_count = line[blue_idx.0 - 3..blue_idx.0]
            .trim()
            .parse::<u16>()
            .unwrap();
        let old_count = game.get(&Color::Blue).unwrap();
        if *old_count < color_count {
            game.insert(Color::Blue, color_count);
        }
    }

    for green_idx in greens {
        let color_count = line[green_idx.0 - 3..green_idx.0]
            .trim()
            .parse::<u16>()
            .unwrap();
        let old_count = game.get(&Color::Green).unwrap();
        if *old_count < color_count {
            game.insert(Color::Green, color_count);
        }
    }

    for color in Color::iter() {
        let color_count = game.get(&color).unwrap();
        let color_limit = limit.get(&color).unwrap();
        if color_count > color_limit {
            return 0;
        }
    }

    return game_id;
}

fn power_of_game(line: &str) -> u32 {
    let mut game: HashMap<Color, u32> =
        HashMap::from([(Color::Red, 0), (Color::Green, 0), (Color::Blue, 0)]);
    let reds: Vec<_> = line.match_indices("red").collect();
    let greens: Vec<_> = line.match_indices("green").collect();
    let blues: Vec<_> = line.match_indices("blue").collect();

    for red_idx in reds {
        let color_count = line[red_idx.0 - 3..red_idx.0]
            .trim()
            .parse::<u32>()
            .unwrap();
        let old_count = game.get(&Color::Red).unwrap();
        if *old_count < color_count {
            game.insert(Color::Red, color_count);
        }
    }

    for blue_idx in blues {
        let color_count = line[blue_idx.0 - 3..blue_idx.0]
            .trim()
            .parse::<u32>()
            .unwrap();
        let old_count = game.get(&Color::Blue).unwrap();
        if *old_count < color_count {
            game.insert(Color::Blue, color_count);
        }
    }

    for green_idx in greens {
        let color_count = line[green_idx.0 - 3..green_idx.0]
            .trim()
            .parse::<u32>()
            .unwrap();
        let old_count = game.get(&Color::Green).unwrap();
        if *old_count < color_count {
            game.insert(Color::Green, color_count);
        }
    }

    return game.get(&Color::Red).unwrap()
        * game.get(&Color::Green).unwrap()
        * game.get(&Color::Blue).unwrap();
}

fn part1(input_file: &str) -> u16 {
    let limit: HashMap<Color, u16> =
        HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);
    if let Ok(lines) = read_lines(input_file) {
        return lines
            .map(|x| game_valid(&x.unwrap(), &limit))
            .collect::<Vec<u16>>()
            .into_iter()
            .sum();
    }
    panic!("file not found")
}

fn part2(input_file: &str) -> u32 {
    if let Ok(lines) = read_lines(input_file) {
        return lines
            .map(|x| power_of_game(&x.unwrap()))
            .collect::<Vec<u32>>()
            .into_iter()
            .sum();
    }
    panic!("file not found")
}

fn main() {
    println!("{}", part1("Inputs/Day2/input1"));
    println!("{}", part2("Inputs/Day2/input1"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_day2() {
        assert_eq!(day2_part1("Inputs/Day2/example"), 8);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
