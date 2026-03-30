use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

pub fn result_day11_stage1(lines: &[String]) -> usize {
    let state = State::parse_layout(lines);
    solve_bfs(state).unwrap_or(usize::MAX)
}

pub fn result_day11_stage2(lines: &[String]) -> usize {
    let mut state = State::parse_layout(lines);
    // Add the extra 4 items (2 pairs) to the first floor
    state.pairs.push(ItemPair {
        generator_floor: 0,
        microchip_floor: 0,
    });
    state.pairs.push(ItemPair {
        generator_floor: 0,
        microchip_floor: 0,
    });
    state.pairs.sort();
    solve_bfs(state).unwrap_or(usize::MAX)
}

fn solve_bfs(start_state: State) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    // Canonicalise the start
    let mut start = start_state;
    start.pairs.sort();

    queue.push_back((start.clone(), 0));
    visited.insert(start);

    while let Some((current, dist)) = queue.pop_front() {
        if current.is_goal() {
            return Some(dist);
        }

        for next in current.get_next_states() {
            if visited.insert(next.clone()) {
                queue.push_back((next, dist + 1));
            }
        }
    }
    None
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Copy, PartialOrd, Ord)]
struct ItemPair {
    generator_floor: usize,
    microchip_floor: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    elevator: usize,
    pairs: Vec<ItemPair>,
}

impl State {
    fn parse_layout(lines: &[String]) -> Self {
        let mut materials: HashMap<&str, ItemPair> = HashMap::new();
        for line in lines {
            if line.contains("contains nothing relevant") {
                continue;
            }
            let words: Vec<&str> = line.split_whitespace().collect();
            let floor: usize = match words[1] {
                "first" => 0,
                "second" => 1,
                "third" => 2,
                "fourth" => 3,
                _ => panic!("Unknown floor number"),
            };
            let words = &words[5..];
            for i in 0..words.len() {
                if words[i] == "microchip" || words[i] == "microchip." || words[i] == "microchip," {
                    let material = words[i - 1].trim_end_matches("-compatible");
                    materials.entry(material).or_default().microchip_floor = floor;
                } else if words[i] == "generator"
                    || words[i] == "generator."
                    || words[i] == "generator,"
                {
                    let material = words[i - 1];
                    materials.entry(material).or_default().generator_floor = floor;
                }
            }
        }
        let mut pairs: Vec<ItemPair> = materials.values().copied().collect();
        pairs.sort();
        State { elevator: 0, pairs }
    }

    fn is_valid(&self) -> bool {
        // Quick look-up: which floors have at least one generator?
        let mut has_generator = [false; 4];
        for pair in self.pairs.iter() {
            has_generator[pair.generator_floor] = true;
        }
        for pair in self.pairs.iter() {
            // If a chip is orphaned (not with its generator)
            if pair.microchip_floor != pair.generator_floor {
                // if there is a generator on this floor: unsafe
                if has_generator[pair.microchip_floor] {
                    return false;
                }
            }
        }
        true
    }

    fn is_goal(&self) -> bool {
        self.elevator == 3
            && self
                .pairs
                .iter()
                .all(|&pair| pair.generator_floor == 3 && pair.microchip_floor == 3)
    }

    fn get_next_states(&self) -> Vec<State> {
        let mut items = Vec::new();
        let mut next_states = Vec::new();
        for (i, pair) in self.pairs.iter().enumerate() {
            if pair.generator_floor == self.elevator {
                items.push((i, true));
            }
            if pair.microchip_floor == self.elevator {
                items.push((i, false));
            }
        }
        let combos_2 = items.iter().combinations(2);
        let combos_1 = items.iter().combinations(1);
        for combo in combos_2.chain(combos_1) {
            for elevator_change in [-1, 1] {
                let target_floor = self.elevator as isize + elevator_change;
                if !(0..=3).contains(&target_floor) {
                    continue;
                }
                let target_floor = target_floor as usize;
                let mut new_state = self.clone();
                new_state.elevator = target_floor;
                for &&(idx, is_gen) in combo.iter() {
                    if is_gen {
                        new_state.pairs[idx].generator_floor = target_floor;
                    } else {
                        new_state.pairs[idx].microchip_floor = target_floor;
                    }
                }
                new_state.pairs.sort();
                if new_state.is_valid() {
                    next_states.push(new_state);
                }
            }
        }
        next_states
    }
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            String::from(
                "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.",
            ),
            String::from("The second floor contains a hydrogen generator."),
            String::from("The third floor contains a lithium generator."),
            String::from("The fourth floor contains nothing relevant."),
        ]
    }

    #[test]
    fn parse_day11() {
        let lines = get_example();
        let state = State::parse_layout(&lines);
        let expected = State {
            elevator: 0,
            pairs: vec![
                ItemPair {
                    generator_floor: 1,
                    microchip_floor: 0,
                }, // Hydrogen
                ItemPair {
                    generator_floor: 2,
                    microchip_floor: 0,
                }, // Lithium
            ],
        };
        assert_eq!(state, expected);
    }

    #[test]
    fn stage1() {
        let result = result_day11_stage1(&get_example());
        assert_eq!(result, 11);
    }
}
