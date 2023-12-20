use debug_print::debug_print;
use debug_print::debug_println;
use std::cmp::min;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const STEP_LIMIT: usize = 3;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Node {
    is_vertical: bool,
    x: usize,
    y: usize,
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    fn same_location(&self, other: &Node) -> bool {
        return self.x == other.x && self.y == other.y;
    }

    fn same_location_and_orientation(&self, other: &Node) -> bool {
        return self.x == other.x && self.y == other.y && self.is_vertical == other.is_vertical;
    }

    fn get_neighbours(&self, map: &Vec<Vec<u8>>) -> Vec<Node> {
        let mut neighbours = Vec::new();

        if self.is_vertical {
            let upper_bound = min(self.x + STEP_LIMIT + 1, map[0].len());
            let lower_bound = self.x.saturating_sub(STEP_LIMIT);
            let mut cost_count: usize = 0;
            for i in (self.x + 1)..upper_bound {
                cost_count += map[self.y][i] as usize;
                neighbours.push(Node {
                    is_vertical: !self.is_vertical,
                    x: i,
                    y: self.y,
                    cost: self.cost + cost_count,
                })
            }
            let mut cost_count: usize = 0;
            for i in (lower_bound..self.x).rev() {
                cost_count += map[self.y][i] as usize;
                neighbours.push(Node {
                    is_vertical: !self.is_vertical,
                    x: i,
                    y: self.y,
                    cost: self.cost + cost_count,
                })
            }
        } else {
            let upper_bound = min(self.y + STEP_LIMIT + 1, map.len());
            let lower_bound = self.y.saturating_sub(STEP_LIMIT);
            let mut cost_count: usize = 0;
            for i in (self.y + 1)..upper_bound {
                cost_count += map[i][self.x] as usize;
                neighbours.push(Node {
                    is_vertical: !self.is_vertical,
                    x: self.x,
                    y: i,
                    cost: self.cost + cost_count,
                })
            }
            let mut cost_count: usize = 0;
            for i in (lower_bound..self.y).rev() {
                cost_count += map[i][self.x] as usize;
                neighbours.push(Node {
                    is_vertical: !self.is_vertical,
                    x: self.x,
                    y: i,
                    cost: self.cost + cost_count,
                })
            }
        }

        return neighbours;
    }
}

fn part1(filename: &str) -> usize {
    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut to_visit = BinaryHeap::new();
    let mut visited: Vec<Node> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let digits: Vec<u8> = line
                .unwrap()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as u8)
                .collect();
            map.push(digits);
        }
        let map = map;
        let goal = Node {
            x: map[0].len() - 1,
            y: map.len() - 1,
            is_vertical: false,
            cost: 2137,
        };
        to_visit.push(Reverse(Node {
            x: 0,
            y: 0,
            cost: 0,
            is_vertical: true,
        }));
        to_visit.push(Reverse(Node {
            x: 0,
            y: 0,
            cost: 0,
            is_vertical: false,
        }));
        visited.push(Node {
            x: 0,
            y: 0,
            cost: 0,
            is_vertical: true,
        });
        visited.push(Node {
            x: 0,
            y: 0,
            cost: 0,
            is_vertical: false,
        });
        loop {
            // for node in &to_visit{
            //     debug_println!("candidate {} {} {}", node.y, node.x, node.cost);
            // }
            if let Some(current_node) = to_visit.pop() {
                let current_node = current_node.0;
                debug_println!(
                    "visiting {} {} at cost {} ",
                    current_node.y,
                    current_node.x,
                    current_node.cost
                );
                if goal.same_location(&current_node) {
                    return current_node.cost;
                }
                for node in current_node.get_neighbours(&map) {
                    let mut found = false;

                    for v_node in visited.iter_mut() {
                        if node.same_location_and_orientation(v_node) {
                            found = true;
                            if v_node.cost > node.cost {
                                to_visit.push(Reverse(node.clone()));
                                v_node.cost = node.cost;
                            }
                        }
                    }
                    if !found {
                        to_visit.push(Reverse(node));
                        visited.push(node);
                    }
                }
            } else {
                panic!();
            }
        }
    }

    panic!();
}

const ULTRA_UPPER_STEP_LIMIT: usize = 10;
const ULTRA_LOWER_STEP_LIMIT: usize = 4;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct NodeUltra {
    is_vertical: bool,
    x: usize,
    y: usize,
    cost: usize,
}

impl Ord for NodeUltra {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for NodeUltra {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl NodeUltra {
    fn same_location(&self, other: &NodeUltra) -> bool {
        return self.x == other.x && self.y == other.y;
    }

    fn same_location_and_orientation(&self, other: &NodeUltra) -> bool {
        return self.x == other.x && self.y == other.y && self.is_vertical == other.is_vertical;
    }

