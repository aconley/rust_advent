fn main() -> std::io::Result<()> {
    let inputs: Vec<String> = rust_advent::read_file_as_lines("06")?;
    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
    Ok(())
}

/// Part 1: Homework
///
/// Converts lines into homework problems, then performs the problems.
///
/// The input consists of:
/// - First N lines: each contains M whitespace-separated i32 values
/// - Last line: M whitespace-separated operators ('+' or '*')
///
/// Each column represents a problem. For example, column 0 contains the first
/// number from each line, and the first operator. The result is the sum of all
/// problem results.
fn part1(input: &[String]) -> i64 {
    if input.is_empty() {
        panic!("Input is empty");
    }

    // Separate number lines from operator line
    let num_lines = &input[..input.len() - 1];
    let op_line = &input[input.len() - 1];

    // Validate: N >= 3
    if num_lines.len() < 3 {
        panic!("Must have at least 3 number lines, got {}", num_lines.len());
    }

    // Parse operator line
    let operators: Vec<char> = op_line
        .split_whitespace()
        .map(|s| {
            let op = s.chars().next().expect("Empty operator");
            if op != '+' && op != '*' {
                panic!("Invalid operator: {}", op);
            }
            op
        })
        .collect();

    let m = operators.len();
    if m == 0 {
        panic!("No operators found");
    }

    // Parse number lines
    let mut numbers: Vec<Vec<i32>> = Vec::new();
    for (i, line) in num_lines.iter().enumerate() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| {
                s.parse()
                    .unwrap_or_else(|_| panic!("Invalid number '{}' in line {}", s, i + 1))
            })
            .collect();

        // Validate: each line must have M entries
        if nums.len() != m {
            panic!("Line {} has {} entries, expected {}", i + 1, nums.len(), m);
        }
        numbers.push(nums);
    }

    // Process each problem (column)
    let mut total = 0i64;
    for col in 0..m {
        let mut result = numbers[0][col] as i64;
        let op = operators[col];

        // Apply operator between all numbers in this column
        for row in 1..numbers.len() {
            let num = numbers[row][col] as i64;
            match op {
                '+' => result += num,
                '*' => result *= num,
                _ => panic!("Invalid operator: {}", op),
            }
        }

        total += result;
    }

    total
}

