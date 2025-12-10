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
fn part1(inputs: &[String]) -> usize {
    let mut current_pos: i32 = 50;
    let mut zero_count: usize = 0;

    for rotation in inputs {
        if rotation.is_empty() {
            continue;
        }

        let direction = rotation.chars().next().unwrap();
        // Parse the numeric part. 
        // We assume the input is well-formed as "L<num>" or "R<num>".
        let amount_str = &rotation[1..];
        let amount: i32 = match amount_str.parse() {
            Ok(num) => num,
            Err(_) => continue, // Skip malformed lines if any
        };

        match direction {
            'L' => {
                // Moving left: subtract amount. Use rem_euclid for correct negative wrapping.
                current_pos = (current_pos - amount).rem_euclid(100);
            },
            'R' => {
                // Moving right: add amount.
                current_pos = (current_pos + amount).rem_euclid(100);
            },
            _ => {
                // Unknown direction, ignore or panic. Ignoring for robustness.
            }
        }

        if current_pos == 0 {
            zero_count += 1;
        }
    }

    zero_count
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
fn part2(inputs: &[String]) -> i64 {
    let mut current_pos: i64 = 50;
    let mut zero_count: i64 = 0;

    for rotation in inputs {
        if rotation.is_empty() {
            continue;
        }

        let direction = rotation.chars().next().unwrap();
        let amount_str = &rotation[1..];
        let amount: i64 = match amount_str.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match direction {
            'L' => {
                // Range visited: [current_pos - amount, current_pos - 1]
                // Number of multiples of 100 in [A, B] is floor(B/100) - floor((A-1)/100)
                let upper = current_pos - 1;
                let lower = current_pos - amount;
                
                let hits = upper.div_euclid(100) - (lower - 1).div_euclid(100);
                zero_count += hits;

                current_pos = (current_pos - amount).rem_euclid(100);
            },
            'R' => {
                // Range visited: [current_pos + 1, current_pos + amount]
                // Note: current_pos + 1 is the first visited, current_pos + amount is last.
                // Number of multiples of 100 in [A, B] is floor(B/100) - floor((A-1)/100)
                // A = current_pos + 1
                // B = current_pos + amount
                // Formula: floor((current_pos + amount)/100) - floor((current_pos + 1 - 1)/100)
                //        = floor((current_pos + amount)/100) - floor(current_pos/100)
                
                let hits = (current_pos + amount).div_euclid(100) - current_pos.div_euclid(100);
                zero_count += hits;
                
                current_pos = (current_pos + amount).rem_euclid(100);
            },
            _ => {}
        }
    }

    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01_example() {
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
    fn test_wrap_around_simple() {
        let inputs = vec![
            "L50".to_string(), // 50 -> 0 (count 1)
            "R99".to_string(), // 0 -> 99
            "R1".to_string(),  // 99 -> 0 (count 2)
        ];
        assert_eq!(part1(&inputs), 2);
    }

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
        assert_eq!(part2(&inputs), 6);
    }

    #[test]
    fn test_part2_large_rotation() {
        // Start 50.
        // R1000:
        // Range (50, 1050].
        // Multiples of 100: 100, 200, ..., 1000.
        // There are 10 multiples.
        let inputs = vec!["R1000".to_string()];
        assert_eq!(part2(&inputs), 10);
    }

    #[test]
    fn test_part2_large_rotation_left() {
        // Start 50.
        // L1000:
        // Range [50 - 1000, 49] = [-950, 49].
        // Multiples of 100: 0, -100, ..., -900.
        // 0, -100, -200, -300, -400, -500, -600, -700, -800, -900.
        // Total 10.
        let inputs = vec!["L1000".to_string()];
        assert_eq!(part2(&inputs), 10);
    }
}
