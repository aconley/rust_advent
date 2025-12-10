/// Day 2.
fn main() -> std::io::Result<()> {
    let inputs: String = rust_advent::read_file_as_string("02")?;
    println!("Part 1: {}", part1(&inputs));
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

        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();

        total += sum_invalid_ids_in_range(start, end);
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
}
