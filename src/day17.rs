use std::collections::VecDeque;

pub fn result_day17_stage1(lines: &[String]) -> String {
    let passcode = &lines[0];
    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), String::new()));
    let target: Point = (3, 3);
    while let Some((position, path)) = queue.pop_front() {
        if position == target {
            return path;
        }

        let hash = format!("{:x}", md5::compute(format!("{}{}", passcode, path)));
        let open_doors: Vec<bool> = hash.chars().take(4).map(|c| "bcdef".contains(c)).collect();
        for (index, _) in open_doors.iter().enumerate().filter(|&(_, d)| *d) {
            let direction = Direction::from_index(index);
            if !direction.is_valid(&position) {
                continue;
            }
            let delta = direction.delta();
            let new_position = (position.0 + delta.0, position.1 + delta.1);
            let mut new_path = path.clone();
            new_path.push(direction.letter());
            queue.push_back((new_position, new_path));
        }
    }
    String::new()
}

pub fn result_day17_stage2(lines: &[String]) -> usize {
    let passcode = &lines[0];
    let path = String::new();
    find_longest(passcode, (0, 0), path).unwrap_or(usize::MAX)
}

fn find_longest(passcode: &str, pos: Point, path: String) -> Option<usize> {
    if pos == (3, 3) {
        return Some(path.len());
    }

    let mut max_len = None;
    let hash = format!("{:x}", md5::compute(format!("{}{}", passcode, path)));
    let open_doors: Vec<bool> = hash.chars().take(4).map(|c| "bcdef".contains(c)).collect();
    for (index, _) in open_doors.iter().enumerate().filter(|&(_, d)| *d) {
        let direction = Direction::from_index(index);
        if !direction.is_valid(&pos) {
            continue;
        }
        let delta = direction.delta();
        let new_pos = (pos.0 + delta.0, pos.1 + delta.1);
        let mut new_path = path.clone();
        new_path.push(direction.letter());
        if let Some(len) = find_longest(passcode, new_pos, new_path) {
            max_len = Some(max_len.unwrap_or(0).max(len));
        }
    }

    max_len
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_index(index: usize) -> Self {
        match index {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => panic!("should only be 4 directions"),
        }
    }

    fn letter(&self) -> char {
        match self {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
        }
    }

    fn delta(&self) -> Point {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn is_valid(&self, current_pos: &Point) -> bool {
        match self {
            Direction::Up => current_pos.1 > 0,
            Direction::Down => current_pos.1 < 3,
            Direction::Left => current_pos.0 > 0,
            Direction::Right => current_pos.0 < 3,
        }
    }
}

type Point = (isize, isize);

#[cfg(test)]
mod day17_tests {
    use super::*;

    #[test]
    fn stage1() {
        let tests = [
            (String::from("ihgpwlah"), String::from("DDRRRD")),
            (String::from("kglvqrro"), String::from("DDUDRLRRUDRD")),
            (
                String::from("ulqzkmiv"),
                String::from("DRURDRUDDLLDLUURRDULRLDUUDDDRR"),
            ),
        ];
        for (input, expected) in tests {
            let result = result_day17_stage1(&[input]);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn stage2() {
        let tests = [
            (String::from("ihgpwlah"), 370),
            (String::from("kglvqrro"), 492),
            (String::from("ulqzkmiv"), 830),
        ];
        for (input, expected) in tests {
            let result = result_day17_stage2(std::slice::from_ref(&input));
            assert_eq!(
                result, expected,
                "For input '{input}', expected {expected}, but got {result}"
            );
        }
    }
}
