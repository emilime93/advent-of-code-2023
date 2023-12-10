use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");

    let steps = steps_to_zzz(input);

    println!("{steps}");
}

fn steps_to_zzz(input: &str) -> u32 {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();

    let instructions: Vec<Instruction> = instructions.chars().into_iter()
        .map(|x| x.to_string().parse::<Instruction>().unwrap())
        .collect();

    let nodes_map: HashMap<Node, (Node, Node)> = create_map(nodes);

    let mut current_node = &Node {name: String::from("AAA")};
    let end_node = &Node { name: String::from("ZZZ") };
    let mut steps: u32 = 0;
    let max_steps = instructions.len() as u32;
    let mut current_instruction = &instructions[0];
    while current_node != end_node {
        let (left, right) = nodes_map.get(&current_node)
            .expect(&*format!("Node not in map: {:?}", current_node));
        current_node = match current_instruction {
            Instruction::Left => {
                left
            }
            Instruction::Right => {
                right
            }
        };
        steps += 1;

        let next_instruction_index = steps % max_steps;
        current_instruction = &instructions[next_instruction_index as usize];
    }

    steps
}

fn create_map(nodes_input: &str) -> HashMap<Node, (Node, Node)> {
    let nodes_list: Vec<(Node, Node, Node)> = nodes_input.lines()
        .map(parse_node_line)
        .collect();
    let mut map = HashMap::new();
    for nodes in nodes_list {
        map.insert(nodes.0, (nodes.1, nodes.2));
    }
    map
}

fn parse_node_line(line: &str) -> (Node, Node, Node) {
    let (current_node, rest) = line.split_once('=').
        expect("Could not split node line on '='");

    let rest = rest.trim()
        .replace('(', "")
        .replace(')', "");
    let (left_node, right_node) = rest
        .split_once(',')
        .expect("Could not parse (left, right) nodes");

    let current_node = Node { name: String::from(current_node.trim()) };
    let left_node = Node { name: String::from(left_node.trim()) };
    let right_node = Node { name: String::from(right_node.trim()) };

    (current_node, left_node, right_node)
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "L" => Ok(Instruction::Left),
            "R" => Ok(Instruction::Right),
            _ => panic!("Unparseable instruction")
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Node {
    name: String,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_input() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(6, steps_to_zzz(input));
    }
}