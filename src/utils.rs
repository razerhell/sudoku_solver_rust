pub fn str_to_vecu8(sudoku: &str) -> Vec<u8> {
    String::from(sudoku)
        .chars()
        .map(|c| { (if let Some(n) = c.to_digit(10) { n } else { 0 }) as u8 })
        .collect()
}

pub fn vecu8_to_str(vec: &Vec<u8>) -> String {
    // implemented via String::from_iter()
    String::from_iter(
        vec.iter().map(|&n| {
            match n {
                0 => '.',
                _ => char::from_digit(n as u32, 10).unwrap(),
            }
        })
    )

    // // implemented via Vec::fold()
    // vec.iter().fold(String::new(), |mut acc, &n| {
    //     acc.push(match n {
    //         0 => '.',
    //         _ => char::from_digit(n as u32, 10).unwrap(),
    //     });
    //     acc
    // })
}

pub fn show(sudoku: &str) {
    println!("+------+------+------+");
    for (i, c) in sudoku.chars().enumerate() {
        if i % 9 == 0 {
            if i != 0 {
                println!("");
                if i % 27 == 0 {
                    println!("+------+------+------+");
                }
            }
            print!("|");
        }

        print!(" {}", c);
        if (i + 1) % 3 == 0 {
            print!("|");
        }
    }

    println!("\n+------+------+------+");
}

pub fn same_row_index(i: usize) -> Vec<usize> {
    let row_head = (i / 9) * 9;
    (row_head..row_head + 9).collect()
}

pub fn same_col_index(i: usize) -> Vec<usize> {
    let col_head = i % 9;
    (col_head..81).step_by(9).collect()
}

pub fn same_block_index(i: usize) -> Vec<usize> {
    let y = i % 9;
    let y = (y / 3) * 3;
    let x = i / 9;
    let x = (x / 3) * 3;
    let res: Vec<usize> = (x * 9 + y..x * 9 + y + 27).step_by(9).collect();
    let res = res
        .iter()
        .flat_map(|&n| {
            let vec: Vec<usize> = (n..n + 3).collect();
            vec
        })
        .collect();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_vecu8() {
        let input =
            ".5..83.17...1..4..3.4..56.8....3...9.9.8245....6....7...9....5...729..861.36.72.4";

        let expected_output: Vec<u8> = vec![
            0,
            5,
            0,
            0,
            8,
            3,
            0,
            1,
            7,
            0,
            0,
            0,
            1,
            0,
            0,
            4,
            0,
            0,
            3,
            0,
            4,
            0,
            0,
            5,
            6,
            0,
            8,
            0,
            0,
            0,
            0,
            3,
            0,
            0,
            0,
            9,
            0,
            9,
            0,
            8,
            2,
            4,
            5,
            0,
            0,
            0,
            0,
            6,
            0,
            0,
            0,
            0,
            7,
            0,
            0,
            0,
            9,
            0,
            0,
            0,
            0,
            5,
            0,
            0,
            0,
            7,
            2,
            9,
            0,
            0,
            8,
            6,
            1,
            0,
            3,
            6,
            0,
            7,
            2,
            0,
            4
        ];

        assert_eq!(str_to_vecu8(&input), expected_output)
    }

    #[test]
    fn test_vecu8_to_str() {
        let input: Vec<u8> = vec![
            0,
            5,
            0,
            0,
            8,
            3,
            0,
            1,
            7,
            0,
            0,
            0,
            1,
            0,
            0,
            4,
            0,
            0,
            3,
            0,
            4,
            0,
            0,
            5,
            6,
            0,
            8,
            0,
            0,
            0,
            0,
            3,
            0,
            0,
            0,
            9,
            0,
            9,
            0,
            8,
            2,
            4,
            5,
            0,
            0,
            0,
            0,
            6,
            0,
            0,
            0,
            0,
            7,
            0,
            0,
            0,
            9,
            0,
            0,
            0,
            0,
            5,
            0,
            0,
            0,
            7,
            2,
            9,
            0,
            0,
            8,
            6,
            1,
            0,
            3,
            6,
            0,
            7,
            2,
            0,
            4
        ];

        let expected_output =
            ".5..83.17...1..4..3.4..56.8....3...9.9.8245....6....7...9....5...729..861.36.72.4";

        assert_eq!(vecu8_to_str(&input), expected_output)
    }

    #[test]
    fn test_same_row_index() {
        let input: usize = 0;
        let expected_output = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(same_row_index(input), expected_output);

        let input: usize = 79;
        let expected_output = vec![72, 73, 74, 75, 76, 77, 78, 79, 80];
        assert_eq!(same_row_index(input), expected_output)
    }

    #[test]
    fn test_same_col_index() {
        let input: usize = 0;
        let expected_output = vec![0, 9, 18, 27, 36, 45, 54, 63, 72];
        assert_eq!(same_col_index(input), expected_output);

        let input: usize = 77;
        let expected_output = vec![5, 14, 23, 32, 41, 50, 59, 68, 77];
        assert_eq!(same_col_index(input), expected_output)
    }

    #[test]
    fn test_same_block_index() {
        let input: usize = 41;
        let expected_output = vec![30, 31, 32, 39, 40, 41, 48, 49, 50];
        assert_eq!(same_block_index(input), expected_output);

        let input: usize = 77;
        let expected_output = vec![57, 58, 59, 66, 67, 68, 75, 76, 77];
        assert_eq!(same_block_index(input), expected_output);

        let input: usize = 70;
        let expected_output = vec![60, 61, 62, 69, 70, 71, 78, 79, 80];
        assert_eq!(same_block_index(input), expected_output)
    }
}
