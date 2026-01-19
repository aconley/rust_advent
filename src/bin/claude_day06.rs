fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = rust_advent::read_file_as_lines("06")?;
    println!("Part 1: {}", part1(&inputs)?);
    println!("Part 2: {}", part2(&inputs)?);
    Ok(())
}

/// Part 1: Homework
///
/// Converts lines into homework problems, then performs the problems.
fn part1(input: &[String]) -> Result<i64, String> {
    // Need at least 3 lines (2 data rows + 1 operator row)
    if input.len() < 3 {
        return Err(format!("Not enough lines: need at least 3, got {}", input.len()));
    }

    // Split into data lines and operator line
    let data_lines = &input[..input.len() - 1];
    let operator_line = &input[input.len() - 1];

    // Parse data lines into numbers
    let mut data: Vec<Vec<i32>> = Vec::new();
    for line in data_lines {
        let numbers: Result<Vec<i32>, _> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect();
        data.push(numbers.map_err(|e| format!("Invalid number: {}", e))?);
    }

    // Check that all lines have the same number of elements (M)
    let m = data[0].len();
    for row in &data {
        if row.len() != m {
            return Err(format!("Inconsistent number of columns: expected {}, got {}", m, row.len()));
        }
    }

    // Parse operators
    let mut operators: Vec<char> = Vec::new();
    for s in operator_line.split_whitespace() {
        let ch = s.chars().next().ok_or_else(|| "Empty operator".to_string())?;
        operators.push(ch);
    }

    if operators.len() != m {
        return Err(format!("Number of operators ({}) doesn't match number of columns ({})", operators.len(), m));
    }

    // Process each column (problem)
    let mut total = 0i64;
    for col_idx in 0..m {
        let operator = operators[col_idx];
        let mut result = data[0][col_idx] as i64;

        // Apply the operator to all values in this column
        for row in data.iter().skip(1) {
            let value = row[col_idx] as i64;
            match operator {
                '+' => result += value,
                '*' => result *= value,
                _ => return Err(format!("Invalid operator: {}", operator)),
            }
        }

        total += result;
    }

    Ok(total)
}

