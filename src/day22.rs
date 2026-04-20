use std::collections::{HashMap, HashSet, VecDeque};

pub fn result_day22_stage1(lines: &[String]) -> usize {
    let nodes = get_nodes(lines);
    let mut viable_pairs: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    for (a_loc, a) in nodes.iter() {
        // Rule 1: Node A is NOT empty
        if a.used == 0 {
            continue;
        }
        for (b_loc, b) in nodes.iter() {
            // Rule 2: Nodes A and B are NOT the same node
            if a_loc == b_loc {
                continue;
            }
            // Rule 3: The data on node A WOULD fit on node B
            if a.used > b.size - b.used {
                continue;
            }
            viable_pairs.insert((*a_loc, *b_loc));
        }
    }
    viable_pairs.len()
}

pub fn result_day22_stage2(lines: &[String]) -> i32 {
    let nodes = get_nodes(lines);
    // Find the dimensions, and average size for determining wall nodes
    let max_x = nodes.keys().map(|&(x, _)| x).max().unwrap();
    // let max_y = nodes.keys().map(|&(_, y)| y).max().unwrap();
    let avg_size = nodes.values().map(|node| node.size).sum::<u32>() / nodes.len() as u32;

    // print_grid(&nodes, max_x, max_y, avg_size);
    let hole_start = nodes
        .values()
        .find(|n| n.used == 0)
        .map(|n| (n.x, n.y))
        .unwrap();

    let target_pos = (max_x - 1, 0);

    // ((x, y), distance)
    let mut queue = VecDeque::new();
    queue.push_back((hole_start, 0));

    let mut visited = HashSet::new();
    visited.insert(hole_start);

    let mut distance = 0;
    while let Some(((cx, cy), dist)) = queue.pop_front() {
        if (cx, cy) == target_pos {
            distance = dist;
            break;
        }

        // Check neighbours
        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = cx + dx;
            let ny = cy + dy;

            if let Some(next_node) = nodes.get(&(nx, ny)) {
                // If it's not a wall and not visited
                if next_node.used < avg_size * 3 && !visited.contains(&(nx, ny)) {
                    visited.insert((nx, ny));
                    queue.push_back(((nx, ny), dist + 1));
                }
            }
        }
    }
    // distance (to (max_x - 1, 0)) + move goal to empty + shuffle empty node round the target data for all steps to origin
    distance + 1 + (max_x - 1) * 5
}

// fn print_grid(nodes: &HashMap<(i32, i32), Node>, max_x: i32, max_y: i32, avg_size: u32) {
//     // Just print the grid for now...
//     for y in 0..=max_y {
//         for x in 0..=max_x {
//             let node = nodes.get(&(x, y)).unwrap();
//             // Logic to choose symbol
//             // Origin
//             if x == 0 && y == 0 {
//                 print!("( )");
//             }
//             // Goal
//             else if x == max_x && y == 0 {
//                 print!(" G ");
//             }
//             // Empty
//             else if node.used == 0 {
//                 print!(" _ ");
//             }
//             // Wall
//             else if node.used > avg_size * 3 {
//                 print!(" # ");
//             } else {
//                 print!(" . ");
//             }
//         }
//         println!();
//     }
// }

#[derive(Debug, PartialEq)]
struct Node {
    name: String,
    x: i32,
    y: i32,
    size: u32,
    used: u32,
    contains_target: bool,
}

impl Node {
    fn from_line(line: &str) -> Option<Self> {
        if !line.starts_with("/dev/grid/") {
            return None;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        let location = parts[0];
        let name = location.split("/").last()?;
        let name_parts: Vec<&str> = name.split("-").collect();
        let x: i32 = name_parts[1].trim_start_matches("x").parse().ok()?;
        let y: i32 = name_parts[2].trim_start_matches("y").parse().ok()?;
        let size: u32 = parts[1].trim_end_matches("T").parse().ok()?;
        let used: u32 = parts[2].trim_end_matches("T").parse().ok()?;
        Some(Node {
            name: name.to_string(),
            x,
            y,
            size,
            used,
            contains_target: false,
        })
    }
}

fn get_nodes(lines: &[String]) -> HashMap<(i32, i32), Node> {
    let mut nodes = HashMap::new();
    for line in lines {
        if let Some(node) = Node::from_line(line) {
            nodes.insert((node.x, node.y), node);
        }
    }
    nodes
}

#[cfg(test)]
mod day22_tests {
    use super::*;

    #[test]
    fn parse() {
        let tests = [
            ("root@ebhq-gridcenter# df -h", None),
            ("Filesystem              Size  Used  Avail  Use%", None),
            (
                "/dev/grid/node-x0-y0     85T   72T    13T   84%",
                Some(Node {
                    name: String::from("node-x0-y0"),
                    x: 0,
                    y: 0,
                    size: 85,
                    used: 72,
                    contains_target: false,
                }),
            ),
            (
                "/dev/grid/node-x31-y29   86T   72T    14T   83%",
                Some(Node {
                    name: String::from("node-x31-y29"),
                    x: 31,
                    y: 29,
                    size: 86,
                    used: 72,
                    contains_target: false,
                }),
            ),
        ];
        for (line, expected) in tests {
            let result = Node::from_line(line);
            assert_eq!(result, expected);
        }
    }
}
