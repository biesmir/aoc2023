use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct SeedDetails {
    seed: u64,
    soil: u64,
    fert: u64,
    water: u64,
    light: u64,
    temp: u64,
    humid: u64,
    location: u64,
}

fn get_seeds_ranged(line: &str) -> Vec<(u64, u64)> {
    let mut seed_list: Vec<(u64, u64)> = Vec::new();
    for number_strs in line[6..]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .chunks(2)
    {
        seed_list.push((
            number_strs[0].parse::<u64>().unwrap(),
            number_strs[0].parse::<u64>().unwrap() + number_strs[1].parse::<u64>().unwrap() - 1,
        ));
    }

    return seed_list;
}

fn map_range_to_next(line: &str, old_list: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut next_list: Vec<(u64, u64)> = Vec::new();
    let numbers: Vec<u64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for i in 0..old_list.len() {
        // println!("{}", line);
        let old = old_list[i];
        if old.1 == 0 {
            continue;
        }
        if old.1 < numbers[1] {
            continue;
        }
        if old.0 > numbers[1] + numbers[2] {
            continue;
        }
        println!("{} {} turns into", old.0, old.1);
        //1o
        if old.0 >= numbers[1] && old.1 < numbers[1] + numbers[2] {
            let low_offset = old.0 - numbers[1];
            let high_offset = old.1 - numbers[1];

            let new = (numbers[0] + low_offset, high_offset + numbers[0]);
            println!("{} {} {} {}", old.0, old.1, new.0, new.1);
            assert!(new.0 <= new.1);
            old_list[i] = (0, 0);
            next_list.push(new);
            continue;
        }
        //2o
        if old.0 <= numbers[1] && old.1 <= numbers[1] + numbers[2] && old.1 >= numbers[1] {
            let high_offset = old.1 - numbers[1];
            let new = (numbers[0], high_offset + numbers[0]-1);
            old_list[i].1 = numbers[1];
            next_list.push(new);
            continue;
        }
        //3o
        if old.0 >= numbers[1] && old.1 >= numbers[1] + numbers[2] {
            let new = (old.0 - numbers[1] + numbers[0], numbers[0] + numbers[2] );
            old_list[i].0 = numbers[1] + numbers[2];
            next_list.push(new);
            continue;
        }
        //4o
        if old.0 <= numbers[1] && old.1 > numbers[1] + numbers[2] {
            let new = (numbers[0], numbers[0] + numbers[2]);
            let leftover = (numbers[1] + numbers[2], old.1);
            old_list[i].1 = numbers[1];
            if leftover.1 > 0{
                old_list.push(leftover);
            }
            next_list.push(new);
            continue;
        }
        panic!("I fucked up");
    }
    for elem in &next_list {
        assert!(elem.0 <= elem.1);
        if elem.0 != 0 {
            // assert_ne!(elem.0, elem.1);
        }
    }

    return next_list;
}

fn get_seeds(line: &str) -> Vec<SeedDetails> {
    let mut seed_list: Vec<SeedDetails> = Vec::new();
    for number_str in line[6..].split_whitespace() {
        if let Ok(number) = number_str.parse::<u64>() {
            println!("seed {}", number);
            let seed = SeedDetails {
                seed: number,
                soil: number,
                fert: number,
                water: number,
                light: number,
                temp: number,
                humid: number,
                location: number,
            };
            seed_list.push(seed);
        } else {
            panic!("error parsing seeds");
        }
    }
    return seed_list;
}

fn get_soil(line: &str, seeds: &mut Vec<SeedDetails>) {
    let numbers: Vec<u64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(numbers.len(), 3);
    for seed in &mut *seeds {
        if seed.seed >= numbers[1] && seed.seed < numbers[1] + numbers[2] {
            let num = seed.seed - numbers[1] + numbers[0];
            seed.soil = num;
            seed.fert = num;
            seed.water = num;
            seed.light = num;
            seed.temp = num;
            seed.humid = num;
            seed.location = num;
            println!("seed soil {}", num);
        }
    }
}

fn get_fert(line: &str, seeds: &mut Vec<SeedDetails>) {
    let numbers: Vec<u64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(numbers.len(), 3);

    for seed in &mut *seeds {
        if seed.soil >= numbers[1] && seed.soil < numbers[1] + numbers[2] {
            let num = seed.soil - numbers[1] + numbers[0];
            seed.fert = num;
            seed.water = num;
            seed.light = num;
            seed.temp = num;
            seed.humid = num;
            seed.location = num;
            println!("seed fertilizer {}", num);
        }
    }
}