/// Part 2: Vertical Homework
///
/// Numbers are formed by reading character columns vertically (top-to-bottom).
/// Problems are identified by operator positions in the operator row.
/// All input lines are padded to equal length.
/// Column positions are processed right-to-left within each problem's range.
fn part2(input: &[String]) -> Result<i64, String> {
    // Validate input - need at least 3 lines (2 data rows + 1 operator row)
    if input.len() < 3 {
        return Err(format!("Not enough lines: need at least 3, got {}", input.len()));
    }

    // Split into data lines and operator line
    let data_lines = &input[..input.len() - 1];
    let operator_line = &input[input.len() - 1];

    // Find maximum line length and pad all lines to that length
    let max_len = input.iter().map(|line| line.len()).max().unwrap();
    let padded_data: Vec<String> = data_lines
        .iter()
        .map(|line| format!("{:width$}", line, width = max_len))
        .collect();
    let padded_operator = format!("{:width$}", operator_line, width = max_len);

    // Find operator positions to identify problems
    let operator_positions: Vec<(usize, char)> = padded_operator
        .chars()
        .enumerate()
        .filter(|(_, ch)| !ch.is_whitespace())
        .collect();

    if operator_positions.is_empty() {
        return Err("No operators found in operator row".to_string());
    }

    // Validate operators
    for (_, op) in &operator_positions {
        if *op != '+' && *op != '*' {
            return Err(format!("Invalid operator: {}", op));
        }
    }

    // Process each problem
    let mut total = 0i64;

    for (problem_idx, &(operator_pos, operator)) in operator_positions.iter().enumerate() {
        // Determine column range for this problem
        // The operator position marks the START of the problem
        let start_col = operator_pos;
        let end_col = if problem_idx + 1 < operator_positions.len() {
            operator_positions[problem_idx + 1].0 - 1
        } else {
            max_len - 1
        };

        // Collect numbers by reading columns right-to-left
        let mut numbers: Vec<i64> = Vec::new();

        for col_idx in (start_col..=end_col).rev() {
            // Read this column top-to-bottom across all data rows
            let mut digits = String::new();
            for row in &padded_data {
                if let Some(ch) = row.chars().nth(col_idx)
                    && ch.is_ascii_digit() {
                    digits.push(ch);
                }
            }

            // If we found any digits, parse as a number
            if !digits.is_empty() {
                let num = digits.parse::<i64>()
                    .map_err(|e| format!("Failed to parse number '{}': {}", digits, e))?;
                numbers.push(num);
            }
        }

        // Apply operator to all numbers in this problem
        if !numbers.is_empty() {
            let result: i64 = match operator {
                '+' => numbers.iter().copied().sum(),
                '*' => numbers.iter().copied().product(),
                _ => return Err(format!("Invalid operator: {}", operator)),
            };
            total += result;
        }
    }

    Ok(total)
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
        // Column 0: 5 * 2 * 6 = 60
        // Column 1: 3 + 1 + 5 = 9
        // Column 2: 7 * 4 * 1 = 28
        // Column 3: 2 * 1 * 0 = 0
        // Total: 60 + 9 + 28 + 0 = 97
        assert_eq!(part1(&input).unwrap(), 97);
    }

    #[test]
    fn test_part1_example2() {
        let input = vec![
            "123 328  51 64 ".to_string(),
            " 45 64  387 23 ".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   +  ".to_string(),
        ];
        // Column 0: 123 * 45 * 6 = 33210
        // Column 1: 328 + 64 + 98 = 490
        // Column 2: 51 * 387 * 215 = 4243455
        // Column 3: 64 + 23 + 314 = 401
        // Total: 33210 + 490 + 4243455 + 401 = 4277556
        assert_eq!(part1(&input).unwrap(), 4277556);
    }

    #[test]
    fn test_part1_minimum_rows() {
        // Test with exactly 3 lines (minimum: 2 data rows + 1 operator row)
        let input = vec![
            "10 20".to_string(),
            "5 3".to_string(),
            "+ *".to_string(),
        ];
        // Column 0: 10 + 5 = 15
        // Column 1: 20 * 3 = 60
        // Total: 15 + 60 = 75
        assert_eq!(part1(&input).unwrap(), 75);
    }

    #[test]
    fn test_part1_all_addition() {
        let input = vec![
            "1 2 3".to_string(),
            "4 5 6".to_string(),
            "7 8 9".to_string(),
            "+ + +".to_string(),
        ];
        // Column 0: 1 + 4 + 7 = 12
        // Column 1: 2 + 5 + 8 = 15
        // Column 2: 3 + 6 + 9 = 18
        // Total: 12 + 15 + 18 = 45
        assert_eq!(part1(&input).unwrap(), 45);
    }

    #[test]
    fn test_part1_all_multiplication() {
        let input = vec![
            "2 3 4".to_string(),
            "5 2 1".to_string(),
            "3 4 5".to_string(),
            "* * *".to_string(),
        ];
        // Column 0: 2 * 5 * 3 = 30
        // Column 1: 3 * 2 * 4 = 24
        // Column 2: 4 * 1 * 5 = 20
        // Total: 30 + 24 + 20 = 74
        assert_eq!(part1(&input).unwrap(), 74);
    }

    #[test]
    fn test_part1_single_column() {
        let input = vec![
            "10".to_string(),
            "20".to_string(),
            "30".to_string(),
            "+".to_string(),
        ];
        // Column 0: 10 + 20 + 30 = 60
        assert_eq!(part1(&input).unwrap(), 60);
    }

    #[test]
    fn test_part1_with_zeros() {
        let input = vec![
            "0 5 10".to_string(),
            "1 0 2".to_string(),
            "+ + *".to_string(),
        ];
        // Column 0: 0 + 1 = 1
        // Column 1: 5 + 0 = 5
        // Column 2: 10 * 2 = 20
        // Total: 1 + 5 + 20 = 26
        assert_eq!(part1(&input).unwrap(), 26);
    }

    #[test]
    fn test_part1_multiplication_with_zero() {
        let input = vec![
            "5 10".to_string(),
            "0 2".to_string(),
            "* *".to_string(),
        ];
        // Column 0: 5 * 0 = 0
        // Column 1: 10 * 2 = 20
        // Total: 0 + 20 = 20
        assert_eq!(part1(&input).unwrap(), 20);
    }

    #[test]
    fn test_part1_too_few_lines() {
        let input = vec![
            "1 2".to_string(),
            "+ *".to_string(),
        ];
        let result = part1(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Not enough lines"));
    }

    #[test]
    fn test_part1_inconsistent_columns() {
        let input = vec![
            "1 2 3".to_string(),
            "4 5".to_string(),  // Missing one column
            "+ * *".to_string(),
        ];
        let result = part1(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Inconsistent number of columns"));
    }

    #[test]
    fn test_part1_wrong_number_of_operators() {
        let input = vec![
            "1 2 3".to_string(),
            "4 5 6".to_string(),
            "+ *".to_string(),  // Only 2 operators for 3 columns
        ];
        let result = part1(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Number of operators"));
    }

    #[test]
    fn test_part1_invalid_number() {
        let input = vec![
            "1 2 abc".to_string(),
            "4 5 6".to_string(),
            "+ * *".to_string(),
        ];
        let result = part1(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid number"));
    }

    #[test]
    fn test_part1_invalid_operator() {
        let input = vec![
            "1 2 3".to_string(),
            "4 5 6".to_string(),
            "+ - *".to_string(),  // '-' is not a valid operator
        ];
        let result = part1(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid operator"));
    }

    // Part 2 Tests

    #[test]
    fn test_part2_example1() {
        // Example from prompt (lines 11-16)
        let input = vec![
            "64  113".to_string(),
            "23  422".to_string(),
            "431 101".to_string(),
            "720  5".to_string(),
            "*   +".to_string(),
        ];
        // Problem 1 (cols 0-2, operator *):
        //   Col 2: ' ', ' ', '1', '0' -> "10"
        //   Col 1: '4', '3', '3', '2' -> "4332"
        //   Col 0: '6', '2', '4', '7' -> "6247"
        //   Numbers: [10, 4332, 6247]
        //   Result: 10 * 4332 * 6247 = 270620040
        //
        // Problem 2 (cols 4-6, operator +):
        //   Col 6: '3', '2', '1', '5' -> "3215"
        //   Col 5: '1', '2', '0', ' ' -> "120"
        //   Col 4: '1', '4', '1', ' ' -> "141"
        //   Numbers: [3215, 120, 141] (reading right to left)
        //   Wait, let me recalculate...
        //   Actually, looking at the positions:
        //   "64  113"
        //   Position 0='6', 1='4', 2=' ', 3=' ', 4='1', 5='1', 6='3'
        //   Operator row: "*   +"
        //   Position 0='*', 1=' ', 2=' ', 3=' ', 4='+'
        //
        //   Operators at: pos 0 (*), pos 4 (+)
        //   Problem 1: cols 0-3 (from start to pos 0)
        //   Wait, that's not right either. Let me re-read the plan...
        //
        //   The operator position defines the END of the problem.
        //   Problem 1: cols 0-0 (start=0, end=0, operator at 0)
        //   Problem 2: cols 1-4 (start=1, end=4, operator at 4)
        //
        //   Hmm, this doesn't make sense. Let me look at the example more carefully.
        //
        //   From the plan: "For each operator at position i:
        //     - Start column: previous_operator_position + 1 (or 0 for first)
        //     - End column: current operator position (inclusive)"
        //
        //   So if operators are at positions 0 and 4:
        //   - Problem 1: start=0, end=0 (only column 0)
        //   - Problem 2: start=1, end=4 (columns 1-4)
        //
        //   But looking at the input "64  113", the first problem should be "64 " and the
        //   second should be " 113". Let me check the expected output from the plan.
        //
        //   Expected: 270620040 + 1667 = 270621707
        //
        //   Let me work backwards. If problem 1 = 270620040:
        //   270620040 = 10 * 4332 * 6247
        //
        //   So we need numbers [10, 4332, 6247] or some permutation.
        //   Reading "64 ", "23 ", "431", "720" vertically:
        //   Col 0: '6','2','4','7' -> "6247"
        //   Col 1: '4','3','3','2' -> "4332"
        //   Col 2: ' ',' ','1','0' -> "10"
        //
        //   Right-to-left: 10, 4332, 6247
        //   10 * 4332 * 6247 = 270620040 ✓
        //
        //   So problem 1 uses columns 0-2, with operator at position...
        //   Looking at "*   +", the * is at position 0 and + is at position 4.
        //
        //   Wait, that means the operator position doesn't define the END of the problem,
        //   it defines the START or maybe something else.
        //
        //   Let me look at "64  113":
        //   - '6' at pos 0
        //   - '4' at pos 1
        //   - ' ' at pos 2
        //   - ' ' at pos 3
        //   - '1' at pos 4
        //   - '1' at pos 5
        //   - '3' at pos 6
        //
        //   And "*   +":
        //   - '*' at pos 0
        //   - ' ' at pos 1
        //   - ' ' at pos 2
        //   - ' ' at pos 3
        //   - '+' at pos 4
        //
        //   So we have 2 operators at positions 0 and 4.
        //
        //   From the example, problem 1 uses "64 " (positions 0-2).
        //   Problem 2 uses " 113" (positions 4-6).
        //
        //   So the operator at position 0 corresponds to columns ending at position... 3?
        //   And the operator at position 4 corresponds to columns 4-6.
        //
        //   Actually, looking at the original problem description more carefully:
        //   The problems are separated by whitespace. The operator position tells us
        //   where the operator is, and we need to figure out which columns belong to
        //   which problem.
        //
        //   From the plan step 2:
        //   "For each operator at position i:
        //     - Start column: previous_operator_position + 1 (or 0 for first)
        //     - End column: current operator position (inclusive)"
        //
        //   Wait, but that would give us:
        //   - Operator at 0: start=0, end=0 (only column 0)
        //   - Operator at 4: start=1, end=4 (columns 1-4)
        //
        //   That doesn't match. Let me re-read the problem more carefully...
        //
        //   Actually, I think I misunderstood. Let me look at the example walkthrough
        //   in the plan:
        //
        //   ```
        //   Input (padded to length 15):
        //   Position: 012345678901234
        //   Row 0:    123 328  51 64
        //   Row 1:     45 64  387 23
        //   Row 2:      6 98  215 314
        //   Operator: *   +   *   +
        //   ```
        //
        //   Operators at positions: 0(*), 4(+), 8(*), 12(+)
        //
        //   Problem ranges:
        //   - Problem 1: cols 0-3, operator *
        //
        //   Wait, the operator is at position 0, but the problem uses cols 0-3?
        //   So the operator position marks the START of the problem, and the problem
        //   extends to the position before the next operator?
        //
        //   Actually no, looking more carefully:
        //   "123 " - positions 0-3
        //   Operator at position 0.
        //   So the operator is at the FIRST position of the problem.
        //
        //   Then:
        //   - Problem 1: operator at 0, range 0 to 3 (before next operator at 4)
        //   - Problem 2: operator at 4, range 4 to 7 (before next operator at 8)
        //   - Problem 3: operator at 8, range 8 to 11 (before next operator at 12)
        //   - Problem 4: operator at 12, range 12 to 14 (end of string)
        //
        //   So the algorithm should be:
        //   - For each operator at position i:
        //     - Start: i
        //     - End: next operator position - 1, or end of string
        //
        //   Let me update my implementation to match this.

        // For now, let me just verify with the expected result
        assert_eq!(part2(&input).unwrap(), 270621707);
    }

    #[test]
    fn test_part2_example2() {
        // Example from prompt (lines 34-37)
        let input = vec![
            "123 328  51 64 ".to_string(),
            " 45 64  387 23 ".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   +  ".to_string(),
        ];
        // Expected: 1058 + 3253600 + 625 + 8544 = 3263827
        assert_eq!(part2(&input).unwrap(), 3263827);
    }

    #[test]
    fn test_part2_simple_single_problem() {
        let input = vec![
            "123".to_string(),
            "456".to_string(),
            "+".to_string(),
        ];
        // Single problem with operator at position 0
        // Columns 0-2 (assuming it extends to end of string)
        // Col 2: '3', '6' -> "36"
        // Col 1: '2', '5' -> "25"
        // Col 0: '1', '4' -> "14"
        // Right-to-left: [36, 25, 14]
        // Sum: 36 + 25 + 14 = 75
        assert_eq!(part2(&input).unwrap(), 75);
    }

    #[test]
    fn test_part2_all_addition() {
        let input = vec![
            "12 34".to_string(),
            "56 78".to_string(),
            "+  + ".to_string(),
        ];
        // Two problems, both with addition
        assert!(part2(&input).is_ok());
    }

    #[test]
    fn test_part2_all_multiplication() {
        let input = vec![
            "12 34".to_string(),
            "56 78".to_string(),
            "*  * ".to_string(),
        ];
        // Two problems, both with multiplication
        assert!(part2(&input).is_ok());
    }

    #[test]
    fn test_part2_mixed_operators() {
        let input = vec![
            "12 34".to_string(),
            "56 78".to_string(),
            "*  + ".to_string(),
        ];
        // Two problems with different operators
        assert!(part2(&input).is_ok());
    }

    #[test]
    fn test_part2_sparse_columns() {
        let input = vec![
            "1  2".to_string(),
            "3  4".to_string(),
            "+  *".to_string(),
        ];
        // Some columns have no digits (whitespace only)
        assert!(part2(&input).is_ok());
    }

    #[test]
    fn test_part2_single_digit_numbers() {
        let input = vec![
            "1 2 3".to_string(),
            "4 5 6".to_string(),
            "+ * +".to_string(),
        ];
        // Each column yields single digit
        assert!(part2(&input).is_ok());
    }

    #[test]
    fn test_part2_too_few_lines() {
        let input = vec![
            "123".to_string(),
            "+".to_string(),
        ];
        let result = part2(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Not enough lines"));
    }

    #[test]
    fn test_part2_no_operators() {
        let input = vec![
            "123".to_string(),
            "456".to_string(),
            "   ".to_string(),  // Only whitespace
        ];
        let result = part2(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No operators found"));
    }

    #[test]
    fn test_part2_invalid_operator() {
        let input = vec![
            "123".to_string(),
            "456".to_string(),
            "-".to_string(),  // Invalid operator
        ];
        let result = part2(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid operator"));
    }
}
