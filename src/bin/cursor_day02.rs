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
        if let Some((start_str, end_str)) = range_str.split_once('-') {
            let start: u64 = start_str
                .parse()
                .expect(&format!("Could not parse start: {}", start_str));
            let end: u64 = end_str
                .parse()
                .expect(&format!("Could not parse end: {}", end_str));
            total += sum_invalid_ids_in_range(start, end);
        }
    }
    
    total
}

/// Function for part 2.
///
/// Given a string of integer ranges, returns the sum of
/// all values in the ranges that are made only of some sequence
/// of digits repeated at least twice.
///
/// For example, 12341234 is invalid (1234 repeated twice),
/// and 1111111 is invalid (1 repeated seven times).
fn part2(ranges: &str) -> u64 {
    let mut total = 0u64;
    
    for range_str in ranges.split(',') {
        let range_str = range_str.trim();
        if let Some((start_str, end_str)) = range_str.split_once('-') {
            let start: u64 = start_str
                .parse()
                .expect(&format!("Could not parse start: {}", start_str));
            let end: u64 = end_str
                .parse()
                .expect(&format!("Could not parse end: {}", end_str));
            total += sum_invalid_ids_in_range_part2(start, end);
        }
    }
    
    total
}

/// Checks if a number is invalid (can be decomposed into two identical values).
/// A number is invalid if it has an even number of digits and the two halves are equal.
fn is_invalid_id(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    
    // Must have even number of digits
    if len % 2 != 0 {
        return false;
    }
    
    // Split in half and check if both halves are equal
    let half = len / 2;
    let first_half = &s[..half];
    let second_half = &s[half..];
    
    first_half == second_half
}

/// Sums all invalid IDs in the given range [start, end] (inclusive).
fn sum_invalid_ids_in_range(start: u64, end: u64) -> u64 {
    let mut sum = 0u64;
    
    for n in start..=end {
        if is_invalid_id(n) {
            sum += n;
        }
    }
    
    sum
}

/// Checks if a number is invalid for part 2 (made only of some sequence of digits repeated at least twice).
/// Examples: 12341234 (1234 two times), 123123123 (123 three times), 1111111 (1 seven times)
fn is_invalid_id_part2(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    
    // Try all possible pattern lengths from 1 to len/2
    // (we need at least 2 repetitions, so pattern length can be at most len/2)
    for pattern_len in 1..=len / 2 {
        // The length must be a multiple of pattern_len for it to be a valid repetition
        if len % pattern_len != 0 {
            continue;
        }
        
        let pattern = &s[..pattern_len];
        let num_repetitions = len / pattern_len;
        
        // Need at least 2 repetitions
        if num_repetitions < 2 {
            continue;
        }
        
        // Check if all chunks match the pattern
        let mut matches = true;
        for i in 1..num_repetitions {
            let start = i * pattern_len;
            let end = start + pattern_len;
            if &s[start..end] != pattern {
                matches = false;
                break;
            }
        }
        
        if matches {
            return true;
        }
    }
    
    false
}

