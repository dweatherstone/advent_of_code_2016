pub fn result_day09_stage1(lines: &[String]) -> usize {
    lines.iter().map(|line| decompress_length(line)).sum()
}

pub fn result_day09_stage2(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|line| decompress_length_recurse(line))
        .sum()
}

fn decompress_length(input: &str) -> usize {
    let mut total_len = 0;
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'(' {
            // Find the end of the marker
            let end_paren = i + input[i..].find(')').unwrap();
            let marker = &input[i + 1..end_paren];

            if let Some((chars, times)) = marker.split_once('x') {
                let num_chars = chars.parse::<usize>().unwrap();
                let repeat_count = times.parse::<usize>().unwrap();
                total_len += num_chars * repeat_count;
                // Move cursor past the marker AND the characters it repeated
                i = end_paren + 1 + num_chars;
            }
        } else {
            total_len += 1;
            i += 1;
        }
    }

    total_len
}

fn decompress_length_recurse(input: &str) -> usize {
    let Some((prefix, rest)) = input.split_once('(') else {
        return input.len();
    };

    // The length of characters before the marker
    let prefix_len = prefix.len();

    // Parse the marker
    let end_paren = rest.find(')').expect("invalid marker format");
    let (marker, after_marker) = rest.split_at(end_paren);

    // Skip the ')' itself
    let after_marker = &after_marker[1..];

    let (chars_str, times_str) = marker.split_once('x').expect("invalid marker content");
    let num_chars: usize = chars_str.parse().unwrap();
    let repeat_count: usize = times_str.parse().unwrap();

    // Split the data to be repeated from the rest of the string
    let (repeated_segment, remainder) = after_marker.split_at(num_chars);

    // Recursion:
    // 1. Decompress the segment inside the marker and multiply
    // 2. Decompress everything that comes after the marker's range
    prefix_len
        + (repeat_count * decompress_length_recurse(repeated_segment))
        + decompress_length_recurse(remainder)
}

#[cfg(test)]
mod day09_tests {
    use super::*;

    #[test]
    fn stage1() {
        let tests = [
            (vec![String::from("ADVENT")], 6),
            (vec![String::from("A(1x5)BC")], 7),
            (vec![String::from("(3x3)XYZ")], 9),
            (vec![String::from("A(2x2)BCD(2x2)EFG")], 11),
            (vec![String::from("(6x1)(1x3)A")], 6),
            (vec![String::from("X(8x2)(3x3)ABCY")], 18),
        ];
        for (input_lines, expected) in tests {
            let result = result_day09_stage1(&input_lines);
            assert_eq!(expected, result);
        }
    }

    #[test]
    fn stage2() {
        let tests = [
            (vec![String::from("(3x3)XYZ")], 9),
            (vec![String::from("X(8x2)(3x3)ABCY")], 20),
            (
                vec![String::from("(27x12)(20x12)(13x14)(7x10)(1x12)A")],
                241920,
            ),
            (
                vec![String::from(
                    "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN",
                )],
                445,
            ),
        ];
        for (input_lines, expected) in tests {
            let result = result_day09_stage2(&input_lines);
            assert_eq!(expected, result);
        }
    }
}
