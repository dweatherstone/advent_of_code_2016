use std::collections::{HashSet, VecDeque};

pub fn result_day13_stage1(lines: &[String]) -> usize {
    let (favourite_number, target) = parse_lines(lines);
    let start = Point { x: 1, y: 1 };
    solve_bfs(start, favourite_number, target).unwrap_or(usize::MAX)
}

pub fn result_day13_stage2(lines: &[String]) -> usize {
    let (favourite_number, _) = parse_lines(lines);
    let start = Point { x: 1, y: 1 };
    max_50_steps(start, favourite_number)
}

fn parse_lines(lines: &[String]) -> (i32, Point) {
    if lines.len() != 2 {
        panic!("malformed input");
    }
    let favourite_number = lines[0].parse::<i32>().unwrap();
    let (x_str, y_str) = lines[1].split_once(",").unwrap();
    let x = x_str.parse().unwrap();
    let y = y_str.parse().unwrap();
    let target = Point { x, y };
    (favourite_number, target)
}

fn solve_bfs(start_point: Point, favourite_number: i32, target: Point) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start_point, 0));
    visited.insert(start_point);

    while let Some((current, dist)) = queue.pop_front() {
        if current == target {
            return Some(dist);
        }

        for next in current.get_next_points(favourite_number) {
            if visited.insert(next) {
                queue.push_back((next, dist + 1));
            }
        }
    }

    None
}

fn max_50_steps(start_point: Point, favourite_number: i32) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start_point, 0));
    visited.insert(start_point);

    while let Some((current, dist)) = queue.pop_front() {
        if dist == 50 {
            continue;
        }

        for next in current.get_next_points(favourite_number) {
            if visited.insert(next) {
                queue.push_back((next, dist + 1));
            }
        }
    }
    visited.len()
}

fn is_wall(x: i32, y: i32, favourite_number: i32) -> bool {
    let multiplied_val = x * x + 3 * x + 2 * x * y + y + y * y + favourite_number;
    let bits = multiplied_val.count_ones();
    bits % 2 == 1
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn get_next_points(&self, favourite_number: i32) -> Vec<Self> {
        // Cannot go diagonally
        let delta = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut next_points = Vec::new();
        for &(dx, dy) in delta.iter() {
            let nx = self.x + dx;
            let ny = self.y + dy;
            if nx >= 0 && ny >= 0 && !is_wall(nx, ny, favourite_number) {
                next_points.push(Point { x: nx, y: ny });
            }
        }
        next_points
    }
}

#[cfg(test)]
mod day13_tests {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![String::from("10"), String::from("7,4")]
    }

    #[test]
    fn stage1() {
        let result = result_day13_stage1(&get_example());
        assert_eq!(result, 11);
    }
}
