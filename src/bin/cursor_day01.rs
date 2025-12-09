/// Day 1.
fn main() -> std::io::Result<()> {
    let inputs: Vec<String> = rust_advent::read_file_as_lines("01")?;
    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
    Ok(())
}

/// Part 1: Count the number of times the dial is pointing at 0 after a rotation.
///
/// The dial goes from 0 to 99, and starts at position 50, with wrapping.
/// 
/// Inputs:
///   input: a vector of strings.  Each string is a rotation of the dial expressed
///          as a single character direction (L or R) followed by a number of clicks.
/// Returns:
///   The number of times the dial is pointing at 0 after a rotation.
fn part1(inputs: &[String]) -> i32 {
    let mut position = 50;
    let mut count = 0;
    
    for rotation in inputs {
        // Parse the rotation string (e.g., "L68" or "R48")
        let direction = rotation.chars().next().unwrap();
        let distance: i32 = rotation[1..].parse().unwrap();
        
        // Apply the rotation
        match direction {
            'L' => {
                // Rotate left (toward lower numbers)
                position = (position - distance + 100) % 100;
            }
            'R' => {
                // Rotate right (toward higher numbers)
                position = (position + distance) % 100;
            }
            _ => panic!("Invalid direction: {}", direction),
        }
        
        // Count if the dial is pointing at 0
        if position == 0 {
            count += 1;
        }
    }
    count
}

