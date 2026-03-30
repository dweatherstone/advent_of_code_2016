pub fn result_day12_stage1(lines: &[String]) -> i32 {
    let mut program = Program::parse_lines(lines);
    program.run();
    // need register a
    program.registers[0]
}

pub fn result_day12_stage2(lines: &[String]) -> i32 {
    let mut program = Program::parse_lines(lines);
    // register c initialized to 1
    program.registers[2] = 1;
    program.run();
    program.registers[0]
}

fn parse_register(word: &str) -> usize {
    match word {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => panic!("unknown register specified"),
    }
}

struct Program {
    instructions: Vec<Instruction>,
    current_idx: usize,
    registers: [i32; 4],
}

impl Program {
    fn parse_lines(lines: &[String]) -> Self {
        let mut instructions = Vec::new();
        for line in lines {
            let words: Vec<&str> = line.split_whitespace().collect();
            let instruction = match words[0] {
                "cpy" => {
                    let x = Parameter::parse(words[1]);
                    let y: usize = parse_register(words[2]);
                    Instruction::Cpy(x, y)
                }
                "inc" => {
                    let x = parse_register(words[1]);
                    Instruction::Inc(x)
                }
                "dec" => {
                    let x = parse_register(words[1]);
                    Instruction::Dec(x)
                }
                "jnz" => {
                    let x = Parameter::parse(words[1]);
                    let y: isize = words[2].parse().unwrap();
                    Instruction::Jnz(x, y)
                }
                _ => panic!("unknown instruction type"),
            };
            instructions.push(instruction);
        }
        Program {
            instructions,
            current_idx: 0,
            registers: [0; 4],
        }
    }

    fn run(&mut self) {
        let instructions_qty = self.instructions.len();
        while self.current_idx < instructions_qty {
            match self.instructions[self.current_idx] {
                Instruction::Cpy(x, y) => {
                    let new_value = match x {
                        Parameter::Integer(value) => value,
                        Parameter::Register(idx) => self.registers[idx],
                    };
                    self.registers[y] = new_value;
                }
                Instruction::Inc(x) => self.registers[x] += 1,
                Instruction::Dec(x) => self.registers[x] -= 1,
                Instruction::Jnz(x, y) => {
                    let x_value = match x {
                        Parameter::Integer(value) => value,
                        Parameter::Register(idx) => self.registers[idx],
                    };
                    if x_value == 0 {
                        self.current_idx += 1;
                    } else {
                        self.current_idx = self.current_idx.wrapping_add_signed(y);
                    }
                    continue;
                }
            }
            self.current_idx += 1;
        }
    }
}

#[derive(Copy, Clone)]
enum Instruction {
    Cpy(Parameter, usize),
    Inc(usize),
    Dec(usize),
    Jnz(Parameter, isize),
}

#[derive(Copy, Clone)]
enum Parameter {
    Integer(i32),
    Register(usize), // a = 0, b = 1, c = 2, d = 3
}

impl Parameter {
    fn parse(word: &str) -> Self {
        // can be either an integer or a register
        if let Ok(value) = word.parse::<i32>() {
            Parameter::Integer(value)
        } else {
            let register = parse_register(word);
            Parameter::Register(register)
        }
    }
}

#[cfg(test)]
mod day12_tests {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            String::from("cpy 41 a"),
            String::from("inc a"),
            String::from("inc a"),
            String::from("dec a"),
            String::from("jnz a 2"),
            String::from("dec a"),
        ]
    }

    #[test]
    fn stage1() {
        let result = result_day12_stage1(&get_example());
        assert_eq!(result, 42);
    }
}
