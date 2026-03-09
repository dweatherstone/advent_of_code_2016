pub fn result_day03_stage1(lines: &[String]) -> usize {
    let mut triangles: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    triangles.iter_mut().for_each(|triangle| triangle.sort());

    triangles
        .iter()
        .filter(|triangle| triangle[0] + triangle[1] > triangle[2])
        .count()
}

pub fn restult_day03_stage2(lines: &[String]) -> usize {
    // let mut triangles = Vec::new();
    // for three_lines in lines.chunks_exact(3) {
    //     let mut triangle1 = Vec::new();
    //     let mut triangle2 = Vec::new();
    //     let mut triangle3 = Vec::new();
    //     for line in three_lines {
    //         let values: Vec<u32> = line
    //             .split_whitespace()
    //             .map(|s| s.parse().unwrap())
    //             .collect();
    //         if values.len() != 3 {
    //             panic!("something went wrong with parsing the line into 3 numbers");
    //         }
    //         triangle1.push(values[0]);
    //         triangle2.push(values[1]);
    //         triangle3.push(values[2]);
    //     }
    //     triangles.push(triangle1);
    //     triangles.push(triangle2);
    //     triangles.push(triangle3);
    // }

    // triangles.iter_mut().for_each(|triangle| triangle.sort());

    // triangles
    //     .iter()
    //     .filter(|triangle| triangle[0] + triangle[1] > triangle[2])
    //     .count()

    lines
        .chunks(3)
        .flat_map(|chunk| {
            // Parse each of the 3 lines into a Vec<u32>
            let rows: Vec<Vec<u32>> = chunk
                .iter()
                .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
                .collect();

            // Perform the "Transpose": Turn 3 rows of 3 into 3 triangles
            (0..3).map(move |col| {
                let mut tri = vec![rows[0][col], rows[1][col], rows[2][col]];
                tri.sort();
                tri
            })
        })
        .filter(|tri| tri[0] + tri[1] > tri[2])
        .count()
}

#[cfg(test)]
mod day03 {
    use super::*;

    #[test]
    fn stage1() {
        let tests = vec![
            (["5 10 25".to_string()], 0),
            (["5 10 13".to_string()], 1),
            (["1 2 5".to_string()], 0),
            (["25 5 10".to_string()], 0),
            (["3 10 8".to_string()], 1),
        ];
        for (input, expected) in tests {
            let result = result_day03_stage1(&input);
            assert_eq!(result, expected);
        }
    }
}