fn get_water(line: &str, seeds: &mut Vec<SeedDetails>) {
    let numbers: Vec<u64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(numbers.len(), 3);
    for seed in &mut *seeds {
        if seed.fert >= numbers[1] && seed.fert < numbers[1] + numbers[2] {
            let num = seed.fert - numbers[1] + numbers[0];
            seed.water = num;
            seed.light = num;
            seed.temp = num;
            seed.humid = num;
            seed.location = num;
            println!("seed water {}", num);
        }
    }
}

fn get_light(line: &str, seeds: &mut Vec<SeedDetails>) {
    let numbers: Vec<u64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(numbers.len(), 3);
    for seed in &mut *seeds {
        if seed.water >= numbers[1] && seed.water < numbers[1] + numbers[2] {
            let num = seed.water - numbers[1] + numbers[0];
            seed.light = num;
            seed.temp = num;
            seed.humid = num;
            seed.location = num;
            println!("seed light {}", num);
        }
    }
}

fn get_temp(line: &str, seeds: &mut Vec<SeedDetails>) {
    let numbers: Vec<u64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(numbers.len(), 3);
    for seed in &mut *seeds {
        if seed.light >= numbers[1] && seed.light < numbers[1] + numbers[2] {
            let num = seed.light - numbers[1] + numbers[0];
            seed.temp = num;
            seed.humid = num;
            seed.location = num;
            println!("seed tempereture {}", num);
        }
    }
}

fn get_humid(line: &str, seeds: &mut Vec<SeedDetails>) {
    let numbers: Vec<u64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(numbers.len(), 3);
    for seed in &mut *seeds {
        if seed.temp >= numbers[1] && seed.temp < numbers[1] + numbers[2] {
            let num = seed.temp - numbers[1] + numbers[0];
            seed.humid = num;
            seed.location = num;
            println!("seed humidity {}", num);
        }
    }
}

fn get_location(line: &str, seeds: &mut Vec<SeedDetails>) {
    let numbers: Vec<u64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(numbers.len(), 3);
    for seed in &mut *seeds {
        if seed.humid >= numbers[1] && seed.humid < numbers[1] + numbers[2] {
            let num = seed.humid - numbers[1] + numbers[0];

            seed.location = num;
            println!("seed location {}", num);
        }
    }
}

fn part1(input_file: &str) -> u64 {
    if let Ok(mut lines) = read_lines(input_file) {
        let mut seed_list = get_seeds(&lines.next().unwrap().unwrap());
        lines.next();
        lines.next();
        loop {
            let line = &lines.next().unwrap().unwrap();
            if line.len() > 1 {
                get_soil(line, &mut seed_list);
            } else {
                break;
            }
        }
        lines.next();
        loop {
            let line = &lines.next().unwrap().unwrap();
            if line.len() > 1 {
                get_fert(line, &mut seed_list);
            } else {
                break;
            }
        }
        lines.next();
        loop {
            let line = &lines.next().unwrap().unwrap();
            if line.len() > 1 {
                get_water(line, &mut seed_list);
            } else {
                break;
            }
        }
        lines.next();
        loop {
            let line = &lines.next().unwrap().unwrap();
            if line.len() > 1 {
                get_light(line, &mut seed_list);
            } else {
                break;
            }
        }
        lines.next();
        loop {
            let line = &lines.next().unwrap().unwrap();
            if line.len() > 1 {
                get_temp(line, &mut seed_list);
            } else {
                break;
            }
        }
        lines.next();
        loop {
            let line = &lines.next().unwrap().unwrap();
            if line.len() > 1 {
                get_humid(line, &mut seed_list);
            } else {
                break;
            }
        }
        lines.next();
        loop {
            match lines.next() {
                Some(Ok(line)) => {
                    if line.len() > 1 {
                        get_location(&line, &mut seed_list);
                    } else {
                        break;
                    }
                }
                Some(Err(_)) => break,
                None => break,
            }
        }
        // let mut min_location: u64 = 0xFFFFFFFFFFFFFFFF;
        // for seed in seed_list {
        //     let location = seed.location;

        //     if min_location > location {
        //         min_location = location;
        //     }
        // }
        return seed_list
            .into_iter()
            .map(|x| x.location)
            .collect::<Vec<u64>>()
            .into_iter()
            .min()
            .unwrap();
    }

    0
}

