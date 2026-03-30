use std::collections::HashMap;

pub fn result_day14_stage1(lines: &[String]) -> usize {
    let salt = &lines[0];
    get_64th_key(salt, 1)
}

pub fn result_day14_stage2(lines: &[String]) -> usize {
    let salt = &lines[0];
    get_64th_key(salt, 2017)
}

fn get_64th_key(salt: &str, md5_qty: i32) -> usize {
    let mut index: usize = 0;
    let mut keys_found = 0;
    let mut cache = HashMap::new();
    loop {
        let hash = get_hash(salt, index, md5_qty, &mut cache);

        if let Some(triplet_char) = find_first_triplet(&hash) {
            let quint = triplet_char.to_string().repeat(5);

            // Check the next 1000 hashes
            for next_index in (index + 1)..=(index + 1000) {
                let next_hash = get_hash(salt, next_index, md5_qty, &mut cache);
                if next_hash.contains(&quint) {
                    keys_found += 1;
                    if keys_found == 64 {
                        return index;
                    }
                    break; // Stop checking quintuplets for this index    
                }
            }
        }
        index += 1;
    }
}

fn get_hash(salt: &str, index: usize, md5_qty: i32, cache: &mut HashMap<usize, String>) -> String {
    cache
        .entry(index)
        .or_insert_with(|| {
            let mut hash = format!("{:x}", md5::compute(format!("{}{}", salt, index)));
            for _ in 0..md5_qty - 1 {
                hash = format!("{:x}", md5::compute(hash));
            }
            hash
        })
        .clone()
}

fn find_first_triplet(hash: &str) -> Option<char> {
    let bytes = hash.as_bytes();

    for window in bytes.windows(3) {
        if window[0] == window[1] && window[0] == window[2] {
            return Some(window[0] as char);
        }
    }
    None
}

#[cfg(test)]
mod day14_tests {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![String::from("abc")]
    }

    #[test]
    fn stage1() {
        let result = result_day14_stage1(&get_example());
        assert_eq!(result, 22728);
    }

    #[test]
    fn stage2() {
        let result = result_day14_stage2(&get_example());
        assert_eq!(result, 22551);
    }
}
