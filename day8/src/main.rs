use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct AOCError;
impl std::fmt::Display for AOCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AOCError")
    }
}
impl Error for AOCError {}

fn main() {
    let input_1 = include_str!("input_1.txt");

    println!("---RESULT_1---");
    let result_1 = solve_1(input_1);
    println!("{}", result_1.unwrap());
    // println!("---RESULT_2---");
    // let result_2 = solve_2(input_1);
    // println!("{}", result_2.unwrap());
}

#[derive(Debug, Clone)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

fn solve_1(input: &str) -> Result<u64, Box<dyn Error>> {
    let mut lines = input.lines();
    let directions = lines.next().ok_or(AOCError)?;
    // skip a line
    lines.next().ok_or(AOCError)?;

    let instructions: Vec<_> = lines
        .map(|instruction| {
            let mut instruction = instruction.split('=');
            (instruction.next().unwrap().trim(), {
                let mut instruction = instruction.next().unwrap().trim()[1..9].split(',');
                (
                    instruction.next().unwrap().trim(),
                    instruction.next().unwrap().trim(),
                )
            })
        })
        .collect();

    let nodes_map = create_map(&instructions);

    let directions: Vec<_> = directions.chars().collect();
    let mut curr_node = nodes_map.get(&"AAA".to_string());

    let steps = directions
        .iter()
        .cycle()
        .enumerate()
        .find_map(|(index, direction)| {
            if let Some(ref node) = curr_node {
                if node.name == "ZZZ" {
                    return Some(index as u64);
                }

                if *direction == 'L' {
                    curr_node = nodes_map.get(&node.left.to_string());
                } else {
                    curr_node = nodes_map.get(&node.right.to_string());
                }
                return None;
            };
            panic!("Shouldnt happen")
        });

    steps.ok_or(Box::new(AOCError))
}
fn create_map<'a>(nodes: &'a [(&'a str, (&'a str, &'a str))]) -> HashMap<String, Node<'a>> {
    let mut map: HashMap<String, Node> = HashMap::new();
    // Construct all the base nodes
    for node in nodes.iter() {
        map.insert(
            node.0.to_string(),
            Node {
                name: node.0,
                left: node.1 .0,
                right: node.1 .1,
            },
        );
    }
    map
}
fn solve_2(input: &str) -> Result<u64, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_1() {
        let input = r#"RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)"#;

        assert_eq!(solve_1(input).unwrap(), 2);

        let input_2 = r#"LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)"#;
        assert_eq!(solve_1(input_2).unwrap(), 6);
    }
    #[test]
    fn test_solve_2() {
        let input = r#""#;
        assert_eq!(solve_2(input).unwrap(), todo!());
    }
}
