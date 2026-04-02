pub fn result_day08_stage1(lines: &[String]) -> usize {
    let mut screen = init_screen(50, 6);
    process_instructions(lines, &mut screen);

    screen.iter().flatten().filter(|&&x| x).count()
}

pub fn result_day08_stage2(lines: &[String]) -> usize {
    let mut screen = init_screen(50, 6);
    process_instructions(lines, &mut screen);

    for row in &screen {
        for (i, &px) in row.iter().enumerate() {
            print!("{}", if px { '#' } else { '.' });
            if (i + 1) % 5 == 0 {
                print!("  ");
            }
        }
        println!()
    }
    0
}

fn init_screen(width: usize, height: usize) -> Vec<Vec<bool>> {
    // Default lights to be off (false)
    vec![vec![false; width]; height]
}

fn process_instructions(lines: &[String], screen: &mut [Vec<bool>]) {
    for line in lines {
        let mut parts = line.split_whitespace();
        let command = parts.next().unwrap();
        match command {
            "rect" => {
                let dims = parts.next().expect("rect requires dimensions");
                rect(dims, screen);
            }
            "rotate" => {
                let axis = parts.next().unwrap(); // row / column
                let line = parts.next().unwrap();
                parts.next(); // skip "by"
                let amount: usize = parts.next().unwrap().parse().unwrap();
                let line_num: usize = line.split_once("=").unwrap().1.parse().unwrap();
                match axis {
                    "row" => rotate_row(line_num, amount, screen),
                    "column" => rotate_col(line_num, amount, screen),
                    _ => panic!("invalid rotate axis"),
                }
            }
            _ => panic!("unknown command"),
        }
    }
}

fn rect(dims: &str, screen: &mut [Vec<bool>]) {
    let (cols_str, rows_str) = dims.split_once("x").unwrap();
    let cols: usize = cols_str.parse().unwrap();
    let rows: usize = rows_str.parse().unwrap();
    for row in screen.iter_mut().take(rows) {
        for px in row.iter_mut().take(cols) {
            *px = true;
        }
    }
}

fn rotate_row(row: usize, amount: usize, screen: &mut [Vec<bool>]) {
    screen[row].rotate_right(amount);
}

fn rotate_col(column: usize, amount: usize, screen: &mut [Vec<bool>]) {
    let height = screen.len();
    let mut col: Vec<_> = (0..height).map(|y| screen[y][column]).collect();
    col.rotate_right(amount);

    for y in 0..height {
        screen[y][column] = col[y];
    }
}

#[cfg(test)]
mod day08_tests {
    use super::*;

    #[test]
    fn stage1() {
        let mut screen = init_screen(7, 3);
        let instructions = vec![
            String::from("rect 3x2"),
            String::from("rotate column x=1 by 1"),
            String::from("rotate row y=0 by 4 "),
            String::from("rotate column x=1 by 1"),
        ];
        process_instructions(&instructions, &mut screen);
        let expected = vec![
            vec![false, true, false, false, true, false, true],
            vec![true, false, true, false, false, false, false],
            vec![false, true, false, false, false, false, false],
        ];
        assert_eq!(screen, expected);
        let result = screen.iter().flatten().filter(|&x| *x).count();
        assert_eq!(result, 6);
    }
}
