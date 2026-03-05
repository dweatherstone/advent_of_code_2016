use std::collections::HashSet;

pub fn result_day01_stage1(lines: &[String]) -> i32 {
    let mut north_south: i32 = 0;
    let mut east_west: i32 = 0;
    let mut facing = Direction::North;
    if lines.len() != 1 {
        panic!("input should be a single line");
    }
    for instr in lines[0].split(", ") {
        let turn = parse_direction(instr);
        let amount = match turn {
            Turn::Left(val) => val,
            Turn::Right(val) => val,
        };
        facing = facing.next(&turn);
        match facing {
            Direction::North => north_south -= amount,
            Direction::South => north_south += amount,
            Direction::East => east_west += amount,
            Direction::West => east_west -= amount,
        }
    }

    north_south.abs() + east_west.abs()
}

pub fn result_day01_stage2(lines: &[String]) -> i32 {
    let mut north_south: i32 = 0;
    let mut east_west: i32 = 0;
    let mut facing = Direction::North;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    if lines.len() != 1 {
        panic!("input should be a single line");
    }
    for instr in lines[0].split(", ") {
        let turn = parse_direction(instr);
        let amount = match turn {
            Turn::Left(val) => val,
            Turn::Right(val) => val,
        };
        facing = facing.next(&turn);
        for _ in 0..amount {
            match facing {
                Direction::North => north_south -= 1,
                Direction::South => north_south += 1,
                Direction::East => east_west += 1,
                Direction::West => east_west -= 1,
            }
            if !visited.insert((north_south, east_west)) {
                return north_south.abs() + east_west.abs();
            }
        }
    }
    north_south.abs() + east_west.abs()
}

fn parse_direction(input: &str) -> Turn {
    let (turn, amount_str) = input.split_at(1);
    let amount = amount_str.parse::<i32>().expect("should be a number");
    match turn {
        "L" => Turn::Left(amount),
        "R" => Turn::Right(amount),
        _ => panic!("unknown turn: '{turn}'"),
    }
}

enum Turn {
    Left(i32),
    Right(i32),
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn next(&self, turn: &Turn) -> Self {
        use Direction::*;
        use Turn::*;
        match (self, turn) {
            (North, Left(_)) | (South, Right(_)) => West,
            (North, Right(_)) | (South, Left(_)) => East,
            (East, Left(_)) | (West, Right(_)) => North,
            (East, Right(_)) | (West, Left(_)) => South,
        }
    }
}

#[cfg(test)]
mod day01 {
    use super::*;

    #[test]
    fn stage1() {
        let tests = [
            (["R2, L3".to_string()], 5),
            (["R2, R2, R2".to_string()], 2),
            (["R5, L5, R5, R3".to_string()], 12),
        ];
        for (input, expected) in tests {
            let result = result_day01_stage1(&input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn stage2() {
        let input = ["R8, R4, R4, R8".to_string()];
        let result = result_day01_stage2(&input);
        assert_eq!(result, 4);
    }
}