/// Part 2: Count the number of times the dial is pointing at 0 at any point
/// during a rotation.
///
/// The dial goes from 0 to 99, and starts at position 50, with wrapping.
/// 
/// Inputs:
///   input: a vector of strings.  Each string is a rotation of the dial expressed
///          as a single character direction (L or R) followed by a number of clicks.
/// Returns:
///   The number of times the dial is pointing at 0 at any point during a rotation.
// This does not give the correct answer.
fn part2(inputs: &[String]) -> i32 {
    let mut position = 50;
    let mut count = 0;
    
    for rotation in inputs {
        // Parse the rotation string (e.g., "L68" or "R48")
        let direction = rotation.chars().next().unwrap();
        let distance: i32 = rotation[1..].parse().unwrap();
        
        let start = position;
        
        // Apply the rotation and count zeros during the rotation
        match direction {
            'L' => {
                let end = (position - distance + 100) % 100;
                
                // Count zeros during rotation: we pass through 0 when (start - k) % 100 == 0
                // for k in [1, distance]. This happens at k = start, start+100, start+200, ...
                // Count how many such k values are in [1, distance]
                let zeros_during = if start == 0 {
                    // When starting at 0, we pass through 0 at k=100, 200, ... up to distance
                    distance / 100
                } else {
                    // Count k = start, start+100, start+200, ... that are <= distance
                    if start <= distance {
                        1 + ((distance - start) / 100)
                    } else {
                        // start > distance: no valid k in range [1, distance] equals start
                        0
                    }
                };
                count += zeros_during;
                
                position = end;
            }
            'R' => {
                let end = (position + distance) % 100;
                
                // Count zeros during rotation using mathematical calculation
                // We pass through 0 when (start + k) % 100 == 0 for k in [1, distance]
                // This means start + k = 100*n, so k = 100*n - start
                // We need: 1 <= 100*n - start <= distance
                // Rearranging: start + 1 <= 100*n <= start + distance
                let min_n = (start + 1 + 99) / 100; // ceil((start + 1) / 100)
                let max_n = (start + distance) / 100; // floor((start + distance) / 100)
                let zeros_during = (max_n - min_n + 1).max(0);
                count += zeros_during;
                
                position = end;
            }
            _ => panic!("Invalid direction: {}", direction),
        }
    }
    
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let inputs = vec![
            "L68".to_string(),
            "L30".to_string(),
            "R48".to_string(),
            "L5".to_string(),
            "R60".to_string(),
            "L55".to_string(),
            "L1".to_string(),
            "L99".to_string(),
            "R14".to_string(),
            "L82".to_string(),
        ];
        assert_eq!(part1(&inputs), 3);
    }

    #[test]
    fn test_single_rotation_to_zero() {
        // Starting at 50, rotate right 50 to get to 0
        let inputs = vec!["R50".to_string()];
        assert_eq!(part1(&inputs), 1);
    }

    #[test]
    fn test_wrap_around_left() {
        // Starting at 50, rotate left 50 to get to 0 (count 1)
        // Then rotate left 1 to get to 99
        let inputs = vec!["L50".to_string(), "L1".to_string()];
        // Start at 50, L50 -> 0 (count 1), L1 -> 99
        assert_eq!(part1(&inputs), 1);
    }

    #[test]
    fn test_wrap_around_right() {
        // Starting at 50, rotate right 50 to get to 0 (count 1)
        // Then rotate right 1 to get to 1
        let inputs = vec!["R50".to_string(), "R1".to_string()];
        assert_eq!(part1(&inputs), 1);
    }

    #[test]
    fn test_multiple_zeros() {
        // Rotate to 0 multiple times
        let inputs = vec![
            "R50".to_string(),  // 50 -> 0 (count 1)
            "R100".to_string(), // 0 -> 0 (count 2)
            "L100".to_string(), // 0 -> 0 (count 3)
        ];
        assert_eq!(part1(&inputs), 3);
    }

    #[test]
    fn test_no_zeros() {
        // Rotate but never hit 0
        let inputs = vec!["R1".to_string(), "R1".to_string(), "R1".to_string()];
        // Start at 50, R1 -> 51, R1 -> 52, R1 -> 53
        assert_eq!(part1(&inputs), 0);
    }

    #[test]
    fn test_wrap_from_99_to_0() {
        // Starting at 50, rotate right 49 to get to 99
        // Then rotate right 1 to get to 0
        let inputs = vec!["R49".to_string(), "R1".to_string()];
        // 50 -> 99, 99 -> 0 (count 1)
        assert_eq!(part1(&inputs), 1);
    }

    #[test]
    fn test_wrap_from_0_to_99() {
        // Starting at 50, rotate right 50 to get to 0 (count 1)
        // Then rotate left 1 to get to 99
        let inputs = vec!["R50".to_string(), "L1".to_string()];
        // 50 -> 0 (count 1), 0 -> 99
        assert_eq!(part1(&inputs), 1);
    }

    // Part 2 tests
    #[test]
    fn test_part2_example() {
        let inputs = vec![
            "L68".to_string(),
            "L30".to_string(),
            "R48".to_string(),
            "L5".to_string(),
            "R60".to_string(),
            "L55".to_string(),
            "L1".to_string(),
            "L99".to_string(),
            "R14".to_string(),
            "L82".to_string(),
        ];
        // Should count zeros during rotations too
        // L68 from 50: passes through 0 once (at position 0 during rotation)
        // L30 from 82: no zeros
        // R48 from 52: ends at 0 (count 1)
        // L5 from 0: no zeros during
        // R60 from 95: passes through 0 once (wraps from 99 to 0)
        // L55 from 55: ends at 0 (count 1)
        // L1 from 0: no zeros during
        // L99 from 99: ends at 0 (count 1)
        // R14 from 0: no zeros during
        // L82 from 14: passes through 0 once (wraps from 0 to 99)
        // Total: 3 (during) + 3 (at end) = 6
        assert_eq!(part2(&inputs), 6);
    }

    #[test]
    fn test_part2_r1000_from_50() {
        // Starting at 50, rotate right 1000
        // We pass through 0 at: 50 (k=50), 150 (k=150), 250, ..., 950, 1050
        // But k goes from 0 to 1000, so we pass through 0 at k=50, 150, 250, 350, 450, 550, 650, 750, 850, 950
        // That's 10 times
        let inputs = vec!["R1000".to_string()];
        assert_eq!(part2(&inputs), 10);
    }

    #[test]
    fn test_part2_single_rotation_to_zero() {
        // Starting at 50, rotate right 50
        // We pass through positions: 50, 51, ..., 99, 0
        // We pass through 0 once (at the end, k=50)
        let inputs = vec!["R50".to_string()];
        assert_eq!(part2(&inputs), 1);
    }

    #[test]
    fn test_part2_rotation_from_zero() {
        // Starting at 50, first rotate to 0, then rotate right 100
        // First rotation: R50, passes through 0 once (at end, k=50)
        // Second rotation: R100 from 0, passes through 0 once (at end, k=100)
        // We don't count the start position (k=0) as it was already counted as the end of the previous rotation
        let inputs = vec!["R50".to_string(), "R100".to_string()];
        // R50: 50 -> 0, passes through 0 once (at k=50, end)
        // R100: 0 -> 0, passes through 0 once (at k=100, end)
        assert_eq!(part2(&inputs), 2);
    }

    #[test]
    fn test_part2_left_rotation_through_zero() {
        // Starting at 50, rotate left 68
        // We pass through positions: 50, 49, ..., 1, 0, 99, 98, ..., 82
        // We pass through 0 once (during the rotation, when wrapping)
        let inputs = vec!["L68".to_string()];
        assert_eq!(part2(&inputs), 1);
    }

    #[test]
    fn test_part2_multiple_wraps() {
        // Starting at 50, rotate right 50 to get to 0, then rotate right 250
        // First: R50 from 50 -> 0, passes through 0 once (at end, k=50)
        // Second: R250 from 0 -> 50, passes through 0 at k=100 and k=200 (during rotation)
        // We don't count k=0 (start) as it was already counted as the end of the previous rotation
        // k=250 gives position 50, not 0
        let inputs = vec!["R50".to_string(), "R250".to_string()];
        // R50: 50 -> 0, passes through 0 once (at k=50, end)
        // R250: 0 -> 50, passes through 0 at k=100 and k=200 (during rotation)
        // Total: 1 + 2 = 3
        assert_eq!(part2(&inputs), 3);
    }

    #[test]
    fn test_part2_no_zeros() {
        // Rotate but never pass through 0
        let inputs = vec!["R1".to_string(), "R1".to_string(), "R1".to_string()];
        // Start at 50, R1 -> 51, R1 -> 52, R1 -> 53
        // Never pass through 0
        assert_eq!(part2(&inputs), 0);
    }

    // Tests that expose bugs in cursor_day01 implementation
    #[test]
    fn test_part2_negative_position_affects_counting() {
        // BUG: After L151 from 50, position becomes -1 (should be 99)
        // Then L50 counting logic is affected by the negative position:
        // - Buggy (position=-1): checks if -1 <= 50 (True), incorrectly counts extra zeros
        // - Correct (position=99): checks if 99 <= 50 (False), correctly counts 0 zeros
        //
        // L151 from 50: passes through 0 once (at k=50), ends at position 99
        // L50 from 99: doesn't pass through 0 (99-50=49, which is not 0)
        // Expected total: 1
        // Buggy gets: 3 (because it miscounts due to negative position)
        let inputs = vec!["L151".to_string(), "L50".to_string()];
        assert_eq!(part2(&inputs), 1);
    }

    #[test]
    fn test_part2_large_left_rotation() {
        // BUG: Left rotation with distance > position + 100 causes negative position
        // Starting at 50, rotate left 200
        // Should end at position 50 (wrapped around twice)
        // Should pass through 0 twice (at k=50 and k=150)
        let inputs = vec!["L200".to_string()];
        assert_eq!(part2(&inputs), 2);
    }

    #[test]
    fn test_part2_another_negative_position_case() {
        // BUG: L201 from 50 creates position = -51 (should be 49)
        // Then L49 should not pass through 0 (from 49, L49 lands on 0 but at the end)
        // But with position=-51, the counting logic gives wrong result
        let inputs = vec!["L201".to_string(), "L49".to_string()];
        // L201 from 50: passes through 0 at k=50 and k=150, ends at 49 (count=2)
        // L49 from 49: passes through 0 at k=49, ends at 0 (count=1)
        // Expected total: 3
        assert_eq!(part2(&inputs), 3);
    }

    #[test]
    fn test_part2_right_rotation_from_negative() {
        // BUG: After position becomes negative, right rotation counting is also affected
        // L251 from 50 creates position = -101 % 100 = -1 (should be 99)
        // Then R100: with position=-1, min_n and max_n calculations are wrong
        let inputs = vec!["L251".to_string(), "R100".to_string()];
        // L251 from 50: passes through 0 at k=50 and k=150, ends at 99 (count=2)
        // R100 from 99: passes through 0 at k=1 (when we hit 100%100=0), ends at 99 (count=1)
        // Expected total: 3
        assert_eq!(part2(&inputs), 3);
    }

}