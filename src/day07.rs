use std::collections::HashSet;

pub fn result_day07_stage1(lines: &[String]) -> usize {
    lines.iter().filter(|input| supports_tls(input)).count()
}

pub fn result_day07_stage2(lines: &[String]) -> usize {
    lines.iter().filter(|s| supports_ssl(s)).count()
}

fn supports_tls(input: &str) -> bool {
    let mut has_abba_outside = false;

    for (i, segment) in input.split(['[', ']']).enumerate() {
        if is_abba(segment) {
            if i % 2 == 1 {
                // inside brackets
                return false;
            } else {
                has_abba_outside = true
            }
        }
    }
    has_abba_outside
}

fn supports_ssl(input: &str) -> bool {
    let mut outside = HashSet::new();
    let mut inside = HashSet::new();
    for (i, segment) in input.split(['[', ']']).enumerate() {
        for w in segment.as_bytes().windows(3) {
            if w[0] == w[2] && w[0] != w[1] {
                if i % 2 == 0 {
                    outside.insert((w[0], w[1]));
                } else {
                    inside.insert((w[1], w[0]));
                }
            }
        }
    }
    outside.iter().any(|x| inside.contains(x))
}

fn is_abba(input: &str) -> bool {
    let input_chars: Vec<char> = input.chars().collect();
    input_chars
        .windows(4)
        .any(|x| x[0] == x[3] && x[1] == x[2] && x[0] != x[1])
}

#[cfg(test)]
mod day07_tests {
    use super::*;

    #[test]
    fn stage1() {
        let tests = [
            ("abba[mnop]qrst", true),
            ("abcd[bddb]xyyx", false),
            ("aaaa[qwer]tyui", false),
            ("ioxxoj[asdfgh]zxcvbn", true),
        ];
        for (input, expected) in tests.iter() {
            assert_eq!(supports_tls(input), *expected);
        }
        let lines: Vec<String> = tests.iter().map(|(s, _)| s.to_string()).collect();
        let result = result_day07_stage1(&lines);
        assert_eq!(result, 2);
    }

    #[test]
    fn stage2() {
        let tests = [
            ("aba[bab]xyz", true),
            ("xyx[xyx]xyx", false),
            ("aaa[kek]eke", true),
            ("zazbz[bzb]cdb", true),
        ];
        for (input, expected) in tests.iter() {
            assert_eq!(supports_ssl(input), *expected);
        }
        let lines: Vec<String> = tests.iter().map(|(s, _)| s.to_string()).collect();
        let result = result_day07_stage2(&lines);
        assert_eq!(result, 3);
    }
}
