// Day 2.
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
    let parsed_ranges = parse_ranges(ranges);
    let mut total_sum: u128 = 0;

    for (start, end) in parsed_ranges {
        total_sum += sum_invalid_in_range(start, end) as u128;
    }

    total_sum as u64
}

/// Parses a string of comma-separated ranges (e.g., "1-10, 20-30")
/// into a vector of (start, end) tuples.
fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .filter_map(|range| {
            let parts: Vec<&str> = range.trim().split('-').collect();
            if parts.len() != 2 {
                return None;
            }
            let start = parts[0].parse::<u64>().ok()?;
            let end = parts[1].parse::<u64>().ok()?;
            Some((start, end))
        })
        .collect()
}

/// Calculates the sum of invalid IDs within a single inclusive range [start, end].
/// An invalid ID is one that can be decomposed into two identical values.
fn sum_invalid_in_range(start: u64, end: u64) -> u64 {
    let mut range_sum: u128 = 0;
    let start_u128 = start as u128;
    let end_u128 = end as u128;

    // Iterate over half-lengths L.
    // The invalid numbers are of form x * (10^L + 1)
    // where 10^(L-1) <= x < 10^L.
    // We check L from 1 up to 10 (covering up to ~1.8e19).
    for l in 1..=10 {
        let p10_l_minus_1 = 10u128.pow(l - 1);
        let p10_l = 10u128.pow(l);
        let m = p10_l + 1;

        let global_min_x = p10_l_minus_1;
        let global_max_x = p10_l - 1;

        // We need x * M >= start => x >= ceil(start / M)
        // (start + M - 1) / M
        let min_x_needed = (start_u128 + m - 1) / m;

        // We need x * M <= end => x <= floor(end / M)
        let max_x_needed = end_u128 / m;

        let effective_min = std::cmp::max(global_min_x, min_x_needed);
        let effective_max = std::cmp::min(global_max_x, max_x_needed);

        if effective_min <= effective_max {
            let count = effective_max - effective_min + 1;
            // Sum of arithmetic progression: count * (first + last) / 2
            let sum_x = (effective_min + effective_max) * count / 2;
            range_sum += sum_x * m;
        }
    }

    range_sum as u64
}

/// Function for part 2.
///
/// An ID is invalid if it is made only of some sequence of digits repeated at least twice.
/// Returns the sum of all invalid IDs in the given ranges.
fn part2(ranges: &str) -> u64 {
    let parsed_ranges = parse_ranges(ranges);
    let mut total_sum: u128 = 0;

    for (start, end) in parsed_ranges {
        total_sum += sum_invalid_in_range_part2(start, end) as u128;
    }

    total_sum as u64
}

/// Calculates the sum of invalid IDs within a single inclusive range [start, end] for Part 2.
/// An invalid ID is one made only of some sequence of digits repeated at least twice.
fn sum_invalid_in_range_part2(start: u64, end: u64) -> u64 {
    let mut range_sum: u128 = 0;

    let start_s = start.to_string();
    let end_s = end.to_string();
    let min_len = start_s.len();
    let max_len = end_s.len();

    // Iterate over total length D of the number
    for d in min_len..=max_len {
        // Determine valid range for this length D
        let p10_d_minus_1 = if d == 0 {
            0
        } else {
            10u128.pow((d - 1) as u32)
        };
        let p10_d = 10u128.pow(d as u32);
        let p10_d_upper = p10_d - 1; // inclusive max for length d

        let range_min = std::cmp::max(start as u128, p10_d_minus_1);
        let range_max = std::cmp::min(end as u128, p10_d_upper);

        if range_min > range_max {
            continue;
        }

        // Find distinct prime factors of D.
        let primes = get_distinct_prime_factors(d as u32);
        if primes.is_empty() {
            continue;
        }

        // Inclusion-Exclusion Principle
        let num_primes = primes.len();
        let num_subsets = 1 << num_primes;

        for i in 1..num_subsets {
            let mut subset = Vec::new();
            let mut set_bits = 0;
            for bit in 0..num_primes {
                if (i >> bit) & 1 == 1 {
                    subset.push(primes[bit]);
                    set_bits += 1;
                }
            }

            // Calculate LCM of the subset of prime factors
            let k_lcm = subset.iter().fold(1, |acc, &x| lcm(acc, x));

            // The period length L for this intersection
            let l_period = d as u32 / k_lcm;

            // Calculate Multiplier M = (10^D - 1) / (10^L - 1)
            let p10_l = 10u128.pow(l_period);
            let mut m = 0u128;
            let mut current_p = 1u128;
            for _ in 0..k_lcm {
                m += current_p;
                current_p *= p10_l;
            }

            // Base range for X: [10^(L-1), 10^L - 1]
            let base_min = 10u128.pow(l_period - 1);
            let base_max = 10u128.pow(l_period) - 1;

            let term = sum_multiples_in_range(m, base_min, base_max, range_min, range_max);

            if set_bits % 2 == 1 {
                range_sum += term;
            } else {
                range_sum -= term;
            }
        }
    }

    range_sum as u64
}

