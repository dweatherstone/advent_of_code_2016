use std::collections::HashMap;

pub fn result_day10_stage1(lines: &[String], compare_low: u32, compare_high: u32) -> usize {
    let mut factory = Factory::parse_input(lines);
    while let Some(bot_id) = factory.ready_queue.pop() {
        let (low_val, high_val, low_to, high_to) = {
            let bot = factory.bots.get_mut(&bot_id).unwrap();
            bot.chips.sort();
            let l = bot.chips[0];
            let h = bot.chips[1];
            bot.chips.clear();

            if l == compare_low && h == compare_high {
                return bot_id;
            }

            (
                l,
                h,
                bot.low_to.clone().unwrap(),
                bot.high_to.clone().unwrap(),
            )
        };

        factory.hand_off(low_val, low_to);
        factory.hand_off(high_val, high_to);
    }
    usize::MAX
}

pub fn result_day10_stage2(lines: &[String]) -> u32 {
    let mut factory = Factory::parse_input(lines);
    while let Some(bot_id) = factory.ready_queue.pop() {
        let (low_val, high_val, low_to, high_to) = {
            let bot = factory.bots.get_mut(&bot_id).unwrap();
            bot.chips.sort();
            let l = bot.chips[0];
            let h = bot.chips[1];
            bot.chips.clear();
            (
                l,
                h,
                bot.low_to.clone().unwrap(),
                bot.high_to.clone().unwrap(),
            )
        };

        factory.hand_off(low_val, low_to);
        factory.hand_off(high_val, high_to);
    }
    (0..3)
        .map(|idx| factory.outputs.get(&idx).unwrap_or(&1))
        .product()
}

#[derive(Debug, PartialEq)]
struct Factory {
    bots: HashMap<usize, Bot>,
    outputs: HashMap<usize, u32>,
    ready_queue: Vec<usize>,
}

impl Factory {
    fn parse_input(lines: &[String]) -> Self {
        let mut factory = Factory {
            bots: HashMap::new(),
            outputs: HashMap::new(),
            ready_queue: Vec::new(),
        };

        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts[0] {
                "value" => {
                    let val = parts[1].parse::<u32>().unwrap();
                    let bot_id = parts[5].parse::<usize>().unwrap();

                    let bot = factory.bots.entry(bot_id).or_default();
                    bot.chips.push(val);
                    if bot.chips.len() == 2 {
                        factory.ready_queue.push(bot_id);
                    }
                }
                "bot" => {
                    let bot_id = parts[1].parse::<usize>().unwrap();

                    // Helper to parse the (type, id) pairs
                    let parse_dest = |type_str: &str, id_str: &str| {
                        let id = id_str.parse::<usize>().unwrap();
                        if type_str == "bot" {
                            Destination::Bot(id)
                        } else {
                            Destination::Output(id)
                        }
                    };

                    let low_to = parse_dest(parts[5], parts[6]);
                    let high_to = parse_dest(parts[10], parts[11]);

                    let bot = factory.bots.entry(bot_id).or_default();
                    bot.low_to = Some(low_to);
                    bot.high_to = Some(high_to);

                    if bot.chips.len() == 2 {
                        factory.ready_queue.push(bot_id);
                    }
                }
                _ => panic!("unknown command"),
            }
        }
        factory
    }

    fn hand_off(&mut self, chip: u32, to: Destination) {
        match to {
            Destination::Bot(id) => {
                let bot = self.bots.entry(id).or_default();
                bot.chips.push(chip);

                if bot.chips.len() == 2 {
                    self.ready_queue.push(id);
                }
            }
            Destination::Output(id) => {
                self.outputs.insert(id, chip);
            }
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
struct Bot {
    chips: Vec<u32>, // Max length will be 2
    low_to: Option<Destination>,
    high_to: Option<Destination>,
}

#[derive(Clone, Debug, PartialEq)]
enum Destination {
    Bot(usize),
    Output(usize),
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            String::from("value 5 goes to bot 2"),
            String::from("bot 2 gives low to bot 1 and high to bot 0"),
            String::from("value 3 goes to bot 1"),
            String::from("bot 1 gives low to output 1 and high to bot 0"),
            String::from("bot 0 gives low to output 2 and high to output 0"),
            String::from("value 2 goes to bot 2"),
        ]
    }

    #[test]
    fn test_parse() {
        let expected = Factory {
            bots: HashMap::from_iter([
                (
                    0,
                    Bot {
                        chips: vec![],
                        low_to: Some(Destination::Output(2)),
                        high_to: Some(Destination::Output(0)),
                    },
                ),
                (
                    1,
                    Bot {
                        chips: vec![3],
                        low_to: Some(Destination::Output(1)),
                        high_to: Some(Destination::Bot(0)),
                    },
                ),
                (
                    2,
                    Bot {
                        chips: vec![5, 2],
                        low_to: Some(Destination::Bot(1)),
                        high_to: Some(Destination::Bot(0)),
                    },
                ),
            ]),
            outputs: HashMap::new(),
            ready_queue: vec![2],
        };
        let factory = Factory::parse_input(&get_example());
        assert_eq!(factory, expected);
    }

    #[test]
    fn stage1() {
        let result = result_day10_stage1(&get_example(), 2, 5);
        assert_eq!(result, 2);
    }
}
