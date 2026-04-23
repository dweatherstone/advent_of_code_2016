use std::collections::HashSet;

use crate::day12::{Instruction, Parameter, Program};

pub fn result_day25_stage1(lines: &[String]) -> i32 {
    for i in 1..i32::MAX {
        let mut program = Program::parse_lines(lines);
        program.registers[0] = i; // Initialize 'a' with our guess
        if program.run_and_validate() {
            return i;
        }
    }
    0
}

pub fn result_day25_stage2(_lines: &[String]) -> i32 {
    0
}

impl Program {
    fn run_and_validate(&mut self) -> bool {
        let mut history = HashSet::new();
        let mut last_output = 1; // Start at 1 so the first valid output must be 0
        while self.current_idx < self.instructions.len() {
            // Create a snapshot of the current state
            let state = (self.current_idx, self.registers);
            if let Instruction::Out(x) = self.instructions[self.current_idx] {
                let val = match x {
                    Parameter::Integer(v) => v,
                    Parameter::Register(idx) => self.registers[idx],
                };
                // Is the output correct?
                if val != 1 - last_output {
                    return false;
                }
                last_output = val;

                // Have we been here before?
                if history.contains(&state) {
                    return true; // Infinite loop detected
                }
                history.insert(state);
                self.current_idx += 1;
            } else {
                self.process_instruction();
            }
        }
        false
    }
}
