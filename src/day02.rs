pub fn result_day02_stage1(lines: &[String]) -> String {
    get_code(lines, true)
}

pub fn result_day02_stage2(lines: &[String]) -> String {
    get_code(lines, false)
}

fn get_code(lines: &[String], is_stage_1: bool) -> String {
    let mut result = String::new();
    let mut position = 5;
    for line in lines {
        for dir in line.chars() {
            position = if is_stage_1 {
                move_position_stage1(position, dir)
            } else {
                move_position_stage2(position, dir)
            };
        }
        result.push_str(&get_string(position));
    }
    result
}

fn move_position_stage1(position: i32, direction: char) -> i32 {
    match direction {
        'R' => match position {
            3 | 6 | 9 => position,
            _ => position + 1,
        },
        'L' => match position {
            1 | 4 | 7 => position,
            _ => position - 1,
        },
        'U' => match position {
            1..=3 => position,
            _ => position - 3,
        },
        'D' => match position {
            7..=9 => position,
            _ => position + 3,
        },
        _ => panic!("unknown direction"),
    }
}

fn move_position_stage2(position: i32, direction: char) -> i32 {
    match direction {
        'R' => match position {
            1 | 4 | 9 | 12 | 13 => position,
            _ => position + 1,
        },
        'L' => match position {
            1 | 2 | 5 | 10 | 13 => position,
            _ => position - 1,
        },
        'U' => match position {
            1 | 2 | 4 | 5 | 9 => position,
            3 => 1,
            13 => 11,
            _ => position - 4,
        },
        'D' => match position {
            5 | 9 | 10 | 12 | 13 => position,
            1 => 3,
            11 => 13,
            _ => position + 4,
        },
        _ => panic!("unknown direction"),
    }
}

fn get_string(position: i32) -> String {
    match position {
        1..=9 => position.to_string(),
        10 => "A".to_string(),
        11 => "B".to_string(),
        12 => "C".to_string(),
        13 => "D".to_string(),
        _ => panic!("integer not on keypad"),
    }
}

#[cfg(test)]
mod day02 {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            String::from("ULL"),
            String::from("RRDDD"),
            String::from("LURDL"),
            String::from("UUUUD"),
        ]
    }

    #[test]
    fn stage1() {
        let result = result_day02_stage1(&get_example());
        assert_eq!(result, "1985".to_string());
    }

    #[test]
    fn stage2() {
        let result = result_day02_stage2(&get_example());
        assert_eq!(result, "5DB3".to_string());
    }
}