    fn get_neighbours(&self, map: &Vec<Vec<u8>>) -> Vec<NodeUltra> {
        let mut neighbours = Vec::new();

        if self.is_vertical {
            let up_up_bound = min(self.x + ULTRA_UPPER_STEP_LIMIT + 1, map[0].len());
            let up_down_bound = min(self.x + ULTRA_LOWER_STEP_LIMIT, map[0].len());
            let down_down_bound = self.x.saturating_sub(ULTRA_UPPER_STEP_LIMIT);
            let down_up_bound = self.x.saturating_sub(ULTRA_LOWER_STEP_LIMIT-1);

            let mut cost_count: usize = 0;
            for i in self.x + 1..up_down_bound {
                cost_count += map[self.y][i] as usize;
            }
            debug_println!("cost {}", cost_count);
            for i in up_down_bound..up_up_bound {
                cost_count += map[self.y][i] as usize;
                debug_println!("+ {}", map[self.y][i]);
                neighbours.push(NodeUltra {
                    is_vertical: !self.is_vertical,
                    x: i,
                    y: self.y,
                    cost: self.cost + cost_count,
                })
            }
            let mut cost_count: usize = 0;
            for i in (down_up_bound..self.x).rev() {
                cost_count += map[self.y][i] as usize;
            }

            for i in (down_down_bound..down_up_bound).rev() {
                cost_count += map[self.y][i] as usize;
                neighbours.push(NodeUltra {
                    is_vertical: !self.is_vertical,
                    x: i,
                    y: self.y,
                    cost: self.cost + cost_count,
                })
            }
        } else {
            let up_up_bound = min(self.y + ULTRA_UPPER_STEP_LIMIT + 1, map.len());
            let up_down_bound = min(self.y + ULTRA_LOWER_STEP_LIMIT, map.len());
            let down_down_bound = self.y.saturating_sub(ULTRA_UPPER_STEP_LIMIT);
            let down_up_bound = self.y.saturating_sub(ULTRA_LOWER_STEP_LIMIT-1);
            let mut cost_count: usize = 0;
            for i in self.y + 1..up_down_bound {
                cost_count += map[i][self.x] as usize;
            }

            for i in up_down_bound..up_up_bound {
                cost_count += map[i][self.x] as usize;
                neighbours.push(NodeUltra {
                    is_vertical: !self.is_vertical,
                    x: self.x,
                    y: i,
                    cost: self.cost + cost_count,
                })
            }
            let mut cost_count: usize = 0;
            for i in (down_up_bound..self.y).rev() {
                cost_count += map[i][self.x] as usize;
            }

            for i in (down_down_bound..down_up_bound).rev() {
                cost_count += map[i][self.x] as usize;
                neighbours.push(NodeUltra {
                    is_vertical: !self.is_vertical,
                    x: self.x,
                    y: i,
                    cost: self.cost + cost_count,
                })
            }
        }

        return neighbours;
    }
}

fn part2(filename: &str) -> usize {
    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut to_visit = BinaryHeap::new();
    let mut visited: Vec<NodeUltra> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let digits: Vec<u8> = line
                .unwrap()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as u8)
                .collect();
            map.push(digits);
        }
        let map = map;
        let goal = NodeUltra {
            x: map[0].len() - 1,
            y: map.len() - 1,
            is_vertical: false,
            cost: 2137,
        };
        to_visit.push(Reverse(NodeUltra {
            x: 0,
            y: 0,
            cost: 0,
            is_vertical: true,
        }));
        to_visit.push(Reverse(NodeUltra {
            x: 0,
            y: 0,
            cost: 0,
            is_vertical: false,
        }));
        visited.push(NodeUltra {
            x: 0,
            y: 0,
            cost: 0,
            is_vertical: true,
        });
        visited.push(NodeUltra {
            x: 0,
            y: 0,
            cost: 0,
            is_vertical: false,
        });
        loop {
            // for node in &to_visit{
            //     debug_println!("candidate {} {} {}", node.y, node.x, node.cost);
            // }
            if let Some(current_node) = to_visit.pop() {
                let current_node = current_node.0;
                debug_println!(
                    "visiting {} {} at cost {} ",
                    current_node.y,
                    current_node.x,
                    current_node.cost
                );
                if goal.same_location(&current_node) {
                    return current_node.cost;
                }
                for node in current_node.get_neighbours(&map) {
                    let mut found = false;

                    for v_node in visited.iter_mut() {
                        if node.same_location_and_orientation(v_node) {
                            found = true;
                            if v_node.cost > node.cost {
                                to_visit.push(Reverse(node.clone()));
                                v_node.cost = node.cost;
                            }
                        }
                    }
                    if !found {
                        to_visit.push(Reverse(node));
                        visited.push(node);
                    }
                }
            } else {
                panic!();
            }
        }
    }

    panic!();
}

fn main() {
    let args: Vec<u8> = env::args()
        .skip(1)
        .map(|arg| arg.parse::<u8>().expect("Invalid arguments"))
        .collect();

    if args.len() > 0 {
        for arg in args {
            match arg {
                1 => println!("{}", part1("Inputs/Day17/input")),
                2 => println!("{}", part2("Inputs/Day17/input")),
                _ => panic!("Invalid part"),
            }
        }
    } else {
        println!("{}", part1("Inputs/Day17/example"));
        println!("{}", part2("Inputs/Day17/input"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1("Inputs/Day17/example"), 102);
    }
    #[test]
    fn test_part2_example1() {
        assert_eq!(part2("Inputs/Day17/example"), 94);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn print_map(map: &Vec<Vec<char>>) {
    debug_println!("");
    for line in map {
        for c in line {
            debug_print!("{}", c);
        }
        debug_println!("");
    }
}