/// Sums all invalid IDs in the given range [start, end] (inclusive) for part 2.
fn sum_invalid_ids_in_range_part2(start: u64, end: u64) -> u64 {
    let mut sum = 0u64;
    
    for n in start..=end {
        if is_invalid_id_part2(n) {
            sum += n;
        }
    }
    
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid_id() {
        // Valid cases (invalid IDs)
        assert!(is_invalid_id(11));
        assert!(is_invalid_id(22));
        assert!(is_invalid_id(1111));
        assert!(is_invalid_id(24452445));
        assert!(is_invalid_id(99));
        assert!(is_invalid_id(1010));
        assert!(is_invalid_id(1188511885));
        assert!(is_invalid_id(222222));
        assert!(is_invalid_id(446446));
        assert!(is_invalid_id(38593859));
        
        // Invalid cases (valid IDs)
        assert!(!is_invalid_id(121));
        assert!(!is_invalid_id(101));
        assert!(!is_invalid_id(1));
        assert!(!is_invalid_id(12));
        assert!(!is_invalid_id(123));
        assert!(!is_invalid_id(1234));
    }

    #[test]
    fn test_sum_invalid_ids_in_range() {
        // Example from prompt: 1-22 should have 11 and 22
        assert_eq!(sum_invalid_ids_in_range(1, 22), 11 + 22);
        
        // Example from prompt: 998-1112 should have 1010 and 1111
        assert_eq!(sum_invalid_ids_in_range(998, 1112), 1010 + 1111);
        
        // Example from prompt: 1405-1410 should have 0
        assert_eq!(sum_invalid_ids_in_range(1405, 1410), 0);
        
        // Example from prompt: 95-115 should have 99
        assert_eq!(sum_invalid_ids_in_range(95, 115), 99);
    }

    #[test]
    fn test_part1_examples() {
        // Example from prompt: 1-22,998-1112,1405-1410
        // Should have: 11 + 22 + 1010 + 1111 = 2154
        assert_eq!(part1("1-22,998-1112, 1405-1410"), 2154);
        
        // Larger example from prompt
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part1(input), 1227775554);
    }

    #[test]
    fn test_is_invalid_id_part2() {
        // Valid cases (invalid IDs for part 2)
        assert!(is_invalid_id_part2(11)); // 1 repeated twice
        assert!(is_invalid_id_part2(22)); // 2 repeated twice
        assert!(is_invalid_id_part2(1111)); // 1 repeated four times, or 11 repeated twice
        assert!(is_invalid_id_part2(12341234)); // 1234 repeated twice
        assert!(is_invalid_id_part2(123123123)); // 123 repeated three times
        assert!(is_invalid_id_part2(1212121212)); // 12 repeated five times
        assert!(is_invalid_id_part2(1111111)); // 1 repeated seven times
        assert!(is_invalid_id_part2(99)); // 9 repeated twice
        assert!(is_invalid_id_part2(111)); // 1 repeated three times
        assert!(is_invalid_id_part2(1010)); // 10 repeated twice
        assert!(is_invalid_id_part2(1188511885)); // 1188511885... wait, let me check
        assert!(is_invalid_id_part2(222222)); // 2 repeated six times, or 22 repeated three times, or 222 repeated twice
        assert!(is_invalid_id_part2(446446)); // 446 repeated twice
        assert!(is_invalid_id_part2(38593859)); // 38593859... let me check
        assert!(is_invalid_id_part2(565656)); // 56 repeated three times
        assert!(is_invalid_id_part2(824824824)); // 824 repeated three times
        assert!(is_invalid_id_part2(2121212121)); // 21 repeated five times
        
        // Invalid cases (valid IDs for part 2)
        assert!(!is_invalid_id_part2(121));
        assert!(!is_invalid_id_part2(101));
        assert!(!is_invalid_id_part2(1));
        assert!(!is_invalid_id_part2(12));
        assert!(!is_invalid_id_part2(123));
        assert!(!is_invalid_id_part2(1234));
        assert!(!is_invalid_id_part2(12345));
    }

    #[test]
    fn test_sum_invalid_ids_in_range_part2() {
        // Example from prompt: 11-22 should have 11 and 22
        assert_eq!(sum_invalid_ids_in_range_part2(11, 22), 11 + 22);
        
        // Example from prompt: 95-115 should have 99 and 111
        assert_eq!(sum_invalid_ids_in_range_part2(95, 115), 99 + 111);
        
        // Example from prompt: 998-1012 should have 999 and 1010
        assert_eq!(sum_invalid_ids_in_range_part2(998, 1012), 999 + 1010);
        
        // Example from prompt: 565653-565659 should have 565656
        assert_eq!(sum_invalid_ids_in_range_part2(565653, 565659), 565656);
    }

    #[test]
    fn test_part2_examples() {
        // Larger example from prompt
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        // Expected: 11 + 22 + 99 + 111 + 999 + 1010 + 1188511885 + 222222 + 446446 + 38593859 + 565656 + 824824824 + 2121212121 = 4174379265
        assert_eq!(part2(input), 4174379265);
    }
}
 