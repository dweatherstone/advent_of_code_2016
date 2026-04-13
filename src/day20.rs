pub fn result_day20_stage1(lines: &[String]) -> u32 {
    let ranges = get_ranges(lines);
    let mut min_val = 0;
    for &(start, end) in ranges.iter() {
        if min_val >= start && min_val <= end {
            min_val = end + 1;
        }
    }
    min_val
}

pub fn result_day20_stage2(lines: &[String]) -> u32 {
    let ranges = get_ranges(lines);
    get_allowed_count(&ranges, u32::MAX)
}

fn get_allowed_count(ranges: &[(u32, u32)], max_val: u32) -> u32 {
    let mut allowed = 0;
    let mut prev_end = 0;
    for &(start, end) in ranges {
        if start > prev_end {
            allowed += start - prev_end - 1;
        }
        if end > prev_end {
            prev_end = end;
        }
    }
    allowed += max_val - prev_end;
    allowed
}

fn get_ranges(lines: &[String]) -> Vec<(u32, u32)> {
    let mut ranges = Vec::new();
    for line in lines {
        let (start_str, end_str) = line.split_once('-').unwrap();
        let start: u32 = start_str.trim().parse().unwrap();
        let end: u32 = end_str.trim().parse().unwrap();
        ranges.push((start, end));
    }
    ranges.sort_by_key(|range| range.0);
    ranges
}

#[cfg(test)]
mod day20_tests {
    use super::*;

    fn get_example_lines() -> Vec<String> {
        vec![
            String::from("5-8"),
            String::from("0-2"),
            String::from("4-7"),
        ]
    }

    #[test]
    fn stage1() {
        let lines = get_example_lines();
        let result = result_day20_stage1(&lines);
        assert_eq!(result, 3);
    }

    #[test]
    fn stage2() {
        let lines = get_example_lines();
        let ranges = get_ranges(&lines);
        let result = get_allowed_count(&ranges, 9);
        assert_eq!(result, 2);
    }
}
