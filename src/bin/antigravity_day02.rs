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
    let mut total_invalid_sum: u64 = 0;

    for range_str in ranges.split(',') {
        if let Some((start_str, end_str)) = range_str.trim().split_once('-') {
            let start: u64 = start_str
                .parse()
                .expect(&format!("Could not parse {}", start_str));
            let end: u64 = end_str
                .parse()
                .expect(&format!("Could not parse {}", end_str));
            total_invalid_sum += sum_invalid_ids_in_range(start, end);
        }
    }

    total_invalid_sum
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
    let s_end = sum_invalid_upto(end);
    let s_start_minus_1 = sum_invalid_upto(start.saturating_sub(1));
    s_end.saturating_sub(s_start_minus_1)
}

/// Calculus the sum of all "invalid" numbers <= limit.
/// An invalid number is one formed by concatenating a number with itself (e.g. 1212).
fn sum_invalid_upto(limit: u64) -> u64 {
    let mut total: u64 = 0;
    // We want numbers of form y * (10^k + 1).
    // k is the number of digits in the half-part y.
    // k can range from 1 to 5 (since limit is u32).
    // range for y is [10^(k-1), 10^k - 1].
    // also y * (10^k + 1) <= limit  =>  y <= limit / (10^k + 1).

    // Powers of 10: 10^0=1, 10^1=10, ...
    // k=1: multiplier=11, y in [1, 9]
    // k=2: multiplier=101, y in [10, 99]
    // k=3: multiplier=1001, y in [100, 999]
    // k=4: multiplier=10001, y in [1000, 9999]
    // k=5: multiplier=100001, y in [10000, 99999]

    let mut p10_prev = 1u64; // 10^(k-1)

    for _k in 1..=5 {
        let p10_curr = p10_prev * 10; // 10^k
        let multiplier = p10_curr + 1;

        // Determine valid range for y: [y_min, y_max]
        let y_min = p10_prev;

        // y_upper_bound from limit
        let y_limit = (limit as u64) / multiplier;

        // y_max is min(10^k - 1, y_limit)
        let y_max_possible = p10_curr - 1;
        let y_max = std::cmp::min(y_max_possible, y_limit);

        if y_min <= y_max {
            let count = y_max - y_min + 1;
            // Sum of arithmetic series y_min..=y_max: n/2 * (first + last)
            let sum_y = count * (y_min + y_max) / 2;
            total += sum_y * multiplier;
        }

        p10_prev = p10_curr;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_invalid_upto_small() {
        // k=1: 11, 22, 33...
        // 11 is the first invalid number.
        assert_eq!(sum_invalid_upto(10), 0);
        assert_eq!(sum_invalid_upto(11), 11);
        assert_eq!(sum_invalid_upto(12), 11);
        assert_eq!(sum_invalid_upto(21), 11);
        assert_eq!(sum_invalid_upto(22), 11 + 22);
    }

    #[test]
    fn test_sum_invalid_upto_larger() {
        // k=1 sum: 11+22+...+99 = 11*(1+..+9) = 11*45 = 495
        assert_eq!(sum_invalid_upto(100), 495);
        // Next is 1010 (k=2, y=10).
        assert_eq!(sum_invalid_upto(1009), 495);
        assert_eq!(sum_invalid_upto(1010), 495 + 1010);
        assert_eq!(sum_invalid_upto(1112), 495 + 1010 + 1111);
    }

    #[test]
    fn test_part1_example_small() {
        assert_eq!(part1("1-22"), 33);
        assert_eq!(part1("95-115"), 99);
        assert_eq!(part1("998-1012"), 1010);
    }

    #[test]
    fn test_part1_large_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part1(input), 1227775554);
    }
}
