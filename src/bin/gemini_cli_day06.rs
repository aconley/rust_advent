use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let inputs = rust_advent::read_file_as_lines("06")?;

    match part1(&inputs) {
        Ok(result) => println!("Part 1: {}", result),
        Err(e) => eprintln!("Part 1 Error: {}", e),
    }

    println!("Part 2: {}", part2(&inputs));
    Ok(())
}

/// Part 1: Homework
///
/// Converts lines into homework problems, then performs the problems.
fn part1(input: &[String]) -> Result<i64, String> {
    if input.is_empty() {
        return Err("Input is empty".to_string());
    }

    // N lines of numbers, 1 line of operators. N >= 3.
    // So minimum 4 lines.
    if input.len() < 4 {
        return Err(
            "Input must have at least 3 lines of numbers and 1 line of operators".to_string(),
        );
    }

    let operator_line = &input[input.len() - 1];
    let number_lines = &input[..input.len() - 1];

    // Parse operators first to determine M and operations
    let operators: Vec<&str> = operator_line.split_whitespace().collect();
    let m = operators.len();

    if m == 0 {
        return Err("Operator line must contain at least one operator".to_string());
    }

    // Initialize accumulators with the values from the first line of numbers
    let first_line_tokens: Vec<&str> = number_lines[0].split_whitespace().collect();
    if first_line_tokens.len() != m {
        return Err(format!(
            "Line 1 has {} numbers, expected {}",
            first_line_tokens.len(),
            m
        ));
    }

    let mut accumulators: Vec<i64> = Vec::with_capacity(m);
    for (_, token) in first_line_tokens.iter().enumerate() {
        let num = token
            .parse::<i64>()
            .map_err(|_| format!("Invalid number '{}' at line 1", token))?;
        accumulators.push(num);
    }

    // Process remaining lines
    for (line_idx, line) in number_lines.iter().enumerate().skip(1) {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() != m {
            return Err(format!(
                "Line {} has {} numbers, expected {}",
                line_idx + 1,
                tokens.len(),
                m
            ));
        }

        for (col_idx, token) in tokens.iter().enumerate() {
            let num = token
                .parse::<i64>()
                .map_err(|_| format!("Invalid number '{}' at line {}", token, line_idx + 1))?;

            let op = operators[col_idx];
            match op {
                "+" => accumulators[col_idx] += num,
                "*" => accumulators[col_idx] *= num,
                _ => return Err(format!("Unknown operator: {}", op)),
            }
        }
    }

    Ok(accumulators.iter().sum())
}

