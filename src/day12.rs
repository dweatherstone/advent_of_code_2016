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

pub struct Program {
    pub instructions: Vec<Instruction>,
    pub current_idx: usize,
    pub registers: [i32; 4],
}

impl Program {
    pub fn parse_lines(lines: &[String]) -> Self {
        let mut instructions = Vec::new();
        for line in lines {
            let words: Vec<&str> = line.split_whitespace().collect();
            let instruction = match words[0] {
                "cpy" => {
                    let x = Parameter::parse(words[1]);
                    let y = Parameter::parse(words[2]);
                    Instruction::Cpy(x, y)
                }
                "inc" => {
                    let x = Parameter::parse(words[1]);
                    Instruction::Inc(x)
                }
                "dec" => {
                    let x = Parameter::parse(words[1]);
                    Instruction::Dec(x)
                }
                "jnz" => {
                    let x = Parameter::parse(words[1]);
                    let y = Parameter::parse(words[2]);
                    Instruction::Jnz(x, y)
                }
                "tgl" => {
                    let x = Parameter::parse(words[1]);
                    Instruction::Tgl(x)
                }
                "out" => {
                    let x = Parameter::parse(words[1]);
                    Instruction::Out(x)
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

    pub fn run(&mut self) {
        let instructions_qty = self.instructions.len();
        while self.current_idx < instructions_qty {
            self.process_instruction();
        }
    }

    pub fn process_instruction(&mut self) {
        match self.instructions[self.current_idx] {
            Instruction::Cpy(x, y) => {
                let new_value = match x {
                    Parameter::Integer(value) => value,
                    Parameter::Register(idx) => self.registers[idx],
                };
                let register_idx = match y {
                    Parameter::Integer(value) => value as usize,
                    Parameter::Register(idx) => idx,
                };
                self.registers[register_idx] = new_value;
            }
            Instruction::Inc(x) => {
                let regesiter_idx = match x {
                    Parameter::Integer(value) => value as usize,
                    Parameter::Register(idx) => idx,
                };
                self.registers[regesiter_idx] += 1;
            }
            Instruction::Dec(x) => {
                let register_idx = match x {
                    Parameter::Integer(value) => value as usize,
                    Parameter::Register(idx) => idx,
                };
                self.registers[register_idx] -= 1;
            }
            Instruction::Jnz(x, y) => {
                let x_value = match x {
                    Parameter::Integer(value) => value,
                    Parameter::Register(idx) => self.registers[idx],
                };
                if x_value == 0 {
                    self.current_idx += 1;
                } else {
                    let y_value = match y {
                        Parameter::Integer(value) => value as isize,
                        Parameter::Register(idx) => self.registers[idx] as isize,
                    };
                    self.current_idx = self.current_idx.wrapping_add_signed(y_value);
                }
                return;
            }
            Instruction::Tgl(x) => self.toggle_instruction(&x),
            Instruction::Out(_) => {}
        }
        self.current_idx += 1;
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instruction {
    Cpy(Parameter, Parameter),
    Inc(Parameter),
    Dec(Parameter),
    Jnz(Parameter, Parameter),
    Tgl(Parameter),
    Out(Parameter),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Parameter {
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
