use rayon::prelude::*;

/// Day 3.
fn main() -> std::io::Result<()> {
    let inputs: Vec<Vec<u8>> = rust_advent::read_number_grid("03")?;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "part1" => println!("Part 1: {}", part1_parallel(&inputs)),
            "part2" => println!("Part 2: {}", part2_parallel(&inputs)),
            _ => {
                println!("Part 1: {}", part1_parallel(&inputs));
                println!("Part 2: {}", part2_parallel(&inputs));
            }
        }
    } else {
        println!("Part 1: {}", part1_parallel(&inputs));
        println!("Part 2: {}", part2_parallel(&inputs));
    }
    Ok(())
}

/// Function for part 1 (single-threaded).
///
/// Given a grid of numbers, for each row find the largest number that
/// can be formed by selecting two numbers from the row in order.
/// For example, in the row [1, 2, 5, 2, 1] the largest number is 52.
/// This function returns the sum of the largest numbers for each row
/// over all provided rows.
pub fn part1(grid: &Vec<Vec<u8>>) -> u64 {
    grid.iter()
        .map(|row| find_max_two_digit(row))
        .sum()
}

/// Function for part 1 (parallel version using rayon).
///
/// For large input files with many rows, this version processes rows in parallel
/// across multiple CPU cores for better performance.
pub fn part1_parallel(grid: &Vec<Vec<u8>>) -> u64 {
    grid.par_iter()
        .map(|row| find_max_two_digit(row))
        .sum()
}

/// Function for part 2 (single-threaded).
///
/// Given a grid of numbers, for each row find the largest 12-digit number that
/// can be formed by selecting 12 numbers from the row in order.
/// Returns the sum of these numbers across all rows.
pub fn part2(grid: &Vec<Vec<u8>>) -> u64 {
    grid.iter()
        .map(|row| find_max_n_digit(row, 12))
        .sum()
}

/// Function for part 2 (parallel version using rayon).
///
/// For large input files with many rows, this version processes rows in parallel
/// across multiple CPU cores for better performance.
pub fn part2_parallel(grid: &Vec<Vec<u8>>) -> u64 {
    grid.par_iter()
        .map(|row| find_max_n_digit(row, 12))
        .sum()
}

/// Optimized helper function to find the maximum 2-digit number in a row.
///
/// Time complexity: O(m) where m is the row length (vs O(m²) naive approach)
///
/// Algorithm: For position i, the best 2-digit number starting at i is
/// row[i] * 10 + max(row[i+1..]). We precompute suffix maximums in one pass,
/// then find the best starting position in another pass.
fn find_max_two_digit(row: &[u8]) -> u64 {
    if row.len() < 2 {
        return 0;
    }

    // Build suffix maximum array: suffix_max[i] = max value in row[i..]
    let mut suffix_max = vec![0u8; row.len()];
    suffix_max[row.len() - 1] = row[row.len() - 1];

    for i in (0..row.len() - 1).rev() {
        suffix_max[i] = suffix_max[i + 1].max(row[i]);
    }

    // Find the maximum 2-digit number
    // For each position i, best we can do is row[i] * 10 + suffix_max[i+1]
    let mut max_value = 0u64;
    for i in 0..row.len() - 1 {
        let value = row[i] as u64 * 10 + suffix_max[i + 1] as u64;
        max_value = max_value.max(value);
    }

    max_value
}

