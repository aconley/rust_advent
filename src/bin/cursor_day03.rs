/// Day 3.
fn main() -> std::io::Result<()> {
    let inputs: Vec<Vec<u8>> = rust_advent::read_number_grid("03")?;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "part1" => println!("Part 1: {}", part1(&inputs)),
            "part2" => println!("Part 2: {}", part2(&inputs)),
            _ => {
                println!("Part 1: {}", part1(&inputs));
                println!("Part 2: {}", part2(&inputs));
            }
        }
    } else {
        println!("Part 1: {}", part1(&inputs));
        println!("Part 2: {}", part2(&inputs));
    }
    Ok(())
}

/// Function for part 1.
///
/// Given a grid of numbers, for each row find the largest number that
/// can be formed by selecting two numbers from the row in order.
/// For example, in the row [1, 2, 5, 2, 1] the largest number is 52.
/// This function returns the sum of the largest numbers for each row
/// over all provided rows.
pub fn part1(grid: &Vec<Vec<u8>>) -> u64 {
    grid.iter()
        .map(|row| {
            let mut max_value = 0u64;
            for i in 0..row.len() {
                for j in (i + 1)..row.len() {
                    let value = (10 * row[i] as u64) + row[j] as u64;
                    max_value = max_value.max(value);
                }
            }
            max_value
        })
        .sum()
}

/// Function for part 2.
///
/// Given a grid of numbers, for each row find the largest 12-digit number that
/// can be formed by selecting 12 digits from the row in order.
/// This function returns the sum of the largest numbers for each row
/// over all provided rows.
pub fn part2(grid: &Vec<Vec<u8>>) -> u64 {
    grid.iter()
        .map(|row| {
            if row.len() < 12 {
                // Can't form a 12-digit number, return 0
                return 0;
            }
            
            if row.len() == 12 {
                // Use all digits
                return digits_to_number(row);
            }
            
            // Greedy algorithm: remove (len - 12) digits to maximize result
            // Use a stack to build the result
            let mut stack: Vec<u8> = Vec::new();
            let to_remove = row.len() - 12;
            let mut removed = 0;
            
            for &digit in row.iter() {
                // Remove from stack while we can still remove digits and
                // the current digit is larger than the top of the stack
                while removed < to_remove 
                    && !stack.is_empty() 
                    && digit > *stack.last().unwrap() {
                    stack.pop();
                    removed += 1;
                }
                stack.push(digit);
            }
            
            // If we haven't removed enough, remove from the end
            while stack.len() > 12 {
                stack.pop();
            }
            
            digits_to_number(&stack)
        })
        .sum()
}

/// Converts a vector of digits to a u64 number.
fn digits_to_number(digits: &[u8]) -> u64 {
    digits.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_example() {
        // From prompt: [1, 5, 3, 7] -> 57 (selecting 5 and 7)
        let grid = vec![vec![1, 5, 3, 7]];
        assert_eq!(part1(&grid), 57);
    }

    #[test]
    fn test_larger_example() {
        // From prompt: 4 rows should sum to 357
        // 987654321111111 -> 98
        // 811111111111119 -> 89
        // 234234234234278 -> 78
        // 818181911112111 -> 92
        let grid = vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ];
        assert_eq!(part1(&grid), 357);
    }

    #[test]
    fn test_single_row() {
        let grid = vec![vec![1, 2, 5, 2, 1]];
        // Possible pairs: 12, 15, 12, 11, 25, 22, 21, 52, 51, 21
        // Max is 52
        assert_eq!(part1(&grid), 52);
    }

    #[test]
    fn test_row_with_duplicates() {
        let grid = vec![vec![9, 9, 9]];
        // Possible pairs: 99, 99, 99
        // Max is 99
        assert_eq!(part1(&grid), 99);
    }

    #[test]
    fn test_row_two_elements() {
        let grid = vec![vec![4, 2]];
        // Only one pair: 42
        assert_eq!(part1(&grid), 42);
    }

    #[test]
    fn test_empty_grid() {
        let grid: Vec<Vec<u8>> = vec![];
        assert_eq!(part1(&grid), 0);
    }

    #[test]
    fn test_multiple_rows_simple() {
        let grid = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        // Row 1: 12
        // Row 2: 34
        // Sum: 12 + 34 = 46
        assert_eq!(part1(&grid), 46);
    }

    #[test]
    fn test_descending_order() {
        let grid = vec![vec![9, 8, 7, 6]];
        // Best pair is 9 and 8 -> 98
        assert_eq!(part1(&grid), 98);
    }

    #[test]
    fn test_ascending_order() {
        let grid = vec![vec![1, 2, 3, 4]];
        // Best pair is 3 and 4 -> 34
        assert_eq!(part1(&grid), 34);
    }

    #[test]
    fn test_part2_larger_example() {
        // From prompt: 4 rows should sum to 3121910778619
        // 987654321111111 -> 987654321111
        // 811111111111119 -> 811111111119
        // 234234234234278 -> 434234234278
        // 818181911112111 -> 888911112111
        let grid = vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ];
        assert_eq!(part2(&grid), 3121910778619);
    }

    #[test]
    fn test_part2_exactly_12_digits() {
        let grid = vec![vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 9, 8]];
        // Should use all 12 digits
        assert_eq!(part2(&grid), 987654321098);
    }

    #[test]
    fn test_part2_fewer_than_12_digits() {
        let grid = vec![vec![1, 2, 3, 4, 5]];
        // Can't form 12-digit number, should return 0
        assert_eq!(part2(&grid), 0);
    }

    #[test]
    fn test_part2_descending_order() {
        // 15 digits: remove smaller digits (0, 1, 2) to maximize result
        // Greedy algorithm removes 0, 1, 2 when it sees larger digits 9, 8, 7
        let grid = vec![vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 9, 8, 7, 6, 5]];
        // Result: 987654398765 (removed 0, 1, 2, kept the larger digits from the end)
        assert_eq!(part2(&grid), 987654398765);
    }

    #[test]
    fn test_part2_ascending_order() {
        // 15 digits in ascending order: remove first 3
        let grid = vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5]];
        // Should remove first 3 smallest: keep 4,5,6,7,8,9,0,1,2,3,4,5
        assert_eq!(part2(&grid), 456789012345);
    }

    #[test]
    fn test_part2_empty_grid() {
        let grid: Vec<Vec<u8>> = vec![];
        assert_eq!(part2(&grid), 0);
    }

    #[test]
    fn test_digits_to_number() {
        assert_eq!(digits_to_number(&[1, 2, 3]), 123);
        assert_eq!(digits_to_number(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1]), 987654321111);
        assert_eq!(digits_to_number(&[0, 1, 2]), 12); // Leading zero is preserved as a digit
    }
}

