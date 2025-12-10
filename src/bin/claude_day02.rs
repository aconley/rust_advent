/// Day 2.
fn main() -> std::io::Result<()> {
    let inputs: String = rust_advent::read_file_as_string("02")?;
    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
    Ok(())
}

/// Function for part 1.
///
/// Given a string of integer ranges, returns the sum of
/// all values in the ranges that can be decomposed into two identical
/// values.
///
/// For example 1-22,30-50 contains the values 11, 22, 33, and 44
/// which sum to 110.
fn part1(ranges: &str) -> u64 {
    let mut total = 0u64;

    for range_str in ranges.split(',') {
        let range_str = range_str.trim();
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            continue;
        }

        let start: u64 = parts[0].parse().expect("Failed to parse range start");
        let end: u64 = parts[1].parse().expect("Failed to parse range end");

        total += sum_invalid_ids_in_range(start, end);
    }

    total
}

/// Calculates the sum of all invalid IDs within a single range [start, end] for part 2.
///
/// An invalid ID is one that consists of a pattern repeated at least twice.
/// For example: 12341234 = "1234" repeated 2 times, 111 = "1" repeated 3 times
///
/// # Arguments
/// * `start` - The start of the range (inclusive)
/// * `end` - The end of the range (inclusive)
///
/// # Returns
/// The sum of all invalid IDs in the range
fn sum_invalid_ids_in_range_part2(start: u64, end: u64) -> u64 {
    use std::collections::HashSet;
    let mut invalid_ids = HashSet::new();

    // Determine the range of digit counts to consider
    let min_digits = if start == 0 { 1 } else { start.to_string().len() };
    let max_digits = end.to_string().len();

    // Try all possible total digit counts
    for total_digits in min_digits..=max_digits {
        // Try all possible pattern lengths (divisors of total_digits)
        for pattern_length in 1..total_digits {
            if total_digits % pattern_length != 0 {
                continue;
            }

            let repetitions = total_digits / pattern_length;
            if repetitions < 2 {
                continue;
            }

            // Calculate the repeater value using u128 to avoid overflow
            // R = (10^total_digits - 1) / (10^pattern_length - 1)
            // This gives us the multiplier for a pattern to create the repeated number
            let total_pow = 10u128.pow(total_digits as u32);
            let pattern_pow = 10u128.pow(pattern_length as u32);
            let repeater_u128 = (total_pow - 1) / (pattern_pow - 1);

            // Check if repeater fits in u64
            if repeater_u128 > u64::MAX as u128 {
                continue;
            }
            let repeater = repeater_u128 as u64;

            // Range of pattern values (must have exactly pattern_length digits)
            let min_pattern = if pattern_length == 1 {
                1
            } else {
                10u64.pow((pattern_length - 1) as u32)
            };
            let max_pattern = 10u64.pow(pattern_length as u32) - 1;

            // Find patterns such that pattern * repeater is in [start, end]
            let pattern_start = min_pattern.max((start + repeater - 1) / repeater);
            let pattern_end = max_pattern.min(end / repeater);

            if pattern_start <= pattern_end {
                for pattern in pattern_start..=pattern_end {
                    if let Some(number) = pattern.checked_mul(repeater) {
                        if number >= start && number <= end {
                            invalid_ids.insert(number);
                        }
                    }
                }
            }
        }
    }

    invalid_ids.iter().sum()
}

/// Function for part 2.
///
/// Given a string of integer ranges, returns the sum of all values
/// that consist of a pattern repeated at least twice.
///
/// For example: 111 = "1" repeated 3 times, 12341234 = "1234" repeated 2 times
fn part2(ranges: &str) -> u64 {
    let mut total = 0u64;

    for range_str in ranges.split(',') {
        let range_str = range_str.trim();
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            continue;
        }

        let start: u64 = parts[0].parse().expect("Failed to parse range start");
        let end: u64 = parts[1].parse().expect("Failed to parse range end");

        total += sum_invalid_ids_in_range_part2(start, end);
    }

    total
}

