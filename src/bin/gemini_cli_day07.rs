
fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_file_as_lines("07")?;
    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
    Ok(())
}

fn part1(input: &[String]) -> u64 {
    if input.is_empty() {
        return 0;
    }

    let cols = input[0].len();
    
    // Use Vec<bool> for efficient tracking without hashing overhead
    // Double buffering to avoid allocation in the loop
    let mut beams = vec![false; cols];
    let mut next_beams = vec![false; cols];
    
    let mut found_s = false;
    for (c, char) in input[0].chars().enumerate() {
        if char == 'S' {
            beams[c] = true;
            found_s = true;
            break;
        }
    }
    
    if !found_s {
        return 0;
    }

    let mut split_count = 0;

    for row_str in input.iter().skip(1) {
        next_beams.fill(false);
        let row_bytes = row_str.as_bytes();
        let mut active = false;

        for c in 0..cols {
            if !beams[c] { continue; }
            active = true;

            match row_bytes[c] {
                b'^' => {
                    split_count += 1;
                    if c > 0 {
                        next_beams[c - 1] = true;
                    }
                    if c + 1 < cols {
                        next_beams[c + 1] = true;
                    }
                },
                _ => {
                    next_beams[c] = true;
                }
            }
        }
        
        if !active {
            break;
        }
        std::mem::swap(&mut beams, &mut next_beams);
    }

    split_count
}

fn part2(input: &[String]) -> u64 {
    if input.is_empty() {
        return 0;
    }

    let cols = input[0].len();
    
    // Double buffering for counts
    let mut counts: Vec<u64> = vec![0; cols];
    let mut next_counts: Vec<u64> = vec![0; cols];
    
    // Initialize start position
    let mut found_s = false;
    for (c, char) in input[0].chars().enumerate() {
        if char == 'S' {
            counts[c] = 1;
            found_s = true;
            break;
        }
    }
    
    if !found_s {
        return 0;
    }

    for row_str in input.iter().skip(1) {
        next_counts.fill(0);
        let row_bytes = row_str.as_bytes();
        let mut active = false;

        for c in 0..cols {
            let count = counts[c];
            if count == 0 { continue; }
            active = true;
            
            match row_bytes[c] {
                b'^' => {
                    // Split: goes to left and right in next row
                    if c > 0 {
                        next_counts[c - 1] += count;
                    }
                    if c + 1 < cols {
                        next_counts[c + 1] += count;
                    }
                },
                _ => {
                    // Pass through: goes straight down
                    next_counts[c] += count;
                }
            }
        }
        
        // Optimization: if no beams active, stop
        if !active {
            break;
        }
        std::mem::swap(&mut counts, &mut next_counts);
    }

    counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example_1() {
        let input = vec![
            "..S..".to_string(),
            ".....".to_string(),
            "..^..".to_string(),
            ".....".to_string(),
        ];
        assert_eq!(part1(&input), 1);
    }

    #[test]
    fn test_part1_example_2() {
        let input = vec![
            "...S...".to_string(),
            ".......".to_string(),
            "...^...".to_string(),
            "..^...^".to_string(),
        ];
        assert_eq!(part1(&input), 2);
    }
    
    #[test]
    fn test_part1_example_3_large() {
        let input = vec![
            ".......S.......".to_string(),
            "...............".to_string(),
            ".......^.......".to_string(),
            "...............".to_string(),
            "......^.^......".to_string(),
            "...............".to_string(),
            ".....^.^.^.....".to_string(),
            "...............".to_string(),
            "....^.^...^....".to_string(),
            "...............".to_string(),
            "...^.^...^.^...".to_string(),
            "...............".to_string(),
            "..^...^.....^..".to_string(),
            "...............".to_string(),
            ".^.^.^.^.^...^.".to_string(),
            "...............".to_string(),
        ];
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn test_part1_merge() {
        let input = vec![
            "..S..".to_string(),
            "..^..".to_string(),
            ".^.^.".to_string(),
        ];
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part2_example_1() {
        let input = vec![
            "..S..".to_string(),
            ".....".to_string(),
            "..^..".to_string(),
            ".....".to_string(),
        ];
        assert_eq!(part2(&input), 2);
    }

    #[test]
    fn test_part2_example_2() {
        let input = vec![
            "...S...".to_string(),
            ".......".to_string(),
            "...^...".to_string(),
            "..^...^".to_string(),
        ];
        assert_eq!(part2(&input), 3);
    }

    #[test]
    fn test_part2_example_3_large() {
        let input = vec![
            ".......S.......".to_string(),
            "...............".to_string(),
            ".......^.......".to_string(),
            "...............".to_string(),
            "......^.^......".to_string(),
            "...............".to_string(),
            ".....^.^.^.....".to_string(),
            "...............".to_string(),
            "....^.^...^....".to_string(),
            "...............".to_string(),
            "...^.^...^.^...".to_string(),
            "...............".to_string(),
            "..^...^.....^..".to_string(),
            "...............".to_string(),
            ".^.^.^.^.^...^.".to_string(),
            "...............".to_string(),
        ];
        assert_eq!(part2(&input), 40);
    }
}