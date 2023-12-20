use debug_print::debug_print;
use debug_print::debug_println;
use std::cmp::min;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

enum WorkflowNode {
    Ordinary(Node),
    Terminator(String),
}

struct Node {
    param_checked: char,
    gt: bool,
    val: usize,
    next: String,
}

struct Workflow {
    nodes: Vec<WorkflowNode>,
}

impl Node {
    fn check(&self, part: &Part) -> Option<String> {
        match self.param_checked {
            'x' => {
                if (self.val > part.x) ^ self.gt {
                    return Some(self.next.clone());
                }
            }
            'm' => {
                if (self.val > part.m) ^ self.gt {
                    return Some(self.next.clone());
                }
            }
            'a' => {
                if (self.val > part.a) ^ self.gt {
                    return Some(self.next.clone());
                }
            }
            's' => {
                if (self.val > part.s) ^ self.gt {
                    return Some(self.next.clone());
                }
            }
            _ => panic!(),
        }
        return None;
    }
}

// impl Workflow{
//     fn

// }
fn part1(filename: &str) -> usize {
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    if let Ok(mut lines) = read_lines(filename) {
        loop {
            let line = lines.next().unwrap().unwrap();
            if line.len() < 2 {
                break;
            }
            let (prefix, sufix) = line.split_once('{').unwrap();

            let mut workflow = Workflow { nodes: Vec::new() };

            let mut sufix = sufix.to_string();
            sufix.pop();

            let mut nodes: Vec<&str> = sufix.split(',').collect();
            let terminator = WorkflowNode::Terminator(nodes.pop().unwrap().to_string());

            for node in nodes {
                if let Some((property, value_next)) = node.split_once('>') {
                    let (value, next) = value_next.split_once(':').unwrap();
                    workflow.nodes.push(WorkflowNode::Ordinary(Node {
                        param_checked: property.chars().nth(0).unwrap(),
                        gt: true,
                        val: value.parse::<usize>().unwrap(),
                        next: next.to_string(),
                    }));
                } else {
                    let (property, value_next) = node.split_once('<').unwrap();
                    let (value, next) = value_next.split_once(':').unwrap();
                    workflow.nodes.push(WorkflowNode::Ordinary(Node {
                        param_checked: property.chars().nth(0).unwrap(),
                        gt: false,
                        val: value.parse::<usize>().unwrap(),
                        next: next.to_string(),
                    }));
                }
            }

            workflow.nodes.push(terminator);

            workflows.insert(prefix.to_string(), workflow);
        }
        let mut res = 0;
        for line in lines {
            let properties: Vec<usize> = line
                .unwrap()
                .split(|c| c == '=' || c == ',' || c == '}')
                .filter(|&s| !s.is_empty())
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();
            let part = Part {
                x: properties[0],
                m: properties[1],
                a: properties[2],
                s: properties[3],
            };
            let mut workflow = workflows.get("in").unwrap();
            'outer: loop {
                for op in &workflow.nodes {
                    match op {
                        WorkflowNode::Terminator(term) => match term.as_str() {
                            "A" => {
                                res += part.x + part.m + part.a + part.s;
                                break 'outer;
                            }
                            "R" => break 'outer,
                            _ => {
                                workflow = workflows.get(term).unwrap();
                                break;
                            }
                        },
                        WorkflowNode::Ordinary(ord) => {
                            if let Some(next) = ord.check(&part) {
                                match next.as_str() {
                                    "A" => {
                                        res += part.x + part.m + part.a + part.s;
                                        break 'outer;
                                    }
                                    "R" => break 'outer,
                                    _ => {
                                        workflow = workflows.get(&next).unwrap();
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return res;
    }
    panic!()
}

fn part2(filename: &str) -> usize {
    todo!()
}

fn main() {
    let args: Vec<u8> = env::args()
        .skip(1)
        .map(|arg| arg.parse::<u8>().expect("Invalid arguments"))
        .collect();

    if args.len() > 0 {
        for arg in args {
            match arg {
                1 => println!("{}", part1("Inputs/Day19/input")),
                2 => println!("{}", part2("Inputs/Day19/input")),
                _ => panic!("Invalid part"),
            }
        }
    } else {
        println!("{}", part1("Inputs/Day19/input"));
        println!("{}", part2("Inputs/Day19/input"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1("Inputs/Day19/example"), 19114);
    }
    #[test]
    fn test_part2_example1() {
        assert_eq!(part2("Inputs/Day19/example"), 167409079868000);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