/// Generalized helper function to find the maximum n-digit number in a row.
///
/// Time complexity: O(m × n) where m is row length and n is number of digits
///
/// Algorithm: Greedy selection with lookahead. For each output position k,
/// find the maximum value in the range [last_pos+1, row.len()-(n-k)].
/// This ensures we have enough remaining positions to fill all n digits.
fn find_max_n_digit(row: &[u8], n: usize) -> u64 {
    if row.len() < n {
        return 0;
    }

    let mut result = 0u64;
    let mut current_pos: isize = -1;

    for k in 0..n {
        // Calculate valid search range
        let start = (current_pos + 1) as usize;
        let end = row.len() - (n - k - 1);

        // Find maximum value and its position in range [start, end)
        let mut max_val = 0u8;
        let mut max_idx = start;

        for i in start..end {
            if row[i] > max_val {
                max_val = row[i];
                max_idx = i;
            }
        }

        // Add digit to result
        result = result * 10 + max_val as u64;
        current_pos = max_idx as isize;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_row() {
        // Example from problem: [1, 5, 3, 7] should give 57 (positions 1 and 3)
        // But actually the max is 75 (positions 3 and 1 if we could reverse)
        // Since we need i < j, we check: 15, 13, 17, 53, 57, 37
        // Maximum is 57
        let grid = vec![vec![1, 5, 3, 7]];
        assert_eq!(part1(&grid), 57);
    }

    #[test]
    fn test_four_rows_example() {
        // Example from problem description
        // 987654321111111 -> 98
        // 811111111111119 -> 89
        // 234234234234278 -> 78
        // 818181911112111 -> 92
        // Total: 357
        let grid = vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ];
        assert_eq!(part1(&grid), 357);
    }

    #[test]
    fn test_single_row_max_at_start() {
        // Row where largest digits are at the beginning
        let grid = vec![vec![9, 8, 1, 1, 1]];
        assert_eq!(part1(&grid), 98);
    }

    #[test]
    fn test_single_row_max_at_end() {
        // Row where largest digit is at the end
        let grid = vec![vec![1, 1, 1, 8, 9]];
        assert_eq!(part1(&grid), 89);
    }

    #[test]
    fn test_two_element_row() {
        // Minimal row with just two elements
        let grid = vec![vec![3, 7]];
        assert_eq!(part1(&grid), 37);
    }

    #[test]
    fn test_all_same_digits() {
        // Row with all same digits
        let grid = vec![vec![5, 5, 5, 5]];
        assert_eq!(part1(&grid), 55);
    }

    #[test]
    fn test_multiple_rows() {
        // Multiple rows with different maximums
        let grid = vec![
            vec![1, 2, 3],    // max: 23
            vec![9, 1, 1],    // max: 91
            vec![4, 5, 6, 7], // max: 67
        ];
        assert_eq!(part1(&grid), 23 + 91 + 67);
    }

    #[test]
    fn test_parallel_version_matches_sequential() {
        // Verify parallel version produces same results as sequential
        let grid = vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ];
        assert_eq!(part1(&grid), part1_parallel(&grid));
        assert_eq!(part1_parallel(&grid), 357);
    }

    // Part 2 tests

    #[test]
    fn test_part2_single_row_example1() {
        // 987654321111111 (15 digits) -> 987654321111
        let grid = vec![vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]];
        assert_eq!(part2(&grid), 987654321111);
    }

    #[test]
    fn test_part2_single_row_example2() {
        // 811111111111119 (15 digits) -> 811111111119
        let grid = vec![vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]];
        assert_eq!(part2(&grid), 811111111119);
    }

    #[test]
    fn test_part2_single_row_example3() {
        // 234234234234278 (15 digits) -> 434234234278
        let grid = vec![vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]];
        assert_eq!(part2(&grid), 434234234278);
    }

    #[test]
    fn test_part2_single_row_example4() {
        // 818181911112111 (15 digits) -> 888911112111
        let grid = vec![vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]];
        assert_eq!(part2(&grid), 888911112111);
    }

    #[test]
    fn test_part2_all_examples() {
        // Test all 4 examples together
        // Expected sum: 3121910778619
        let grid = vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ];
        assert_eq!(part2(&grid), 3121910778619);
    }

    #[test]
    fn test_part2_parallel_matches_sequential() {
        // Verify parallel version produces same results as sequential
        let grid = vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ];
        assert_eq!(part2(&grid), part2_parallel(&grid));
        assert_eq!(part2_parallel(&grid), 3121910778619);
    }

    #[test]
    fn test_part2_exactly_12_elements() {
        // Row with exactly 12 elements should select all in order
        let grid = vec![vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 9, 8]];
        assert_eq!(part2(&grid), 987654321098);
    }

    #[test]
    fn test_part2_fewer_than_12_elements() {
        // Row with fewer than 12 elements should return 0
        let grid = vec![vec![9, 8, 7, 6, 5]];
        assert_eq!(part2(&grid), 0);
    }
}