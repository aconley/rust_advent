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

        // Count full rotations
        count += distance / 100;

        let rem = distance % 100;

        if direction == "L" {
            if position > 0 && rem >= position {
                count += 1;
            }
            position = (position - rem).rem_euclid(100);
        } else {
            if position + rem >= 100 {
                count += 1;
            }
            position = (position + rem).rem_euclid(100);
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
    fn test_wrapping() {
        // Start at 50.
        // L50 -> 0 (count 1)
        // L1 -> 99
        // R1 -> 0 (count 2)
        let input = vec!["L50".to_string(), "L1".to_string(), "R1".to_string()];
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_full_rotation() {
        // Start at 50.
        // R100 -> 50 (no 0)
        // L100 -> 50 (no 0)
        // L50 -> 0 (count 1)
        // R100 -> 0 (count 2)
        let input = vec![
            "R100".to_string(),
            "L100".to_string(),
            "L50".to_string(),
            "R100".to_string(),
        ];
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_no_zeros() {
        // Start at 50.
        // R10 -> 60
        // L20 -> 40
        let input = vec!["R10".to_string(), "L20".to_string()];
        assert_eq!(part1(&input), 0);
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
        assert_eq!(part2(&input), 6);
    }

    #[test]
    fn test_part2_full_rotation() {
        // R1000 from 50 should hit 0 ten times
        let input = vec!["R1000".to_string()];
        assert_eq!(part2(&input), 10);
    }

    #[test]
    fn test_part2_start_zero_left() {
        // Start 50. L50 -> 0 (count 1).
        // L100 -> 0 (count 1).
        let input = vec!["L50".to_string(), "L100".to_string()];
        assert_eq!(part2(&input), 2);
    }

    #[test]
    fn test_part2_start_zero_right() {
        // Start 50. R50 -> 0 (count 1).
        // R100 -> 0 (count 1).
        let input = vec!["R50".to_string(), "R100".to_string()];
        assert_eq!(part2(&input), 2);
    }
}
