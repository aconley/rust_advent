fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_file_as_lines("07")?;
    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
    Ok(())
}

/// Part 1: Beam splitter
fn part1(input: &[String]) -> u64 {
    if input.is_empty() {
        return 0;
    }

    let width = input[0].len();
    let mut current_beams = vec![false; width];
    let mut next_beams = vec![false; width];
    let mut total_splits = 0;

    // Find the starting position 'S' in the first row.
    for (c, &byte) in input[0].as_bytes().iter().enumerate() {
        if byte == b'S' {
            current_beams[c] = true;
            break;
        }
    }

    // Process from the second row onwards.
    for row_str in input.iter().skip(1) {
        let row_bytes = row_str.as_bytes();
        next_beams.fill(false);
        let mut row_has_beams = false;

        for c in 0..width {
            if current_beams[c] {
                if row_bytes[c] == b'^' {
                    total_splits += 1;
                    if c > 0 {
                        next_beams[c - 1] = true;
                        row_has_beams = true;
                    }
                    if c + 1 < width {
                        next_beams[c + 1] = true;
                        row_has_beams = true;
                    }
                } else {
                    next_beams[c] = true;
                    row_has_beams = true;
                }
            }
        }
        std::mem::swap(&mut current_beams, &mut next_beams);
        if !row_has_beams {
            break;
        }
    }

    total_splits
}

fn part2(input: &[String]) -> u64 {
    if input.is_empty() {
        return 0;
    }

    let width = input[0].len();
    let mut ways = vec![0u64; width];
    let mut next_ways = vec![0u64; width];

    // Find the starting position 'S' in the first row.
    for (c, &byte) in input[0].as_bytes().iter().enumerate() {
        if byte == b'S' {
            ways[c] = 1;
            break;
        }
    }

    // Process from the second row onwards.
    for row_str in input.iter().skip(1) {
        let row_bytes = row_str.as_bytes();
        next_ways.fill(0);
        let mut row_has_ways = false;

        for c in 0..width {
            let w = ways[c];
            if w == 0 {
                continue;
            }

            if row_bytes[c] == b'^' {
                if c > 0 {
                    next_ways[c - 1] += w;
                    row_has_ways = true;
                }
                if c + 1 < width {
                    next_ways[c + 1] += w;
                    row_has_ways = true;
                }
            } else {
                next_ways[c] += w;
                row_has_ways = true;
            }
        }
        std::mem::swap(&mut ways, &mut next_ways);
        if !row_has_ways {
            break;
        }
    }

    ways.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = vec![
            "..S..".to_string(),
            ".....".to_string(),
            "..^..".to_string(),
            ".....".to_string(),
        ];
        assert_eq!(part1(&input), 1);
    }

    #[test]
    fn test_example_2() {
        let input = vec![
            "...S...".to_string(),
            ".......".to_string(),
            "...^...".to_string(),
            "..^...^".to_string(),
        ];
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_example_3() {
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
    fn test_merging_beams() {
        let input = vec![
            "..S..".to_string(),
            "..^..".to_string(),
            ".^.^.".to_string(),
        ];
        // Row 0: S at 2
        // Row 1: ^ at 2 -> split, total_splits = 1, beams at 1, 3
        // Row 2: ^ at 1, 3 -> split both, total_splits = 3, beams at (0, 2), (2, 4) -> {0, 2, 4}
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_edge_of_grid() {
        let input = vec!["S..".to_string(), "^..".to_string(), "...".to_string()];
        // Row 0: S at 0
        // Row 1: ^ at 0 -> split, total_splits = 1, beams at 1 (0-1 is out of bounds)
        assert_eq!(part1(&input), 1);
    }

    #[test]
    fn test_no_beams_hit_splitter() {
        let input = vec![
            "..S..".to_string(),
            ".....".to_string(),
            "^...^".to_string(),
        ];
        assert_eq!(part1(&input), 0);
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
    fn test_part2_example_3() {
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

    #[test]
    fn test_part2_merging_paths() {
        let input = vec![
            "..S..".to_string(),
            "..^..".to_string(),
            ".^.^.".to_string(),
        ];
        // Row 0: ways[2] = 1
        // Row 1: ^ at 2. next_ways[1]=1, next_ways[3]=1
        // Row 2: ^ at 1 and 3.
        // ways[1] split -> next_ways[0]+=1, next_ways[2]+=1
        // ways[3] split -> next_ways[2]+=1, next_ways[4]+=1
        // Row 2 final ways: [1, 0, 2, 0, 1]
        assert_eq!(part2(&input), 4);
    }
}