/// Calculates the sum of all invalid IDs within a single range [start, end].
///
/// An invalid ID is one that can be decomposed into two identical values.
/// For example: 1111 = "11" + "11", 24452445 = "2445" + "2445"
///
/// # Arguments
/// * `start` - The start of the range (inclusive)
/// * `end` - The end of the range (inclusive)
///
/// # Returns
/// The sum of all invalid IDs in the range
fn sum_invalid_ids_in_range(start: u64, end: u64) -> u64 {
    let mut total = 0u64;

    // Try all possible even digit lengths (2, 4, 6, 8, 10)
    // An invalid ID with 2n digits has the form: X * (10^n + 1)
    // where X is an n-digit number without leading zeros
    for digit_count in (2..=10).step_by(2) {
        let n = digit_count / 2;
        let repeater = 10u64.pow(n) + 1;

        // Range of X values for this digit length
        let min_x = if n == 1 { 1 } else { 10u64.pow(n - 1) };
        let max_x = 10u64.pow(n) - 1;

        // Find X values such that X * repeater is in [start, end]
        let x_start = min_x.max((start + repeater - 1) / repeater);
        let x_end = max_x.min(end / repeater);

        if x_start <= x_end {
            // Use arithmetic series formula: sum = n * (first + last) / 2
            let count = x_end - x_start + 1;
            let sum_of_x = (x_start + x_end) * count / 2;
            total += sum_of_x * repeater;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for sum_invalid_ids_in_range function
    #[test]
    fn test_single_range_with_2_digit_ids() {
        // Range 1-22 contains: 11, 22
        assert_eq!(sum_invalid_ids_in_range(1, 22), 11 + 22);
        // Range 11-99 contains all 2-digit invalid IDs
        assert_eq!(
            sum_invalid_ids_in_range(11, 99),
            11 + 22 + 33 + 44 + 55 + 66 + 77 + 88 + 99
        );
    }

    #[test]
    fn test_single_range_with_4_digit_ids() {
        // Range 998-1112 contains: 1010, 1111
        assert_eq!(sum_invalid_ids_in_range(998, 1112), 1010 + 1111);
        // Range 1010-1111 contains: 1010, 1111
        assert_eq!(sum_invalid_ids_in_range(1010, 1111), 1010 + 1111);
    }

    #[test]
    fn test_single_range_no_invalid_ids() {
        // No invalid IDs in these ranges
        assert_eq!(sum_invalid_ids_in_range(1, 10), 0);
        assert_eq!(sum_invalid_ids_in_range(23, 32), 0);
        assert_eq!(sum_invalid_ids_in_range(100, 1000), 0);
        assert_eq!(sum_invalid_ids_in_range(1405, 1410), 0);
    }

    #[test]
    fn test_single_range_exact_match() {
        // Range contains exactly one invalid ID
        assert_eq!(sum_invalid_ids_in_range(11, 11), 11);
        assert_eq!(sum_invalid_ids_in_range(99, 99), 99);
        assert_eq!(sum_invalid_ids_in_range(1010, 1010), 1010);
        assert_eq!(sum_invalid_ids_in_range(222222, 222222), 222222);
    }

    #[test]
    fn test_single_range_with_6_digit_ids() {
        // Range 222220-222224 contains: 222222
        assert_eq!(sum_invalid_ids_in_range(222220, 222224), 222222);
        // Range 446443-446449 contains: 446446
        assert_eq!(sum_invalid_ids_in_range(446443, 446449), 446446);
    }

    #[test]
    fn test_single_range_with_8_digit_ids() {
        // Range 38593856-38593862 contains: 38593859
        assert_eq!(sum_invalid_ids_in_range(38593856, 38593862), 38593859);
    }

    #[test]
    fn test_single_range_with_10_digit_ids() {
        // Range 1188511880-1188511890 contains: 1188511885
        assert_eq!(sum_invalid_ids_in_range(1188511880, 1188511890), 1188511885);
    }

    #[test]
    fn test_single_range_mixed_digit_lengths() {
        // Range 11-1111 contains 2-digit and 4-digit invalid IDs
        let expected = 11 + 22 + 33 + 44 + 55 + 66 + 77 + 88 + 99 + 1010 + 1111;
        assert_eq!(sum_invalid_ids_in_range(11, 1111), expected);
    }

    // Tests for part1 function (parsing multiple ranges)
    #[test]
    fn test_example1() {
        // 1-22 contains: 11, 22
        // 998-1112 contains: 1010, 1111
        // 1405-1410 contains: none
        // Sum: 11 + 22 + 1010 + 1111 = 2154
        assert_eq!(part1("1-22,998-1112,1405-1410"), 2154);
    }

    #[test]
    fn test_example2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        // Expected invalid IDs:
        // 11-22: 11, 22 (sum: 33)
        // 95-115: 99 (sum: 99)
        // 998-1012: 1010 (sum: 1010)
        // 1188511880-1188511890: 1188511885 (sum: 1188511885)
        // 222220-222224: 222222 (sum: 222222)
        // 1698522-1698528: none
        // 446443-446449: 446446 (sum: 446446)
        // 38593856-38593862: 38593859 (sum: 38593859)
        // Rest: none
        // Total: 1227775554
        assert_eq!(part1(input), 1227775554);
    }

    #[test]
    fn test_single_invalid_ids() {
        assert_eq!(part1("11-11"), 11);
        assert_eq!(part1("22-22"), 22);
        assert_eq!(part1("99-99"), 99);
        assert_eq!(part1("1010-1010"), 1010);
        assert_eq!(part1("1111-1111"), 1111);
    }

    #[test]
    fn test_multiple_invalid_ids() {
        // All 2-digit invalid IDs
        assert_eq!(part1("11-99"), 11 + 22 + 33 + 44 + 55 + 66 + 77 + 88 + 99);
        // Mix of 2-digit and 4-digit
        assert_eq!(
            part1("11-1111"),
            11 + 22 + 33 + 44 + 55 + 66 + 77 + 88 + 99 + 1010 + 1111
        );
    }

    #[test]
    fn test_no_invalid_ids() {
        assert_eq!(part1("1-10"), 0);
        assert_eq!(part1("23-32"), 0);
        assert_eq!(part1("100-1000"), 0);
        assert_eq!(part1("1112-1200"), 0);
    }

    #[test]
    fn test_edge_cases() {
        // Range that includes exactly one invalid ID at the boundary
        assert_eq!(part1("22-22"), 22);
        assert_eq!(part1("1-22"), 11 + 22);
        assert_eq!(part1("11-22"), 11 + 22);

        // Range with spaces (should handle trimming)
        assert_eq!(part1("11-22, 99-99"), 11 + 22 + 99);
    }

    #[test]
    fn test_large_numbers() {
        // 6-digit invalid ID: 222222 = 222 * 1001
        assert_eq!(part1("222222-222222"), 222222);
        // 8-digit invalid ID: 38593859 = 3859 * 10001
        assert_eq!(part1("38593859-38593859"), 38593859);
    }

    // Tests for sum_invalid_ids_in_range_part2 function
    #[test]
    fn test_part2_single_range_2_digit_repetitions() {
        // 11 = "1" repeated 2 times
        assert_eq!(sum_invalid_ids_in_range_part2(11, 11), 11);
        // 22 = "2" repeated 2 times
        assert_eq!(sum_invalid_ids_in_range_part2(22, 22), 22);
        // 11-22 contains 11 and 22
        assert_eq!(sum_invalid_ids_in_range_part2(11, 22), 11 + 22);
    }

    #[test]
    fn test_part2_single_range_3_digit_repetitions() {
        // 111 = "1" repeated 3 times
        assert_eq!(sum_invalid_ids_in_range_part2(111, 111), 111);
        // 999 = "9" repeated 3 times
        assert_eq!(sum_invalid_ids_in_range_part2(999, 999), 999);
        // 95-115 contains 99 ("9" x2) and 111 ("1" x3)
        assert_eq!(sum_invalid_ids_in_range_part2(95, 115), 99 + 111);
    }

    #[test]
    fn test_part2_single_range_4_digit_repetitions() {
        // 1010 = "10" repeated 2 times
        assert_eq!(sum_invalid_ids_in_range_part2(1010, 1010), 1010);
        // 1111 = "1" repeated 4 times (or "11" x2)
        assert_eq!(sum_invalid_ids_in_range_part2(1111, 1111), 1111);
        // 998-1012 contains 999 ("9" x3) and 1010 ("10" x2)
        assert_eq!(sum_invalid_ids_in_range_part2(998, 1012), 999 + 1010);
    }

    #[test]
    fn test_part2_single_range_6_digit_repetitions() {
        // 565656 = "56" repeated 3 times
        assert_eq!(sum_invalid_ids_in_range_part2(565656, 565656), 565656);
        // 565653-565659 contains 565656
        assert_eq!(sum_invalid_ids_in_range_part2(565653, 565659), 565656);
    }

    #[test]
    fn test_part2_single_range_9_digit_repetitions() {
        // 824824824 = "824" repeated 3 times
        assert_eq!(sum_invalid_ids_in_range_part2(824824824, 824824824), 824824824);
        // 824824821-824824827 contains 824824824
        assert_eq!(
            sum_invalid_ids_in_range_part2(824824821, 824824827),
            824824824
        );
    }

    #[test]
    fn test_part2_single_range_10_digit_repetitions() {
        // 2121212121 = "21" repeated 5 times
        assert_eq!(
            sum_invalid_ids_in_range_part2(2121212121, 2121212121),
            2121212121
        );
        // 2121212118-2121212124 contains 2121212121
        assert_eq!(
            sum_invalid_ids_in_range_part2(2121212118, 2121212124),
            2121212121
        );
    }

    #[test]
    fn test_part2_no_double_counting() {
        // 1111 can be seen as "1" x4 or "11" x2, but should only be counted once
        assert_eq!(sum_invalid_ids_in_range_part2(1111, 1111), 1111);
        // Range with multiple overlapping patterns
        assert_eq!(sum_invalid_ids_in_range_part2(11, 1111), 11 + 22 + 33 + 44 + 55 + 66 + 77 + 88 + 99 + 111 + 222 + 333 + 444 + 555 + 666 + 777 + 888 + 999 + 1010 + 1111);
    }

    #[test]
    fn test_part2_no_invalid_ids() {
        // Ranges with no repetition patterns
        assert_eq!(sum_invalid_ids_in_range_part2(1, 10), 0);
        assert_eq!(sum_invalid_ids_in_range_part2(23, 32), 0);
        assert_eq!(sum_invalid_ids_in_range_part2(1698522, 1698528), 0);
    }

    // Tests for part2 function (parsing multiple ranges)
    #[test]
    fn test_part2_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        // Expected invalid IDs:
        // 11-22: 11, 22
        // 95-115: 99, 111
        // 998-1012: 999, 1010
        // 1188511880-1188511890: 1188511885
        // 222220-222224: 222222
        // 1698522-1698528: none
        // 446443-446449: 446446
        // 38593856-38593862: 38593859
        // 565653-565659: 565656
        // 824824821-824824827: 824824824
        // 2121212118-2121212124: 2121212121
        // Total: 4174379265
        assert_eq!(part2(input), 4174379265);
    }

    #[test]
    fn test_part2_simple_cases() {
        assert_eq!(part2("11-11"), 11);
        assert_eq!(part2("111-111"), 111);
        assert_eq!(part2("11-22,99-99"), 11 + 22 + 99);
    }
}
