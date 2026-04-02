pub fn result_day04_stage1(lines: &[String]) -> u32 {
    lines
        .iter()
        .map(|l| parse_line(l))
        .filter(is_valid)
        .map(|room| room.sector_id)
        .sum()
}

pub fn result_day04_stage2(lines: &[String]) -> String {
    lines
        .iter()
        .map(|l| parse_line(l))
        .filter(is_valid)
        .map(|mut room| {
            room.name = decrypt_name(&room.name, room.sector_id);
            room
        })
        .find(|room| room.name.contains("north"))
        .map(|room| format!("{}: {}", room.name, room.sector_id))
        .expect("Room not found!")
}

struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}

fn is_valid(room: &Room) -> bool {
    // Count frequencies using a fixed-size array
    let mut counts = [0u32; 26];
    for c in room.name.chars().filter(|&c| c != '-') {
        counts[(c as usize) - ('a' as usize)] += 1;
    }

    // Map to (char, count) pairs for sorting
    let mut freq: Vec<(char, u32)> = counts
        .iter()
        .enumerate()
        .filter(|&(_, &count)| count > 0)
        .map(|(i, &count)| ((i as u8 + b'a') as char, count))
        .collect();

    // Sort: primary by count (desc), secondary by char (asc)
    freq.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    // Generate the expected checksum
    let expected: String = freq.into_iter().take(5).map(|(c, _)| c).collect();

    expected == room.checksum
}

fn parse_line(line: &str) -> Room {
    // Find the last '-' to separate name from sector+checksum
    let last_dash = line.rfind('-').unwrap();
    let bracket = line.find('[').unwrap();

    Room {
        name: line[..last_dash].to_string(),
        sector_id: line[last_dash + 1..bracket].parse().unwrap(),
        checksum: line[bracket + 1..line.len() - 1].to_string(),
    }
}

fn decrypt_name(encrypted: &str, shift: u32) -> String {
    let shift_amount = (shift % 26) as u8;
    let decrypted_bytes: Vec<u8> = encrypted
        .bytes()
        .map(|b| {
            if b == b'-' {
                b' '
            } else {
                // Normalise 'a' to 0
                let base = b - b'a';
                // Shift and wrap using modulo
                let shifted = (base + shift_amount) % 26;
                // Convert back to ASCII 'a'-'z'
                shifted + b'a'
            }
        })
        .collect();
    String::from_utf8(decrypted_bytes).unwrap()
}

#[cfg(test)]
mod day04_tests {
    use super::*;

    #[test]
    fn stage1() {
        let lines = vec![
            "aaaaa-bbb-z-y-x-123[abxyz]".to_string(),
            "a-b-c-d-e-f-g-h-987[abcde]".to_string(),
            "not-a-real-room-404[oarel]".to_string(),
            "totally-real-room-200[decoy]".to_string(),
        ];
        let result = result_day04_stage1(&lines);
        assert_eq!(result, 1514);
    }

    #[test]
    fn stage2() {
        let result = decrypt_name(&String::from("qzmt-zixmtkozy-ivhz"), 343);
        assert_eq!(result, String::from("very encrypted name"));
    }
}
