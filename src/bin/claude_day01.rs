/// Day 1.
fn main() -> std::io::Result<()> {
    let inputs: Vec<String> = rust_advent::read_file_as_lines("01")?;
    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
    Ok(())
}


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

fn part2(_inputs: &[String]) -> i32 {
    // TODO: implement part 2
    0
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
            "L50".to_string(), // 50 -> 0 (count = 1)
            "R100".to_string(), // 0 -> 0 (count = 2)
            "L100".to_string(), // 0 -> 0 (count = 3)
        ];
        assert_eq!(part1(&input), 3);
    }
}