/// Part 2: Homework with vertical digit parsing
///
/// Similar to part 1, but numbers are written vertically (top to bottom)
/// and read right to left. Whitespace is significant and separates columns.
///
/// Each line is split into "cells" separated by whitespace. For each column of problems
/// (right to left), we read character positions from right to left. At each position,
/// we read the character from each row (top to bottom), collecting digits and ignoring
/// spaces. These digits form a number. We then apply the operator between all numbers
/// in that column.
fn part2(input: &[String]) -> i64 {
    if input.is_empty() {
        panic!("Input is empty");
    }

    // Separate number lines from operator line
    let num_lines = &input[..input.len() - 1];
    let op_line = &input[input.len() - 1];

    // Validate: N >= 3
    if num_lines.len() < 3 {
        panic!("Must have at least 3 number lines, got {}", num_lines.len());
    }

    // Parse operators by splitting the operator line on whitespace
    // Each whitespace-separated token should contain one operator
    let operators: Vec<char> = op_line
        .split_whitespace()
        .filter_map(|s| {
            let op = s.chars().next()?;
            if op == '+' || op == '*' {
                Some(op)
            } else {
                None
            }
        })
        .collect();

    if operators.is_empty() {
        panic!("No operators found");
    }

    let num_columns = operators.len();

    // Parse cells: handle the case where '12  35 12' -> '12 ', ' 35', '12'
    // Strategy: split on 2+ consecutive spaces, but preserve single spaces within cells
    let mut cells_by_line: Vec<Vec<String>> = Vec::new();
    for line in num_lines {
        let mut cells = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let cell_start = i;

            // Find the end of this cell
            // A cell ends at: 2+ consecutive spaces, or end of line
            while i < chars.len() {
                if chars[i].is_whitespace() {
                    // Check how many consecutive spaces we have
                    let space_start = i;
                    while i < chars.len() && chars[i].is_whitespace() {
                        i += 1;
                    }
                    let space_count = i - space_start;

                    if space_count >= 2 {
                        // This is a boundary - cell ends before the spaces
                        let cell: String = chars[cell_start..space_start].iter().collect();
                        if !cell.is_empty() || cells.is_empty() {
                            cells.push(cell);
                        }
                        // i is now after all the spaces, ready for next cell
                        break;
                    }
                    // Single space - continue (it's part of the cell or will be handled)
                } else {
                    i += 1;
                }
            }

            // If we reached end of line, add remaining as a cell
            if i >= chars.len() && cell_start < chars.len() {
                let cell: String = chars[cell_start..].iter().collect();
                if !cell.trim().is_empty() || !cells.is_empty() {
                    cells.push(cell);
                }
            }
        }

        // Ensure we have the right number of cells
        while cells.len() < num_columns {
            cells.push(String::new());
        }
        // If we have too many, take the rightmost ones
        if cells.len() > num_columns {
            let start_idx = cells.len() - num_columns;
            cells = cells[start_idx..].to_vec();
        }
        cells_by_line.push(cells);
    }

    // Process each column from right to left
    let mut total = 0i64;
    for col_idx in (0..num_columns).rev() {
        let op = operators[col_idx];

        // Get cells for this column from each line (from the right)
        let mut column_cells = Vec::new();
        for cells in &cells_by_line {
            let cell_idx = cells.len().saturating_sub(1).saturating_sub(col_idx);
            if cell_idx < cells.len() {
                column_cells.push(cells[cell_idx].clone());
            } else {
                column_cells.push(String::new());
            }
        }

        // Find maximum cell length in this column
        let max_cell_len = column_cells.iter().map(|s| s.len()).max().unwrap_or(0);

        // Read character positions from right to left
        // For each position, we collect digits from all rows (top to bottom) to form a number
        let mut numbers = Vec::new();

        for pos in (0..max_cell_len).rev() {
            // Read character at this position from each row (top to bottom)
            let mut digit_sequence = Vec::new();
            for cell in &column_cells {
                if pos < cell.len() {
                    // Get character at this position
                    let ch = cell.chars().nth(pos).unwrap_or(' ');
                    if let Some(digit) = ch.to_digit(10) {
                        digit_sequence.push(digit as u8);
                    }
                    // Ignore spaces - they don't contribute to the number
                }
            }

            // If we found any digits at this position, form a number
            if !digit_sequence.is_empty() {
                // Convert digit sequence to number (reading top to bottom)
                let num = digit_sequence
                    .iter()
                    .fold(0i64, |acc, &d| acc * 10 + d as i64);
                numbers.push(num);
            }
        }

        // Apply operator between all numbers
        if let Some(&first) = numbers.first() {
            let mut result = first;
            for &num in numbers.iter().skip(1) {
                match op {
                    '+' => result += num,
                    '*' => result *= num,
                    _ => panic!("Invalid operator: {}", op),
                }
            }
            total += result;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = vec![
            "5 3 7 2".to_string(),
            "2 1 4 1".to_string(),
            "6 5 1 0".to_string(),
            "* + * *".to_string(),
        ];
        // Problem 0: 5 * 2 * 6 = 60
        // Problem 1: 3 + 1 + 5 = 9
        // Problem 2: 7 * 4 * 1 = 28
        // Problem 3: 2 * 1 * 0 = 0
        // Total: 60 + 9 + 28 + 0 = 97
        assert_eq!(part1(&input), 97);
    }

    #[test]
    fn test_part1_example2() {
        let input = vec![
            "123 328  51 64 ".to_string(),
            " 45 64  387 23 ".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   +  ".to_string(),
        ];
        // Problem 0: 123 * 45 * 6 = 33210
        // Problem 1: 328 + 64 + 98 = 490
        // Problem 2: 51 * 387 * 215 = 4243455
        // Problem 3: 64 + 23 + 314 = 401
        // Total: 33210 + 490 + 4243455 + 401 = 4277556
        assert_eq!(part1(&input), 4277556);
    }

    #[test]
    fn test_part1_minimum_size() {
        // Minimum: N=3, M=1
        let input = vec![
            "5".to_string(),
            "2".to_string(),
            "6".to_string(),
            "*".to_string(),
        ];
        // Problem 0: 5 * 2 * 6 = 60
        assert_eq!(part1(&input), 60);
    }

    #[test]
    fn test_part1_single_column_addition() {
        let input = vec![
            "10".to_string(),
            "20".to_string(),
            "30".to_string(),
            "+".to_string(),
        ];
        // Problem 0: 10 + 20 + 30 = 60
        assert_eq!(part1(&input), 60);
    }

    #[test]
    fn test_part1_single_column_multiplication() {
        let input = vec![
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "*".to_string(),
        ];
        // Problem 0: 2 * 3 * 4 = 24
        assert_eq!(part1(&input), 24);
    }

    #[test]
    fn test_part1_mixed_operators() {
        let input = vec![
            "1 2 3".to_string(),
            "4 5 6".to_string(),
            "7 8 9".to_string(),
            "+ * +".to_string(),
        ];
        // Problem 0: 1 + 4 + 7 = 12
        // Problem 1: 2 * 5 * 8 = 80
        // Problem 2: 3 + 6 + 9 = 18
        // Total: 12 + 80 + 18 = 110
        assert_eq!(part1(&input), 110);
    }

    #[test]
    fn test_part1_large_numbers() {
        let input = vec![
            "1000 2000".to_string(),
            "3000 4000".to_string(),
            "5000 6000".to_string(),
            "* +".to_string(),
        ];
        // Problem 0: 1000 * 3000 * 5000 = 15000000000
        // Problem 1: 2000 + 4000 + 6000 = 12000
        // Total: 15000000000 + 12000 = 15000012000
        assert_eq!(part1(&input), 15000012000);
    }

    #[test]
    fn test_part1_with_zeros() {
        let input = vec![
            "0 5 0".to_string(),
            "10 0 20".to_string(),
            "0 0 0".to_string(),
            "* + *".to_string(),
        ];
        // Problem 0: 0 * 10 * 0 = 0
        // Problem 1: 5 + 0 + 0 = 5
        // Problem 2: 0 * 20 * 0 = 0
        // Total: 0 + 5 + 0 = 5
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn test_part1_with_negative_numbers() {
        let input = vec![
            "-1 2 -3".to_string(),
            "4 -5 6".to_string(),
            "-7 8 -9".to_string(),
            "+ * +".to_string(),
        ];
        // Problem 0: -1 + 4 + (-7) = -4
        // Problem 1: 2 * (-5) * 8 = -80
        // Problem 2: -3 + 6 + (-9) = -6
        // Total: -4 + (-80) + (-6) = -90
        assert_eq!(part1(&input), -90);
    }

    #[test]
    fn test_part1_four_lines() {
        // N=4, M=2
        let input = vec![
            "1 2".to_string(),
            "3 4".to_string(),
            "5 6".to_string(),
            "7 8".to_string(),
            "+ *".to_string(),
        ];
        // Problem 0: 1 + 3 + 5 + 7 = 16
        // Problem 1: 2 * 4 * 6 * 8 = 384
        // Total: 16 + 384 = 400
        assert_eq!(part1(&input), 400);
    }

    #[test]
    fn test_part1_many_columns() {
        // N=3, M=5
        let input = vec![
            "1 2 3 4 5".to_string(),
            "6 7 8 9 10".to_string(),
            "11 12 13 14 15".to_string(),
            "+ + + + +".to_string(),
        ];
        // Problem 0: 1 + 6 + 11 = 18
        // Problem 1: 2 + 7 + 12 = 21
        // Problem 2: 3 + 8 + 13 = 24
        // Problem 3: 4 + 9 + 14 = 27
        // Problem 4: 5 + 10 + 15 = 30
        // Total: 18 + 21 + 24 + 27 + 30 = 120
        assert_eq!(part1(&input), 120);
    }

    #[test]
    fn test_part1_all_multiplication() {
        let input = vec![
            "2 3 4".to_string(),
            "5 6 7".to_string(),
            "8 9 10".to_string(),
            "* * *".to_string(),
        ];
        // Problem 0: 2 * 5 * 8 = 80
        // Problem 1: 3 * 6 * 9 = 162
        // Problem 2: 4 * 7 * 10 = 280
        // Total: 80 + 162 + 280 = 522
        assert_eq!(part1(&input), 522);
    }

    #[test]
    fn test_part1_all_addition() {
        let input = vec![
            "1 10 100".to_string(),
            "2 20 200".to_string(),
            "3 30 300".to_string(),
            "+ + +".to_string(),
        ];
        // Problem 0: 1 + 2 + 3 = 6
        // Problem 1: 10 + 20 + 30 = 60
        // Problem 2: 100 + 200 + 300 = 600
        // Total: 6 + 60 + 600 = 666
        assert_eq!(part1(&input), 666);
    }

    #[test]
    #[ignore]
    fn test_part2_simple() {
        // Simple test case to verify parsing
        let input = vec![
            "64 ".to_string(),
            "23 ".to_string(),
            "431".to_string(),
            "720".to_string(),
            "*".to_string(),
        ];
        // Position 2: (space, space, '1', '0') -> 10
        // Position 1: ('4', '3', '3', '2') -> 4332
        // Position 0: ('6', '2', '4', '7') -> 6247
        // Result: 10 * 4332 * 6247 = 270530040
        let result = part2(&input);
        assert_eq!(result, 270530040);
    }

    #[test]
    #[ignore]
    fn test_part2_example1() {
        let input = vec![
            "64  113".to_string(),
            "23  422".to_string(),
            "431 101".to_string(),
            "720  5 ".to_string(),
            "*   +  ".to_string(),
        ];
        // First problem: ('64 ', '23 ', '431', '720', *)
        //   which is 10 * 4332 * 6247 = 270620040
        // Second problem: ('113', '422', '101', ' 5 ', +)
        //   which is 321 + 1205 + 141 = 1667
        let result = part2(&input);
        // Expected: 270620040 + 1667 = 270621707
        assert_eq!(result, 270621707);
    }

    #[test]
    #[ignore]
    fn test_part2_example2() {
        let input = vec![
            "123 328  51 64 ".to_string(),
            " 45 64  387 23 ".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   +  ".to_string(),
        ];
        // Problems from right to left:
        // 1. (4 431 623 +) -> 4 + 431 + 623 = 1058
        // 2. (175 581 32 *) -> 175 * 581 * 32 = 3253600
        // 3. (8 248 369 +) -> 8 + 248 + 369 = 625
        // 4. (356 24 1 *) -> 356 * 24 * 1 = 8544
        // Total: 1058 + 3253600 + 625 + 8544 = 3266827
        assert_eq!(part2(&input), 3266827);
    }
}
