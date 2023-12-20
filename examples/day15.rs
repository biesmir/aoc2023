use debug_print::debug_print;
use debug_print::debug_println;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn hash_string(s: &str) -> u8 {
    let mut solution: u64 = 0;
    for b in s.as_bytes() {
        solution += *b as u64;
        solution *= 17;
        solution %= 256;
    }
    return solution.try_into().unwrap();
}

fn hash_line(line: &str) -> u64 {
    let mut res: u64 = 0;
    for step in line.split(',') {
        res += hash_string(&step) as u64;
    }
    return res;
}

fn the_same_lens(inside: &str, lens: &str) -> bool {
    let (lens_mark, _) = inside.split_once('=').unwrap();
    return lens_mark == lens;
}

fn execute_command<'a>(boxes: &mut Vec<Vec<&'a str>>, cmd: &'a str) {
    match cmd.chars().nth(cmd.len() - 1).unwrap() {
        '-' => {
            let new_lens_mark = &cmd[..cmd.len() - 1];
            let hash: usize = hash_string(new_lens_mark).try_into().unwrap();
            boxes[hash].retain(|x| !the_same_lens(x, new_lens_mark))
        }
        _ => {
            let (new_lens_mark, _) = cmd.split_once('=').unwrap();
            let hash: usize = hash_string(new_lens_mark).try_into().unwrap();
            let mut contains: bool = false;
            for lens in &mut boxes[hash] {
                let (lens_mark, _) = lens.split_once('=').unwrap();
                if lens_mark == new_lens_mark {
                    *lens = cmd;
                    contains = true;
                    break;
                }
            }
            if !contains {
                boxes[hash].push(cmd);
            }
        }
    }
}

fn part1(filename: &str) -> u64 {
    if let Ok(lines) = read_lines(filename) {
        return lines
            .map(|x| hash_line(&x.unwrap()))
            .collect::<Vec<u64>>()
            .into_iter()
            .sum();
    }
    panic!();
}

fn part2(filename: &str) -> usize {
    let mut solution = 0;
    let mut boxes: Vec<Vec<&str>> = vec![Vec::new(); 256];
    if let Ok(mut lines) = read_lines(filename) {
        let line = lines.nth(0).unwrap().unwrap();
        let line_clone = line.clone();
        let commands: Vec<&str> = line_clone.split(',').collect();
        commands
            .into_iter()
            .for_each(|cmd| execute_command(&mut boxes, cmd));

        for (box_idx, bx) in boxes.iter().enumerate() {
            for (lens_idx, cmd) in bx.iter().enumerate() {
                let (_, f_len) = cmd.split_once('=').unwrap();
                solution += (box_idx + 1) * (lens_idx + 1) * f_len.parse::<usize>().unwrap();
                debug_println!("{} * {} * {}", (box_idx + 1), (lens_idx + 1), f_len.parse::<usize>().unwrap());
            }
        }
    }
    return solution;
}

fn main() {
    let args: Vec<u8> = env::args()
        .skip(1)
        .map(|arg| arg.parse::<u8>().expect("Invalid arguments"))
        .collect();

    if args.len() > 0 {
        for arg in args {
            match arg {
                1 => println!("{}", part1("Inputs/Day15/input")),
                2 => println!("{}", part2("Inputs/Day15/input")),
                _ => panic!("Invalid part"),
            }
        }
    } else {
        println!("{}", part1("Inputs/Day15/input"));
        println!("{}", part2("Inputs/Day15/input"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_string() {
        assert_eq!(hash_string("HASH"), 52);
    }

    #[test]
    fn test_hash_string2() {
        assert_eq!(hash_string("rn"), 0);
    }


    #[test]
    fn test_hash_line() {
        assert_eq!(
            hash_line("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );
    }

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1("Inputs/Day15/example"), 1320);
    }
    #[test]
    fn test_part2_example1() {
        assert_eq!(part2("Inputs/Day15/example"), 145);
    }


}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
