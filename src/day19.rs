pub fn result_day19_stage1(lines: &[String]) -> usize {
    if lines.len() != 1 {
        panic!("unexpected file input");
    }
    let elf_qty: usize = lines[0].parse().unwrap();
    solve_stage1(elf_qty)
}

pub fn result_day19_stage2(lines: &[String]) -> usize {
    if lines.len() != 1 {
        panic!("unexpected file input");
    }
    let elf_qty: usize = lines[0].parse().unwrap();
    if elf_qty == 1 {
        return 1;
    }
    solve_stage2(elf_qty)
}

fn solve_stage1(n: usize) -> usize {
    let mut p = 1;
    while p * 2 <= n {
        p *= 2;
    }
    let l = n - p;
    (2 * l) + 1
}

fn solve_stage2(n: usize) -> usize {
    let mut p = 1;
    while p * 3 < n {
        p *= 3;
    }
    // 2 different sequences depending on whether n-P <= P or not
    if n - p <= p {
        n - p
    } else {
        p + 2 * (n - 2 * p)
    }
}

#[cfg(test)]
mod day19_tests {
    use super::*;

    #[test]
    fn stage1() {
        let result = solve_stage1(5);
        assert_eq!(result, 3, "failed stage 1. Should be 3, got {}", result);
    }

    #[test]
    fn stage2() {
        let result = solve_stage2(5);
        assert_eq!(result, 2, "failed stage 2. Should be 2, got {}", result);
    }
}