fn part2(input_file: &str) -> u64 {
    if let Ok(mut lines) = read_lines(input_file) {
        let mut old_list = get_seeds_ranged(&lines.next().unwrap().unwrap());
        lines.next();
        lines.next();
        // println!(
        //     "old list {} {} {} {}",
        //     old_list[0].0, old_list[0].1, old_list[1].0, old_list[1].1
        // );
        for _ in 0..7 {
            let mut new_list: Vec<(u64, u64)> = Vec::new();
            println!("");

            loop {
                match lines.next() {
                    Some(Ok(line)) => {
                        // let line = &line.unwrap();
                        //                        println!("{}", line);
                        if line.len() > 1 {
                            let mut res = map_range_to_next(&line, &mut old_list);
                            // println!("len {}", res.len());
                            new_list.append(&mut res);
                            // println!("len new {}",new_list.len());
                        } else {
                            // println!("len new {}",new_list.len());

                            for old in old_list {
                                // println!("old {} {}", old.0, old.1);
                                // println!("new list {} {}", new_list[0].0, new_list[0].1);
                                // assert_ne!(old.0, 1284963);
                                if old.0 != 0 && 0 != old.1 {
                                    new_list.push(old);
                                } else{
                                    println!("discarding {} {}", old.0, old.1)
                                }
                            }
                            old_list = new_list;
                            break;
                        }
                    }
                    Some(Err(_)) => break,
                    None => break,
                }
            }
            lines.next();
            println!("len {}", old_list.len());
            for elem in &old_list {
                println!("{:10} {:10}", elem.0, elem.1);
            }
        }
        old_list.retain(|&x| x.0 != 0 && x.1 != 0);
        println!("len {}", old_list.len());
        for elem in &old_list {
            println!("{:10} {:10}", elem.0, elem.1);
        }

        return old_list
            .into_iter()
            .map(|x| x.0)
            .collect::<Vec<u64>>()
            .into_iter()
            .min()
            .unwrap();
    }
    2137
}

