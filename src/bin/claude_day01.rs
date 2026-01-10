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

    for line in inputs {
        let direction = &line[0..1];
        let distance: i32 = line[1..].parse().unwrap();

        if direction == "L" {
            position = (position - distance).rem_euclid(100);
        } else {
            position = (position + distance).rem_euclid(100);
        }

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
fn part2(inputs: &[String]) -> i32 {
    let mut position = 50;
    let mut count = 0;

    for line in inputs {
        let direction = &line[0..1];
        let distance: i32 = line[1..].parse().unwrap();

        if direction == "L" {
            // Count zeros during left rotation
            // Formula depends on whether we start at 0 or not
            if position == 0 {
                count += distance / 100;
            } else if distance >= position {
                count += 1 + (distance - position) / 100;
            }
            position = (position - distance).rem_euclid(100);
        } else {
            // Count zeros during right rotation
            // Number of times we pass through 0 = floor((start + dist) / 100)
            count += (position + distance) / 100;
            position = (position + distance).rem_euclid(100);
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = vec![
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
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_dial_wrapping() {
        // Test wrapping from 0 to 99 (left)
        let input = vec!["L50".to_string(), "L1".to_string()];
        assert_eq!(part1(&input), 1); // 50 -> 0 -> 99

        // Test wrapping from 99 to 0 (right)
        let input = vec!["R49".to_string(), "R1".to_string()];
        assert_eq!(part1(&input), 1); // 50 -> 99 -> 0
    }

    #[test]
    fn test_multiple_zeros() {
        // Test landing on 0 multiple times
        let input = vec![
            "L50".to_string(),  // 50 -> 0 (count = 1)
            "R100".to_string(), // 0 -> 0 (count = 2)
            "L100".to_string(), // 0 -> 0 (count = 3)
        ];
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part2_example() {
        let input = vec![
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
        // Expected: 3 at end of rotations + 3 during rotations = 6
        assert_eq!(part2(&input), 6);
    }

    #[test]
    fn test_part2_multiple_wraps() {
        // Test R1000 from position 50 - should pass through 0 ten times
        let input = vec!["R1000".to_string()];
        assert_eq!(part2(&input), 10);
    }

    #[test]
    fn test_part2_left_from_zero() {
        // Test rotating left from position 0
        let input = vec![
            "L50".to_string(),  // 50 -> 0 (1 zero)
            "L100".to_string(), // 0 -> 0 via full rotation (1 zero)
        ];
        assert_eq!(part2(&input), 2);
    }

    #[test]
    fn test_part2_no_intermediate_zeros() {
        // Test rotation that doesn't pass through 0
        let input = vec!["R10".to_string()]; // 50 -> 60, no zeros
        assert_eq!(part2(&input), 0);
    }

    #[test]
    fn test_part2_negative_position_affects_counting() {
        let inputs = vec!["L151".to_string(), "L50".to_string()];
        assert_eq!(part2(&inputs), 2);
    }

    #[test]
    fn test_part2_large_left_rotation() {
        let inputs = vec!["L200".to_string()];
        assert_eq!(part2(&inputs), 2);
    }

    #[test]
    fn test_part2_another_negative_position_case() {
        let inputs = vec!["L201".to_string(), "L49".to_string()];
        assert_eq!(part2(&inputs), 3);
    }

    #[test]
    fn test_part2_right_rotation_from_negative() {
        let inputs = vec!["L251".to_string(), "R100".to_string()];
        assert_eq!(part2(&inputs), 4);
    }
}
