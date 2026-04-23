use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

pub fn result_day24_stage1(lines: &[String]) -> u32 {
    let state = State::from_lines(lines);
    let dist_matrix = state.calculate_distance_matrix();
    calculate_shortest_path(state.targets.len(), &dist_matrix, false)
}

pub fn result_day24_stage2(lines: &[String]) -> u32 {
    let state = State::from_lines(lines);
    let dist_matrix = state.calculate_distance_matrix();
    calculate_shortest_path(state.targets.len(), &dist_matrix, true)
}

struct State {
    walls: Vec<Vec<bool>>,
    targets: HashMap<usize, (usize, usize)>,
}

impl State {
    fn from_lines(lines: &[String]) -> Self {
        let mut walls = vec![vec![false; lines[0].len()]; lines.len()];
        let mut targets = HashMap::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.char_indices() {
                match ch {
                    ('0'..='9') => _ = targets.insert(ch.to_digit(10).unwrap() as usize, (x, y)),
                    '#' => walls[y][x] = true,
                    '.' => continue,
                    _ => panic!("unexpected character in input"),
                }
            }
        }
        State { walls, targets }
    }

    fn calculate_distance_matrix(&self) -> Vec<Vec<u32>> {
        let num_targets = self.targets.len();
        let mut dist_matrix = vec![vec![0; num_targets]; num_targets];
        for (&start_digit, &start_pos) in self.targets.iter() {
            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();
            queue.push_back((start_pos.0, start_pos.1, 0));
            visited.insert(start_pos);

            while let Some((cx, cy, dist)) = queue.pop_front() {
                for (&other_digit, &other_pos) in self.targets.iter() {
                    if cx == other_pos.0 && cy == other_pos.1 {
                        dist_matrix[start_digit][other_digit] = dist;
                    }
                }

                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = cx as isize + dx;
                    let ny = cy as isize + dy;
                    if nx >= 0
                        && nx < self.walls[0].len() as isize
                        && ny >= 0
                        && ny < self.walls.len() as isize
                    {
                        let new_x = nx as usize;
                        let new_y = ny as usize;
                        if !self.walls[new_y][new_x] && visited.insert((new_x, new_y)) {
                            queue.push_back((new_x, new_y, dist + 1));
                        }
                    }
                }
            }
        }
        dist_matrix
    }
}

fn calculate_shortest_path(
    num_targets: usize,
    dist_matrix: &[Vec<u32>],
    return_to_start: bool,
) -> u32 {
    let remaining_nodes: Vec<usize> = (1..num_targets).collect();
    let mut shortest_path = u32::MAX;
    for path in remaining_nodes.iter().permutations(remaining_nodes.len()) {
        let mut current_total_distance = 0;
        let mut last_node = 0;

        for &next_node in path {
            current_total_distance += dist_matrix[last_node][next_node];
            last_node = next_node;
        }

        if return_to_start {
            current_total_distance += dist_matrix[last_node][0];
        }

        if current_total_distance < shortest_path {
            shortest_path = current_total_distance;
        }
    }

    shortest_path
}

#[cfg(test)]
mod day24_tests {
    use super::*;

    fn get_example_lines() -> Vec<String> {
        vec![
            String::from("###########"),
            String::from("#0.1.....2#"),
            String::from("#.#######.#"),
            String::from("#4.......3#"),
            String::from("###########"),
        ]
    }

    #[test]
    fn parse() {
        let lines = get_example_lines();
        let state = State::from_lines(&lines);
        let expected_walls = vec![
            vec![true; 11],
            vec![
                true, false, false, false, false, false, false, false, false, false, true,
            ],
            vec![
                true, false, true, true, true, true, true, true, true, false, true,
            ],
            vec![
                true, false, false, false, false, false, false, false, false, false, true,
            ],
            vec![true; 11],
        ];
        assert_eq!(state.walls, expected_walls);
        let expected_targets = HashMap::from([
            (0, (1, 1)),
            (1, (3, 1)),
            (2, (9, 1)),
            (3, (9, 3)),
            (4, (1, 3)),
        ]);
        assert_eq!(state.targets, expected_targets);
    }

    #[test]
    fn stage1() {
        let result = result_day24_stage1(&get_example_lines());
        assert_eq!(result, 14);
    }
}