/// Helper: Sum of (X * m) for X in [base_min, base_max] such that product in [r_min, r_max]
fn sum_multiples_in_range(
    m: u128,
    base_min: u128,
    base_max: u128,
    r_min: u128,
    r_max: u128,
) -> u128 {
    // X * m >= r_min => X >= ceil(r_min / m)
    let min_x_needed = (r_min + m - 1) / m;
    // X * m <= r_max => X <= floor(r_max / m)
    let max_x_needed = r_max / m;

    let effective_min = std::cmp::max(base_min, min_x_needed);
    let effective_max = std::cmp::min(base_max, max_x_needed);

    if effective_min <= effective_max {
        let count = effective_max - effective_min + 1;
        // Sum of X: count * (min + max) / 2
        let sum_x = (effective_min + effective_max) * count / 2;
        sum_x * m
    } else {
        0
    }
}

fn get_distinct_prime_factors(mut n: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            factors.push(d);
            while n % d == 0 {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: u32, b: u32) -> u32 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ranges() {
        let input = "1-22, 998-1112,1405-1410";
        let expected = vec![(1, 22), (998, 1112), (1405, 1410)];
        assert_eq!(parse_ranges(input), expected);

        let input_spaces = " 1-10 , 20-30 ";
        let expected_spaces = vec![(1, 10), (20, 30)];
        assert_eq!(parse_ranges(input_spaces), expected_spaces);

        // Test bad input handling (should skip)
        let input_bad = "1-10, bad, 20-30";
        let expected_bad = vec![(1, 10), (20, 30)];
        assert_eq!(parse_ranges(input_bad), expected_bad);
    }

    #[test]
    fn test_sum_invalid_in_range() {
        // 1-22: 11, 22 -> 33
        assert_eq!(sum_invalid_in_range(1, 22), 33);

        // 998-1112: 1010, 1111 -> 2121
        assert_eq!(sum_invalid_in_range(998, 1112), 2121);

        // 1405-1410: none -> 0
        assert_eq!(sum_invalid_in_range(1405, 1410), 0);

        // 1-10: none -> 0
        assert_eq!(sum_invalid_in_range(1, 10), 0);

        // 11-11: 11
        assert_eq!(sum_invalid_in_range(11, 11), 11);
    }

    #[test]
    fn test_example_1() {
        // 1-22: 11, 22 -> 33
        // 998-1112: 1010, 1111 -> 2121
        // 1405-1410: none
        // Total: 33 + 2121 = 2154
        let input = "1-22,998-1112, 1405-1410";
        assert_eq!(part1(input), 2154);
    }

    #[test]
    fn test_example_2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part1(input), 1227775554);
    }

    #[test]
    fn test_basic_ranges() {
        // 1-10: no invalid numbers (11 is first)
        assert_eq!(part1("1-10"), 0);
        // 11-11: 11
        assert_eq!(part1("11-11"), 11);
        // 10-12: 11
        assert_eq!(part1("10-12"), 11);
    }

    #[test]
    fn test_part2_example_full() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part2(input), 4174379265);
    }

    #[test]
    fn test_part2_basic() {
        // 1-22: 11, 22. Same as part 1.
        assert_eq!(part2("1-22"), 33);

        // 95-115:
        // 99 (invalid), 111 (invalid).
        // 99 + 111 = 210.
        assert_eq!(part2("95-115"), 210);

        // 12341234
        let val = 12341234;
        assert_eq!(part2("12341230-12341235"), val);

        // Overlap case: 111111 (repetition of 11 three times, or 111 two times).
        // Should be counted once.
        // Range containing only 111111.
        assert_eq!(part2("111111-111111"), 111111);
    }

    #[test]
    fn test_helpers() {
        assert_eq!(get_distinct_prime_factors(6), vec![2, 3]);
        assert_eq!(get_distinct_prime_factors(12), vec![2, 3]);
        assert_eq!(get_distinct_prime_factors(5), vec![5]);
        assert_eq!(get_distinct_prime_factors(1), vec![]);

        assert_eq!(lcm(2, 3), 6);
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(1, 5), 5);
    }
}
