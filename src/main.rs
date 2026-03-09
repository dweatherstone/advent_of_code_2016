use std::{fmt::Display, fs::read_to_string, path::Path, time::Instant};

use strum::EnumIter;

use crate::{
    day01::{result_day01_stage1, result_day01_stage2},
    day02::{result_day02_stage1, result_day02_stage2},
    day03::{restult_day03_stage2, result_day03_stage1},
    day04::{result_day04_stage1, result_day04_stage2},
};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

#[derive(EnumIter)]
enum Days {
    Day01,
    Day02,
    Day03,
    Day04,
}

impl Days {
    fn get_results(&self) {
        use Days::*;

        // match self {
        //     // Standard process from input file
        //     Day01 | Day02 | Day03 => {
        let lines = get_lines(Path::new(&self.get_path_str()));
        let result1 = self.get_result1_from_lines(&lines);
        println!("{self} stage 1: {result1}");
        let result2 = self.get_result2_from_lines(&lines);
        println!("{self} stage 2: {result2}");
        //     }
        // }
    }

    fn get_path_str(&self) -> String {
        use Days::*;

        let filename = match self {
            Day01 => "day01_input.txt",
            Day02 => "day02_input.txt",
            Day03 => "day03_input.txt",
            Day04 => "day04_input.txt",
            //_ => panic!("undefined path string"),
        };
        format!("input/{filename}")
    }

    fn get_result1_from_lines(&self, lines: &[String]) -> Box<dyn Display> {
        use Days::*;
        match self {
            Day01 => Box::new(result_day01_stage1(lines)),
            Day02 => Box::new(result_day02_stage1(lines)),
            Day03 => Box::new(result_day03_stage1(lines)),
            Day04 => Box::new(result_day04_stage1(lines)),
            //_ => panic!("undefined result 1 function")
        }
    }

    fn get_result2_from_lines(&self, lines: &[String]) -> Box<dyn Display> {
        use Days::*;
        match self {
            Day01 => Box::new(result_day01_stage2(lines)),
            Day02 => Box::new(result_day02_stage2(lines)),
            Day03 => Box::new(restult_day03_stage2(lines)),
            Day04 => Box::new(result_day04_stage2(lines)),
            //_ => panic!("undefined result 1 function")
        }
    }

    fn run(&self, with_timing: bool) {
        let start = if with_timing {
            Some(Instant::now())
        } else {
            None
        };
        self.get_results();
        if with_timing {
            let duration = start.unwrap().elapsed();
            println!("{self} time taken: {:?}\n", duration);
        }
    }
}

impl Display for Days {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Days::*;
        match self {
            Day01 => write!(f, "Day 1"),
            Day02 => write!(f, "Day 2"),
            Day03 => write!(f, "Day 3"),
            Day04 => write!(f, "Day 4"),
        }
    }
}

fn main() {
    Days::Day04.run(true);
}

fn get_lines(path: &Path) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
