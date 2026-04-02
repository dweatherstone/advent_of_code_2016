use std::collections::HashMap;

pub fn result_day06_stage1(lines: &[String]) -> String {
    get_message(lines, true)
}

pub fn result_day06_stage2(lines: &[String]) -> String {
    get_message(lines, false)
}

fn get_message(lines: &[String], is_max: bool) -> String {
    let mut messages = vec![vec![char::default(); lines.len()]; lines[0].len()];
    for (line_idx, line) in lines.iter().enumerate() {
        for (char_idx, ch) in line.char_indices() {
            messages[char_idx][line_idx] = ch;
        }
    }
    let mut result = String::new();
    for message in messages.iter() {
        let mut freq = HashMap::new();
        for &ch in message.iter() {
            *freq.entry(ch).or_insert(0) += 1;
        }
        let result_char = if is_max {
            freq.iter()
                .max_by_key(|(_, v)| **v)
                .map(|(k, _)| *k)
                .unwrap()
        } else {
            freq.iter()
                .min_by_key(|(_, v)| **v)
                .map(|(k, _)| *k)
                .unwrap()
        };
        result.push(result_char);
    }
    result
}

#[cfg(test)]
mod day06_tests {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            String::from("eedadn"),
            String::from("drvtee"),
            String::from("eandsr"),
            String::from("raavrd"),
            String::from("atevrs"),
            String::from("tsrnev"),
            String::from("sdttsa"),
            String::from("rasrtv"),
            String::from("nssdts"),
            String::from("ntnada"),
            String::from("svetve"),
            String::from("tesnvt"),
            String::from("vntsnd"),
            String::from("vrdear"),
            String::from("dvrsen"),
            String::from("enarar"),
        ]
    }

    #[test]
    fn stage1() {
        let result = result_day06_stage1(&get_example());
        assert_eq!(result, "easter".to_string());
    }

    #[test]
    fn stage2() {
        let result = result_day06_stage2(&get_example());
        assert_eq!(result, "advent".to_string());
    }
}
