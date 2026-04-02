use std::{
    char::from_digit,
    io::{self, Write},
};

use rand::prelude::*;

pub fn result_day05_stage1(door_id: &str) -> String {
    find_password(door_id, true)
}

pub fn result_day05_stage2(door_id: &str) -> String {
    find_password(door_id, false)
}

fn find_password(door_id: &str, is_in_order: bool) -> String {
    let mut password = [' '; 8];
    let mut index = 0;
    let mut index_set = [false; 8];
    let mut rng = rand::rng();
    while index_set.iter().any(|i| !i) {
        let input = format!("{}{}", door_id, index);
        let digest = md5::compute(&input);
        // --- Visualizer Section ---
        // Only update the screen every 1000 iterations to keep it fast
        if index % 1000 == 0 {
            print!("\rPassword: ");
            for i in 0..8 {
                if index_set[i] {
                    print!("{}", password[i]);
                } else {
                    // Print a random hex char for the "flicker"
                    let random_hex: char = from_digit(rng.random_range(0..16), 16).unwrap();
                    print!("{}", random_hex);
                }
            }
            io::stdout().flush().unwrap();
        }
        if digest[0] == 0 && digest[1] == 0 && digest[2] < 16 {
            if is_in_order {
                // Part 1
                // Convert the 3rd byte's second half to a hex character
                let hex_char = format!("{:x}", digest[2]);
                let password_index = index_set.iter().filter(|&x| *x).count();
                password[password_index] = hex_char.chars().last().unwrap();
                index_set[password_index] = true;
            } else {
                // Part 2
                // The 6th character is literally just the value of digest[2]
                // because we already checked that the first 4 bits are 0 (digest[2] < 16)
                let password_index = digest[2] as usize;
                if password_index < 8 && !index_set[password_index] {
                    // The 7th character is the "high" part of the 4th byte
                    let val = digest[3] >> 4;
                    // Convert that 0-15 value to a hex character '0'-'f'
                    let hex_char = from_digit(val as u32, 16).unwrap();
                    password[password_index] = hex_char;
                    index_set[password_index] = true;
                }
            }
        }
        index += 1;
    }
    password.iter().collect()
}

#[cfg(test)]
mod day05_tests {
    use super::*;

    #[test]
    fn stage1() {
        let door_id = "abc";
        let result = result_day05_stage1(door_id);
        assert_eq!(result, String::from("18f47a30"));
    }

    #[test]
    fn stage2() {
        let door_id = "abc";
        let result = result_day05_stage2(door_id);
        assert_eq!(result, String::from("05ace8e3"));
    }
}
