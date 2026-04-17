pub fn result_day21_stage1(lines: &[String]) -> String {
    let mut pg = PasswordGenerator::new(lines, "abcdefgh").unwrap();
    pg.apply_operations();
    pg.letters.iter().collect()
}

pub fn result_day21_stage2(lines: &[String]) -> String {
    let mut pg = PasswordGenerator::new(lines, "fbgdceah").unwrap();
    pg.apply_reverse_operations();
    pg.letters.iter().collect()
}

struct PasswordGenerator {
    letters: Vec<char>,
    operations: Vec<Operation>,
}

impl PasswordGenerator {
    fn new(lines: &[String], initial_string: &str) -> Option<PasswordGenerator> {
        let mut operations = Vec::new();
        for line in lines {
            if let Some(operation) = Operation::from_line(line) {
                operations.push(operation);
            } else {
                return None;
            }
        }
        let letters: Vec<char> = initial_string.chars().collect();
        Some(PasswordGenerator {
            letters,
            operations,
        })
    }

    fn apply_operations(&mut self) {
        let len = self.operations.len();
        for i in 0..len {
            match self.operations[i] {
                Operation::SwapPosition(x, y) => self.swap_positions(x, y),
                Operation::SwapLetter(x, y) => self.swap_letters(x, y),
                Operation::RotateLeft(x) => self.rotate(x, true),
                Operation::RotateRight(x) => self.rotate(x, false),
                Operation::RotatePosition(x) => self.rotate_position(x),
                Operation::Reverse(x, y) => self.reverse(x, y),
                Operation::Move(x, y) => self.move_letter(x, y),
            }
        }
    }

    fn apply_reverse_operations(&mut self) {
        let len = self.operations.len();
        for i in (0..len).rev() {
            match self.operations[i] {
                Operation::SwapPosition(x, y) => self.swap_positions(x, y),
                Operation::SwapLetter(x, y) => self.swap_letters(x, y),
                Operation::RotateLeft(x) => self.rotate(x, false),
                Operation::RotateRight(x) => self.rotate(x, true),
                Operation::RotatePosition(x) => self.rotate_position_rev(x),
                Operation::Reverse(x, y) => self.reverse(x, y),
                Operation::Move(x, y) => self.move_letter_rev(x, y),
            }
        }
    }

    fn swap_positions(&mut self, x: usize, y: usize) {
        self.letters.swap(x, y);
    }

    fn swap_letters(&mut self, x_char: char, y_char: char) {
        for ch in self.letters.iter_mut() {
            if ch == &x_char {
                *ch = y_char;
            } else if ch == &y_char {
                *ch = x_char;
            }
        }
    }

    fn rotate(&mut self, x: usize, is_left: bool) {
        if is_left {
            self.letters.rotate_left(x);
        } else {
            self.letters.rotate_right(x);
        }
    }

    fn rotate_position(&mut self, x: char) {
        self.letters = forward_rotate_rule(&self.letters, x);
    }

    fn rotate_position_rev(&mut self, x: char) {
        let mut test_state = self.letters.clone();
        for _ in 0..self.letters.len() {
            // Rotate left once
            test_state.rotate_left(1);

            // Check if applying the FORWARD rule to test_state
            // result in our current self.letters
            if forward_rotate_rule(&test_state, x) == self.letters {
                self.letters = test_state;
                return;
            }
        }
    }

    fn reverse(&mut self, x: usize, y: usize) {
        self.letters[x..=y].reverse();
    }

    fn move_letter(&mut self, x: usize, y: usize) {
        let ch = self.letters.remove(x);
        self.letters.insert(y, ch);
    }

    fn move_letter_rev(&mut self, x: usize, y: usize) {
        let ch = self.letters.remove(y);
        self.letters.insert(x, ch);
    }
}

