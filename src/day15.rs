pub fn result_day15_stage1(lines: &[String]) -> usize {
    let discs = parse_lines(lines);
    solve(discs)
}

pub fn result_day15_stage2(lines: &[String]) -> usize {
    let mut discs = parse_lines(lines);
    let max_disc = discs.iter().map(|d| d.id).max().unwrap();
    discs.push(Disc {
        id: max_disc + 1,
        positions: 11,
        start: 0,
    });
    solve(discs)
}

fn parse_lines(lines: &[String]) -> Vec<Disc> {
    let mut discs = Vec::new();
    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        let id = words[1].trim_start_matches("#").parse::<usize>().unwrap();
        let positions = words[3].parse::<usize>().unwrap();
        let start = words[11].trim_end_matches(".").parse::<usize>().unwrap();
        discs.push(Disc {
            id,
            positions,
            start,
        });
    }
    discs
}

fn solve(mut discs: Vec<Disc>) -> usize {
    discs.sort_by_key(|d| std::cmp::Reverse(d.positions));
    (0..)
        .find(|&time| discs.iter().all(|d| d.does_fall(time)))
        .unwrap()
}

struct Disc {
    id: usize,
    positions: usize,
    start: usize,
}

impl Disc {
    fn does_fall(&self, time: usize) -> bool {
        (self.start + time + self.id).is_multiple_of(self.positions)
    }
}

#[cfg(test)]
mod day15_tests {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            String::from("Disc #1 has 5 positions; at time=0, it is at position 4."),
            String::from("Disc #2 has 2 positions; at time=0, it is at position 1."),
        ]
    }

    #[test]
    fn stage1() {
        let result = result_day15_stage1(&get_example());
        assert_eq!(result, 5);
    }
}
