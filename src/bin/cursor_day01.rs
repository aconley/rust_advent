/// Day 1.
fn main() -> std::io::Result<()> {
    let inputs: Vec<String> = rust_advent::read_file_as_lines("01")?;
    println!("Part 1: {}", part1(&inputs));
    Ok(())
}


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
}