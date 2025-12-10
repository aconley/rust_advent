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

/// Function for part 2.
fn part2(ranges: &str) -> u64 {
    let mut total: u128 = 0;

    for range_str in ranges.split(',') {
        if let Some((start_str, end_str)) = range_str.trim().split_once('-') {
            let start: u64 = start_str
                .parse()
                .expect(&format!("Could not parse {}", start_str));
            let end: u64 = end_str
                .parse()
                .expect(&format!("Could not parse {}", end_str));
            total += sum_invalid_ids_in_range_part2(start, end) as u128;
        }
    }

    total as u64
}

fn sum_invalid_ids_in_range_part2(start: u64, end: u64) -> u64 {
    let mut total: u128 = 0;
    let start_len = start.to_string().len();
    let end_len = end.to_string().len();

    // Iterate over each total digit length D involved in the range.
    for d in start_len..=end_len {
        // Range of numbers with D digits: [10^(D-1), 10^D - 1]
        // Intersect with [start, end].
        let p10_d_minus_1 = if d == 1 { 1 } else { 10u64.pow((d - 1) as u32) };
        let p10_d_minus_1_u128 = p10_d_minus_1 as u128;

        let min_d = p10_d_minus_1_u128;
        let max_d = (min_d * 10) - 1;

        let range_start = std::cmp::max(start as u128, min_d);
        let range_end = std::cmp::min(end as u128, max_d);

        if range_start > range_end {
            continue;
        }

        let primes = get_prime_factors(d);
        // We want to sum numbers composed of period L where L | D and L < D.
        // The possible periods are D/p for each prime factor p of D.
        // We use Inclusion-Exclusion on the boolean properties "has period D/p".
        // The intersection of "freq(D/p1)" and "freq(D/p2)" is "freq(gcd(D/p1, D/p2))"
        // gcd(D/a, D/b) = D / lcm(a, b).
        // Since a, b are single primes, lcm(a, b) = a*b if a!=b.

        // For a subset of distinct prime factors S, the term corresponds to
        // period L = D / product(S).
        // If |S| is odd, we ADD. If |S| is even, we SUBTRACT.

        // Example D=6 (primes 2, 3).
        // Terms:
        // + Period D/2 = 3.
        // + Period D/3 = 2.
        // - Period D/(2*3) = 1.

        let num_primes = primes.len();
        if num_primes == 0 {
            // D=1. No prime factors. No L < D.
            continue;
        }

        let subset_count = 1 << num_primes;
        for i in 1..subset_count {
            let mut product = 1u32;
            let mut set_bits = 0;
            for bit in 0..num_primes {
                if (i >> bit) & 1 == 1 {
                    product *= primes[bit];
                    set_bits += 1;
                }
            }

            let l = (d as u32) / product;

            // Sum numbers in [range_start, range_end] with period l.
            let term = sum_with_period(d as u32, l, range_start, range_end);

            if set_bits % 2 == 1 {
                total = total.wrapping_add(term);
            } else {
                total = total.wrapping_sub(term);
            }
        }
    }

    total as u64
}

fn sum_with_period(d: u32, l: u32, start: u128, end: u128) -> u128 {
    // Number X with length D and period L is Y * R
    // where R = (10^D - 1) / (10^L - 1).
    // Y has length L, i.e., 10^(L-1) <= Y <= 10^L - 1.
    // Exception L=1: Range includes 1..9, no leading zero issues.

    // We compute R using u128.
    let num = 10u128.pow(d) - 1;
    let den = 10u128.pow(l) - 1;
    let r = num / den;

    // Y constraints from structure:
    let min_y_struct = 10u128.pow(l - 1);
    let max_y_struct = 10u128.pow(l) - 1;

    // Y constraints from range:
    // Y * R >= start  =>  Y >= (start + R - 1) / R
    let min_y_range = (start + r - 1) / r;
    // Y * R <= end    =>  Y <= end / R
    let max_y_range = end / r;

    let min_y = std::cmp::max(min_y_struct, min_y_range);
    let max_y = std::cmp::min(max_y_struct, max_y_range);

    if min_y > max_y {
        return 0;
    }

    let count = max_y - min_y + 1;
    let sum_y = (min_y + max_y) * count / 2;
    sum_y * r
}

fn get_prime_factors(n: usize) -> Vec<u32> {
    let mut factors = Vec::new();
    let mut d = 2;
    let mut temp = n;
    while d * d <= temp {
        if temp % d == 0 {
            factors.push(d as u32);
            while temp % d == 0 {
                temp /= d;
            }
        }
        d += 1;
    }
    if temp > 1 {
        factors.push(temp as u32);
    }
    factors
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
        // 99 (invalid, period 1), 111 (invalid, period 1).
        // 99 + 111 = 210.
        assert_eq!(part2("95-115"), 210);

        // 12341234 (period 4)
        assert_eq!(part2("12341230-12341235"), 12341234);

        // Overlap case: 111111 (repetition of 11 three times, or 111 two times, or 1 six times).
        // Should be counted once.
        assert_eq!(part2("111111-111111"), 111111);
    }
}
