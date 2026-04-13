pub fn result_day18_stage1(lines: &[String]) -> usize {
    if lines.len() != 1 {
        panic!("input should only have one line");
    }
    solve(lines[0].as_str(), 40)
}

pub fn result_day18_stage2(lines: &[String]) -> usize {
    if lines.len() != 1 {
        panic!("input should only have one line");
    }
    solve(lines[0].as_str(), 400000)
}

fn solve(input: &str, total_rows: usize) -> usize {
    let mut current_row: Vec<bool> = input.trim().chars().map(|c| c == '^').collect();

    let row_len = current_row.len();
    let mut safe_count = current_row.iter().filter(|&&is_trap| !is_trap).count();
    for _ in 1..total_rows {
        let mut next_row = Vec::with_capacity(row_len);
        for i in 0..row_len {
            // Left XOR Right
            let left = if i > 0 { current_row[i - 1] } else { false };
            let right = if i < row_len - 1 {
                current_row[i + 1]
            } else {
                false
            };
            next_row.push(left ^ right);
        }

        safe_count += next_row.iter().filter(|&&is_trap| !is_trap).count();
        current_row = next_row;
    }

    safe_count
}

#[cfg(test)]
mod day18_tests {
    use super::*;

    #[test]
    fn simple_example() {
        let input = "..^^.";
        let result = solve(input, 2);
        assert_eq!(result, 4, "failed test on first line");
        let result = solve(input, 3);
        assert_eq!(result, 6, "failed test on second line");
    }

    #[test]
    fn stage1() {
        let input = ".^^.^.^^^^";
        let result = solve(input, 10);
        assert_eq!(result, 38);
    }
}
