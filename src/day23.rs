use crate::day12::{Instruction, Parameter, Program};

pub fn result_day23_stage1(lines: &[String]) -> i32 {
    let mut program = Program::parse_lines(lines);
    // Instructions say register a starts as 7
    program.registers[0] = 7;
    program.run();
    // need register a
    program.registers[0]
}

pub fn result_day23_stage2(lines: &[String]) -> i32 {
    let mut program = Program::parse_lines(lines);
    // Instructions say register a starts as 12 for part 2
    program.registers[0] = 12;
    program.run();
    // need register a
    program.registers[0]
}

impl Program {
    pub fn toggle_instruction(&mut self, inst_diff: &Parameter) {
        let line_diff = match inst_diff {
            Parameter::Integer(value) => *value,
            Parameter::Register(idx) => self.registers[*idx],
        };
        let line_to_toggle = self.current_idx as i32 + line_diff;
        if line_to_toggle < 0 || line_to_toggle >= self.instructions.len() as i32 {
            return;
        }
        let line_to_toggle = line_to_toggle as usize;
        let old_instr = self.instructions[line_to_toggle];
        self.instructions[line_to_toggle] = match old_instr {
            Instruction::Inc(x) => Instruction::Dec(x),
            Instruction::Dec(x) => Instruction::Inc(x),
            Instruction::Tgl(x) => Instruction::Inc(x),
            Instruction::Jnz(x, y) => Instruction::Cpy(x, y),
            Instruction::Cpy(x, y) => Instruction::Jnz(x, y),
        };
    }
}

#[cfg(test)]
mod day23_tests {
    use crate::day12::{Instruction, Parameter};

    use super::*;

    fn get_example_lines() -> Vec<String> {
        vec![
            String::from("cpy 2 a"),
            String::from("cpy 3 b"),
            String::from("tgl a"),
            String::from("tgl a"),
            String::from("tgl a"),
            String::from("cpy 1 a"),
            String::from("dec a"),
            String::from("dec a"),
        ]
    }

    #[test]
    fn parse() {
        let program = Program::parse_lines(&get_example_lines());
        let expected = [
            Instruction::Cpy(Parameter::Integer(2), Parameter::Register(0)),
            Instruction::Cpy(Parameter::Integer(3), Parameter::Register(1)),
            Instruction::Tgl(Parameter::Register(0)),
            Instruction::Tgl(Parameter::Register(0)),
            Instruction::Tgl(Parameter::Register(0)),
            Instruction::Cpy(Parameter::Integer(1), Parameter::Register(0)),
            Instruction::Dec(Parameter::Register(0)),
            Instruction::Dec(Parameter::Register(0)),
        ];
        assert_eq!(program.instructions.len(), expected.len());
        for (inst, exp) in program.instructions.iter().zip(expected.iter()) {
            assert_eq!(inst, exp);
        }
    }

    #[test]
    fn stage1() {
        let mut program = Program::parse_lines(&get_example_lines());
        program.run();
        let result = program.registers[0];
        assert_eq!(result, 3);
    }
}