fn part2(input: &[String]) -> i64 {
    // 1. Validation
    if input.len() < 4 {
        panic!("Input must have at least 3 lines of numbers and 1 line of operators");
    }

    // 2. Pad Lines
    let max_len = input.iter().map(|s| s.len()).max().unwrap_or(0);
    let padded_input: Vec<Vec<char>> = input
        .iter()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.resize(max_len, ' ');
            chars
        })
        .collect();

    let num_rows = padded_input.len() - 1;
    let operator_row = &padded_input[padded_input.len() - 1];

    // 3. Find Problems (Operators)
    let mut problem_starts = Vec::new();
    for (idx, &ch) in operator_row.iter().enumerate() {
        if ch == '+' || ch == '*' {
            problem_starts.push(idx);
        }
    }

    let mut total_sum: i64 = 0;

    // 4. Process Each Problem
    for &start_col in &problem_starts {
        let op_char = operator_row[start_col];

        // Determine end column (separator or end of line)
        // Scan right from start_col until we hit a full column of spaces or end of line.
        // Wait, "separated by a single column of whitespace".
        // The numbers themselves might have spaces (e.g. alignment).
        // But the prompt says "separated by a single column of whitespace".
        // This usually implies that the separator is the *first* column encountered that is fully whitespace.
        // However, we also need to stop if we hit the next problem start?
        // Let's check if there are overlapping problems?
        // Prompt says "Each row of numeric values is now seperated by a single column of whitespace".
        // So the blocks are distinct.

        let mut end_col = start_col;
        while end_col < max_len {
            // Check if column is separator
            let is_separator = (0..padded_input.len()).all(|r| padded_input[r][end_col] == ' ');
            if is_separator {
                // But wait, the operator itself is at start_col.
                // If start_col was space, we wouldn't be here.
                // So we check from start_col + 1?
                // No, loop condition covers it.
                break;
            }

            // Also, we might just hit another operator if the spacing is tight?
            // "separated by a single column of whitespace". This implies at least one space column between blocks.
            // So we can safely scan until we hit a space column or end of line.
            // But we must check if valid content exists.
            end_col += 1;
        }

        // 5. Extract Numbers (Right to Left)
        // Range is [start_col, end_col)
        let mut numbers = Vec::new();

        for col in (start_col..end_col).rev() {
            // Build number string from rows 0 to num_rows-1
            let mut num_str = String::new();
            for r in 0..num_rows {
                let ch = padded_input[r][col];
                if !ch.is_whitespace() {
                    num_str.push(ch);
                }
            }

            if !num_str.is_empty() {
                let num = num_str.parse::<i64>().expect("Failed to parse number");
                numbers.push(num);
            }
        }

        // 6. Calculate
        if numbers.is_empty() {
            continue;
        }

        let mut result = numbers[0];
        for &num in &numbers[1..] {
            match op_char {
                '+' => result += num,
                '*' => result *= num,
                _ => panic!("Unknown operator"),
            }
        }
        total_sum += result;
    }

    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = vec![
            "5 3 7 2".to_string(),
            "2 1 4 1".to_string(),
            "6 5 1 0".to_string(),
            "* + * *".to_string(),
        ];
        assert_eq!(part1(&input), Ok(97));
    }

    #[test]
    fn test_example_2() {
        let input = vec![
            "123 328  51 64".to_string(),
            " 45 64  387 23".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   +".to_string(),
        ];
        assert_eq!(part1(&input), Ok(4277556));
    }

    #[test]
    fn test_error_too_few_lines() {
        let input = vec!["1 2".to_string(), "3 4".to_string(), "+ *".to_string()];
        assert!(part1(&input).is_err());
    }

    #[test]
    fn test_error_inconsistent_columns() {
        let input = vec![
            "1 2".to_string(),
            "3 4 5".to_string(),
            "6 7".to_string(),
            "+ *".to_string(),
        ];
        assert!(part1(&input).is_err());
    }

    #[test]
    fn test_error_inconsistent_operators() {
        let input = vec![
            "1 2".to_string(),
            "3 4".to_string(),
            "6 7".to_string(),
            "+".to_string(),
        ];
        assert!(part1(&input).is_err());
    }

    #[test]
    fn test_error_invalid_number() {
        let input = vec![
            "1 a".to_string(),
            "3 4".to_string(),
            "6 7".to_string(),
            "+ *".to_string(),
        ];
        assert!(part1(&input).is_err());
    }

    #[test]
    fn test_error_invalid_operator() {
        let input = vec![
            "1 2".to_string(),
            "3 4".to_string(),
            "6 7".to_string(),
            "+ /".to_string(),
        ];
        assert!(part1(&input).is_err());
    }

    #[test]
    fn test_part2_example_1() {
        let input = vec![
            "64  113".to_string(),
            "23  422".to_string(),
            "431 101".to_string(),
            "720  5 ".to_string(),
            "*   +  ".to_string(),
        ];
        // Problem 1: 10 * 4332 * 6247 = 270620040
        // Problem 2: 321 + 1205 + 141 = 1667
        // Total: 270621707
        assert_eq!(part2(&input), 270621707);
    }

    #[test]
    fn test_part2_example_2() {
        let input = vec![
            "123 328  51 64 ".to_string(),
            " 45 64  387 23 ".to_string(),
            "  6 98  215 314".to_string(),
            "*   +   *   +  ".to_string(),
        ];
        // From prompt: 33210 + 490 + 4243455 + 401 = 4277556 ? NO, that was Part 1 example.
        // Part 2 prompt:
        // 1. (4 431 623 +) -> 1058
        // 2. (175 581 32 *) -> 3253600
        // 3. (8 248 369 +) -> 625
        // 4. (356 24 1 *) -> 8544
        // Total: 1058 + 3253600 + 625 + 8544 = 3263827
        assert_eq!(part2(&input), 3263827);
    }
}
