fn main() {
    let input = include_str!("./input.txt");

    let furthest_length = find_furthest_length(input);

    println!("{furthest_length}");
}

fn find_furthest_length(input: &str) -> u32 {
    let matrix = parse_to_node_matrix(input);
    let round_points = traverse(&matrix);
    (round_points as f64 / 2f64).ceil() as u32
}

fn parse_to_node_matrix(input: &str) -> Vec<Vec<Node>> {
    let mut matrix: Vec<Vec<Node>> = Vec::new();
    for (row_num, line) in input.lines().enumerate() {
        let mut matrix_line = Vec::new();
        for (col_num, c) in line.chars().enumerate() {
            matrix_line.push(parse_to_node(c, Point { row_num, col_num }));
        }
        matrix.push(matrix_line);
    }
    matrix
}

fn traverse(matrix: &Vec<Vec<Node>>) -> u32 {
    let start_node: &Node = find_start_node(matrix);
    let (neighbour_node_1, _) = find_neighbour_nodes(start_node, matrix);
    traverse_and_count(start_node, neighbour_node_1, matrix, 0)
}

fn traverse_and_count(current_node: &Node, target_node: &Node, matrix: &Vec<Vec<Node>>, num_steps: u32) -> u32 {
    if target_node.is_start_node() {
        return num_steps + 1;
    }
    let (n1, n2) = find_neighbour_nodes(target_node, matrix);
    if current_node == n1 {
        traverse_and_count(target_node, n2, matrix, num_steps + 1)
    } else {
        traverse_and_count(target_node, n1, matrix, num_steps + 1)
    }
}

fn find_neighbour_nodes<'a>(target: &'a Node, matrix: &'a [Vec<Node>]) -> (&'a Node, &'a Node) {
    let directions_to_check = match target.clone().node_type {
        NodeType::Pipe(direction) => direction,
        NodeType::Start => vec![Direction::North, Direction::East, Direction::South, Direction::West],
        _ => panic!("Checking neighbours for ground... not possible.")
    };
    let mut neighbours = Vec::new();
    for direction in directions_to_check {
        if let Some(connected) = test_neighbour(target, direction, matrix) {
            neighbours.push(connected);
        }
    }

    if neighbours.len() != 2 {
        panic!("Node {:?} should only have 2 connected neighbors, had {}, {:?}", target, neighbours.len(), neighbours);
    }
    (neighbours[0], neighbours[1])
}

fn test_neighbour<'a>(target: &Node, direction: Direction, matrix: &'a [Vec<Node>]) -> Option<&'a Node> {
    let row_num = target.point.row_num;
    let col_num = target.point.col_num;
    let (row_num, col_num) = match direction {
        Direction::North => {
            (row_num - 1, col_num)
        }
        Direction::East => {
            (row_num, col_num + 1)
        }
        Direction::South => {
            (row_num + 1, col_num)
        }
        Direction::West => {
            (row_num, col_num - 1)
        }
    };
    let neighbour: &Node = matrix
        .get(row_num)?
        .get(col_num)?;
    match neighbour {
        Node { node_type: NodeType::Pipe( dir ), .. } => {
            if dir.contains(&direction.opposite()) {
                Some(neighbour)
            } else {
                None
            }
        },
        Node { node_type: NodeType::Start, .. } => {
            Some(neighbour)
        },
        _ => None,
    }
}

fn find_start_node(matrix: &[Vec<Node>]) -> &Node {
    matrix
        .iter()
        .flatten()
        .find(|node: &&Node| match node {
            Node { node_type: NodeType::Start, .. } => true,
            Node { .. } => false,
        })
        .expect("A start node should be present in the matrix")
}

fn parse_to_node(c: char, point: Point) -> Node {
    match c {
        '|' => Node { point, node_type: NodeType::Pipe(vec![Direction::North, Direction::South]) },
        '-' => Node { point, node_type: NodeType::Pipe(vec![Direction::West, Direction::East]) },
        'L' => Node { point, node_type: NodeType::Pipe(vec![Direction::North, Direction::East]) },
        'J' => Node { point, node_type: NodeType::Pipe(vec![Direction::North, Direction::West]) },
        '7' => Node { point, node_type: NodeType::Pipe(vec![Direction::West, Direction::South]) },
        'F' => Node { point, node_type: NodeType::Pipe(vec![Direction::South, Direction::East]) },
        '.' => Node { point, node_type: NodeType::Ground },
        'S' => Node { point, node_type: NodeType::Start },
        _ => panic!("Invalid input, character {} not a valid node", c),
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Node {
    point: Point,
    node_type: NodeType,
}

impl Node {
    fn is_start_node(&self) -> bool {
        self.node_type == NodeType::Start
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum NodeType {
    Ground,
    Start,
    Pipe(Vec<Direction>)
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Point {
    row_num: usize,
    col_num: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";

        assert_eq!(find_furthest_length(input), 4);
    }
}