fn main() {
    println!("solution {}", part1("Inputs/Day5/input"));
    println!("solution {}", part2("Inputs/Day5/input")); ////
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(part1("Inputs/Day5/example"), 35);
    }
    #[test]
    fn test_example_part2() {
        assert_eq!(part2("Inputs/Day5/example"), 46);
    }
    #[test]
    fn test_my_example_part2() {
        assert_eq!(part2("Inputs/Day5/example2"), 46);
    }

    #[test]
    fn test_my_example_part1() {
        assert_eq!(part1("Inputs/Day5/example2"), 35);
    }

    #[test]
    fn test_equal_range() {
        let line = "420 2137 69";
        let mut old_list = vec![(2137, 2137 + 69)];
        let new_list = map_range_to_next(line, &mut old_list);
        assert_eq!(new_list[0].0, 420);
        assert_eq!(new_list[0].1, 420 + 68);
        assert_eq!(old_list[0].0, 0);
        assert_eq!(old_list[0].1, 0);
    }

    #[test]
    fn test_no_common() {
        let line = "420 2137 69";
        let mut old_list = vec![(420, 425)];
        let new_list = map_range_to_next(line, &mut old_list);
        assert_eq!(new_list.len(), 0);
        assert_eq!(old_list[0].0, 420);
        assert_eq!(old_list[0].1, 425);
    }
    #[test]
    fn test_no_common2() {
        let line = "420 2137 69";
        let mut old_list = vec![(4200, 4250)];
        let new_list = map_range_to_next(line, &mut old_list);
        assert_eq!(new_list.len(), 0);
        assert_eq!(old_list[0].0, 4200);
        assert_eq!(old_list[0].1, 4250);
    }

    #[test]
    fn test_contained_range() {
        let line = "420 2137 69";
        let mut old_list = vec![(2137 + 5, 2137 + 69 - 5)];
        let new_list = map_range_to_next(line, &mut old_list);
        assert_eq!(new_list[0].0, 425);
        assert_eq!(new_list[0].1, 420 + 69 - 5 - 1);
        assert_eq!(old_list[0].0, 0);
        assert_eq!(old_list[0].1, 0);
    }

    #[test]
    fn test_contained_range_low_eq() {
        let line = "420 2137 69";
        let mut old_list = vec![(2137, 2137 + 69 - 5)];
        let new_list = map_range_to_next(line, &mut old_list);
        assert_eq!(new_list[0].0, 420);
        assert_eq!(new_list[0].1, 420 + 69 - 5 - 1);
        assert_eq!(old_list[0].0, 0);
        assert_eq!(old_list[0].1, 0);
    }
    #[test]
    fn test_contained_range_high_eq() {
        let line = "420 2137 69";
        let mut old_list = vec![(2137 + 5, 2137 + 69)];
        let new_list = map_range_to_next(line, &mut old_list);
        assert_eq!(new_list[0].0, 425);
        assert_eq!(new_list[0].1, 420 + 69 - 1);
        assert_eq!(old_list[0].0, 0);
        assert_eq!(old_list[0].1, 0);
    }
    #[test]
    fn test_contained_range_overlap_high() {
        let line = "420 2137 69";
        let mut old_list = vec![(2000, 2137 + 69 - 5)];
        let new_list = map_range_to_next(line, &mut old_list);
        assert_eq!(new_list[0].0, 420);
        assert_eq!(new_list[0].1, 420 + 68 - 5);
        assert_eq!(old_list[0].0, 2000);
        assert_eq!(old_list[0].1, 2137);
    }
    #[test]
    fn test_contained_range_overlap_low() {
        let line = "420 2137 69";
        let mut old_list = vec![(2137 + 5, 3000)];
        let new_list = map_range_to_next(line, &mut old_list);
        assert_eq!(new_list[0].0, 425);
        assert_eq!(new_list[0].1, 420 + 68);
        assert_eq!(old_list[0].0, 2137 + 69);
        assert_eq!(old_list[0].1, 3000);
    }
    #[test]
    fn test_contained_range_fliped() {
        let line = "420 2137 5";
        let mut old_list = vec![(2000, 3000)];
        let new_list = map_range_to_next(line, &mut old_list);
        assert_eq!(new_list[0].0, 420);
        assert_eq!(new_list[0].1, 424);
        assert_eq!(old_list[0].0, 2000);
        assert_eq!(old_list[0].1, 2137);
        assert_eq!(old_list[1].0, 2137 + 5);
        assert_eq!(old_list[1].1, 3000);
    }
    #[test]
    fn test_equal_range_from_zero() {
        let line = "420 0 5";
        let mut old_list = vec![(0, 5)];
        let new_list = map_range_to_next(line, &mut old_list);
        assert_eq!(new_list[0].0, 420);
        assert_eq!(new_list[0].1, 424);
        assert_eq!(old_list[0].0, 0);
        assert_eq!(old_list[0].1, 0);
    }

    fn test_contained_range_to_zero() {
        let line = "0 420 5";
        let mut old_list = vec![(420, 425)];
        let new_list = map_range_to_next(line, &mut old_list);
        assert_eq!(new_list[0].0, 0);
        assert_eq!(new_list[0].1, 4);
        assert_eq!(old_list[0].0, 0);
        assert_eq!(old_list[0].1, 0);
    }

    fn test_equal_range_no_change() {
        let line = "420 420 5";
        let mut old_list = vec![(420, 425)];
        let new_list = map_range_to_next(line, &mut old_list);
        assert_eq!(new_list[0].0, 420);
        assert_eq!(new_list[0].1, 424);
        assert_eq!(old_list[0].0, 0);
        assert_eq!(old_list[0].1, 0);
    }

    fn test_contained_range_upper_connected() {
        let line = "420 420 5";
        let mut old_list = vec![(420, 425)];
        let new_list = map_range_to_next(line, &mut old_list);
        assert_eq!(new_list[0].0, 0);
        assert_eq!(new_list[0].1, 4);
        assert_eq!(old_list[0].0, 0);
        assert_eq!(old_list[0].1, 0);
    }

    #[test]
    fn test_from_input_1() {
        let line = "67314744 0 262295201";
        let mut old_list = vec![(1284963, 12560465)];
        let new_list = map_range_to_next(line, &mut old_list);
        assert_eq!(new_list[0].0, 1284963 + 67314744);
        assert_eq!(new_list[0].1, 67314744 + 12560465 - 1);
        assert_eq!(old_list[0].0, 0);
        assert_eq!(old_list[0].1, 0);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
