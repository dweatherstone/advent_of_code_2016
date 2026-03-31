use itertools::Itertools;

pub fn result_day16_stage1(lines: &[String]) -> String {
    let state = lines[0].clone();
    let target_len: usize = lines[1].parse().unwrap();
    fill_and_checksum(state, target_len)
}

pub fn result_day16_stage2(lines: &[String]) -> String {
    let state = lines[0].clone();
    let target_len: usize = lines[2].parse().unwrap();
    fill_and_checksum(state, target_len)
}

fn fill_and_checksum(mut state: String, target_len: usize) -> String {
    while state.len() < target_len {
        let mut b = state.clone();
        b = b.chars().rev().collect();
        b = b
            .chars()
            .map(|c| match c {
                '0' => '1',
                '1' => '0',
                _ => c,
            })
            .collect();
        state = format!("{}0{}", state, b);
    }
    state = state.chars().take(target_len).collect();
    get_checksum(&state)
}

fn get_checksum(state: &str) -> String {
    let mut checksum = String::from(state);
    loop {
        checksum = checksum
            .chars()
            .tuples()
            .map(|(a, b)| if a == b { '1' } else { '0' })
            .collect();
        if checksum.len() % 2 == 1 {
            break;
        }
    }
    checksum
}

#[cfg(test)]
mod day16_tests {
    use super::*;

    #[test]
    fn stage1() {
        let lines = [String::from("10000"), String::from("20")];
        let result = result_day16_stage1(&lines);
        assert_eq!(result, "01100");
    }
}