fn forward_rotate_rule(letters: &[char], x: char) -> Vec<char> {
    let x_idx = letters.iter().position(|&c| c == x).unwrap();
    let rotate_amount = if x_idx >= 4 { 2 + x_idx } else { 1 + x_idx };
    let rotate_amount = rotate_amount % letters.len();
    let mut result = letters.to_vec();
    result.rotate_right(rotate_amount);
    result
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operation {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateRight(usize),
    RotateLeft(usize),
    RotatePosition(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Operation {
    fn from_line(line: &str) -> Option<Operation> {
        let mut words = line.split_whitespace();
        match words.next().unwrap() {
            "swap" => match words.next().unwrap() {
                "position" => {
                    let x_str = words.next().unwrap();
                    let x: usize = x_str.parse().unwrap();
                    assert_eq!(words.next(), Some("with"));
                    assert_eq!(words.next(), Some("position"));
                    let y_str = words.next().unwrap();
                    let y: usize = y_str.parse().unwrap();
                    Some(Operation::SwapPosition(x, y))
                }
                "letter" => {
                    let x_str = words.next().unwrap();
                    let x = x_str.chars().nth(0).unwrap();
                    assert_eq!(words.next(), Some("with"));
                    assert_eq!(words.next(), Some("letter"));
                    let y_str = words.next().unwrap();
                    let y = y_str.chars().nth(0).unwrap();
                    Some(Operation::SwapLetter(x, y))
                }
                _ => None,
            },
            "rotate" => match words.next().unwrap() {
                "left" => {
                    let x_str = words.next().unwrap();
                    let x: usize = x_str.parse().unwrap();
                    Some(Operation::RotateLeft(x))
                }
                "right" => {
                    let x_str = words.next().unwrap();
                    let x: usize = x_str.parse().unwrap();
                    Some(Operation::RotateRight(x))
                }
                "based" => {
                    assert_eq!(words.next(), Some("on"));
                    assert_eq!(words.next(), Some("position"));
                    assert_eq!(words.next(), Some("of"));
                    assert_eq!(words.next(), Some("letter"));
                    let x_str = words.next().unwrap();
                    let x = x_str.chars().nth(0).unwrap();
                    Some(Operation::RotatePosition(x))
                }
                _ => None,
            },
            "reverse" => {
                assert_eq!(words.next(), Some("positions"));
                let x_str = words.next().unwrap();
                let x: usize = x_str.parse().unwrap();
                assert_eq!(words.next(), Some("through"));
                let y_str = words.next().unwrap();
                let y: usize = y_str.parse().unwrap();
                Some(Operation::Reverse(x, y))
            }
            "move" => {
                assert_eq!(words.next(), Some("position"));
                let x_str = words.next().unwrap();
                let x: usize = x_str.parse().unwrap();
                assert_eq!(words.next(), Some("to"));
                assert_eq!(words.next(), Some("position"));
                let y_str = words.next().unwrap();
                let y: usize = y_str.parse().unwrap();
                Some(Operation::Move(x, y))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod day21_tests {
    use super::*;

    fn get_example_lines() -> Vec<String> {
        vec![
            String::from("swap position 4 with position 0"),
            String::from("swap letter d with letter b"),
            String::from("reverse positions 0 through 4"),
            String::from("rotate left 1 step"),
            String::from("move position 1 to position 4"),
            String::from("move position 3 to position 0"),
            String::from("rotate based on position of letter b"),
            String::from("rotate based on position of letter d"),
        ]
    }

    #[test]
    fn parse() {
        let lines = get_example_lines();
        let expected = [
            Some(Operation::SwapPosition(4, 0)),
            Some(Operation::SwapLetter('d', 'b')),
            Some(Operation::Reverse(0, 4)),
            Some(Operation::RotateLeft(1)),
            Some(Operation::Move(1, 4)),
            Some(Operation::Move(3, 0)),
            Some(Operation::RotatePosition('b')),
            Some(Operation::RotatePosition('d')),
        ];
        for (i, line) in lines.iter().enumerate() {
            let result = Operation::from_line(line);
            assert_eq!(result, expected[i]);
        }
    }

    #[test]
    fn single_operations() {
        let letters = vec!['a', 'b', 'c', 'd', 'e'];
        let mut pg = PasswordGenerator {
            letters,
            operations: Vec::new(),
        };
        pg.swap_positions(4, 0);
        assert_eq!(pg.letters, vec!['e', 'b', 'c', 'd', 'a']);
    }

    #[test]
    fn stage1() {
        let mut operations = Vec::new();
        for line in get_example_lines() {
            if let Some(operation) = Operation::from_line(&line) {
                operations.push(operation);
            } else {
                panic!("could not parse input line: {line}");
            }
        }
        let letters = vec!['a', 'b', 'c', 'd', 'e'];
        let mut pg = PasswordGenerator {
            letters,
            operations,
        };
        pg.apply_operations();
        let result: String = pg.letters.iter().collect();
        assert_eq!(result, String::from("decab"));
    }
}
