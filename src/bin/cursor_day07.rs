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

    let rows = input.len();
    let cols = input[0].len();

    // Find the start position 'S'
    let mut start_col = 0;
    for (col, ch) in input[0].chars().enumerate() {
        if ch == 'S' {
            start_col = col;
            break;
        }
    }

    // Track which positions have beams (row, col)
    let mut beams = vec![vec![false; cols]; rows];
    // Track which splitters have been hit
    let mut split_splitters = std::collections::HashSet::new();

    // Initialize: beam starts at 'S' position
    beams[0][start_col] = true;

    // Process each row, propagating beams downward
    for row in 0..rows - 1 {
        for col in 0..cols {
            if beams[row][col] {
                // Check what's in the next row at this column
                let next_char = input[row + 1].chars().nth(col).unwrap();

                match next_char {
                    '.' => {
                        // Beam continues straight down
                        beams[row + 1][col] = true;
                    }
                    '^' => {
                        // Beam hits a splitter - split left and right
                        split_splitters.insert((row + 1, col));

                        // Create beam on the left (if valid)
                        if col > 0 {
                            beams[row + 1][col - 1] = true;
                        }
                        // Create beam on the right (if valid)
                        if col < cols - 1 {
                            beams[row + 1][col + 1] = true;
                        }
                    }
                    _ => {
                        // 'S' shouldn't appear in later rows, but handle gracefully
                    }
                }
            }
        }
    }

    split_splitters.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = vec![
            "..S..".to_string(),
            ".....".to_string(),
            "..^..".to_string(),
            ".....".to_string(),
        ];
        assert_eq!(part1(&input), 1);
    }

    #[test]
    fn test_example2() {
        let input = vec![
            "...S...".to_string(),
            ".......".to_string(),
            "...^...".to_string(),
            "..^...^".to_string(),
        ];
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_example3() {
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
    fn test_example4() {
        let input = vec![
            "..S..".to_string(),
            "..^..".to_string(),
            ".^.^.".to_string(),
        ];
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_single_row() {
        let input = vec!["S".to_string()];
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_no_splitters() {
        let input = vec![
            "..S..".to_string(),
            ".....".to_string(),
            ".....".to_string(),
        ];
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_splitter_at_edge() {
        let input = vec![
            ".S.".to_string(),
            "...".to_string(),
            ".^.".to_string(),
            "...".to_string(),
        ];
        assert_eq!(part1(&input), 1);
    }

    #[test]
    fn test_beam_continues_through_empty() {
        // Test that beams continue through multiple rows of empty space
        let input = vec![
            "..S..".to_string(),
            ".....".to_string(),
            ".....".to_string(),
            "..^..".to_string(),
        ];
        // Beam travels through 2 rows of empty space before hitting splitter
        assert_eq!(part1(&input), 1);
    }

    #[test]
    fn test_cascade_splits() {
        // Beam splits, then each split beam hits another splitter
        let input = vec![
            "...S...".to_string(),
            ".......".to_string(),
            "...^...".to_string(),
            "..^...^".to_string(),
            ".^...^.".to_string(),
        ];
        // Let's trace: S at col 3
        // Row 1: beam at col 3
        // Row 2: splitter at col 3 -> beams at col 2 and 4. Count: 1
        // Row 3:
        //   - Beam at col 2 hits splitter at col 2 -> beams at col 1 and 3. Count: 2
        //   - Beam at col 4 hits empty -> continues to col 4
        // Row 4:
        //   - Beam at col 1 hits splitter at col 1 -> beams at col 0 and 2. Count: 3
        //   - Beam at col 3 hits empty -> continues to col 3
        //   - Beam at col 4 hits empty -> continues to col 4
        // Total: 3 splits
        assert_eq!(part1(&input), 3);
    }

    // Part 2 tests
    #[test]
    fn test_part2_example1() {
        let input = vec![
            "..S..".to_string(),
            ".....".to_string(),
            "..^..".to_string(),
            ".....".to_string(),
        ];
        // 2 paths: one goes left, one goes right
        assert_eq!(part2(&input), 2);
    }

    #[test]
    fn test_part2_example2() {
        let input = vec![
            "...S...".to_string(),
            ".......".to_string(),
            "...^...".to_string(),
            "..^...^".to_string(),
        ];
        // 3 paths total
        assert_eq!(part2(&input), 3);
    }

    #[test]
    fn test_part2_example3() {
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
        // 40 paths
        assert_eq!(part2(&input), 40);
    }

    #[test]
    fn test_part2_no_splitters() {
        let input = vec![
            "..S..".to_string(),
            ".....".to_string(),
            ".....".to_string(),
        ];
        // One path straight down
        assert_eq!(part2(&input), 1);
    }

    #[test]
    fn test_part2_single_row() {
        let input = vec!["S".to_string()];
        // With only one row, the path can't propagate, so 0 paths
        // But our implementation counts the starting position as 1 path
        // Actually, if there's no next row, we should return 0
        // Let me check: if rows == 1, we never enter the loop, so we sum the last row
        // which has 1 path at start_col. But semantically, a path needs to propagate.
        // For consistency with "no paths formed", let's expect 0, but we need to fix the impl
        assert_eq!(part2(&input), 0);
    }

    #[test]
    fn test_part2_splitter_at_edge_left() {
        let input = vec!["S..".to_string(), "...".to_string(), "^..".to_string()];
        // Beam hits splitter at col 0, can only go right (col 1)
        assert_eq!(part2(&input), 1);
    }

    #[test]
    fn test_part2_splitter_at_edge_right() {
        let input = vec!["..S".to_string(), "...".to_string(), "..^".to_string()];
        // Beam hits splitter at right edge, can only go left (col 1)
        assert_eq!(part2(&input), 1);
    }

    #[test]
    fn test_part2_multiple_splitters_sequential() {
        // Test case where paths from first splitter don't hit second splitter
        let input = vec![
            "..S..".to_string(),
            ".....".to_string(),
            "..^..".to_string(),
            ".....".to_string(),
            "..^..".to_string(),
        ];
        // Row 0: 1 path at col 2
        // Row 1: path continues to col 2
        // Row 2: splitter at col 2 -> paths go to col 1 and 3
        // Row 3: paths continue at col 1 and 3
        // Row 4: splitter at col 2, but paths are at col 1 and 3, so they don't hit it
        // Total: 2 paths
        assert_eq!(part2(&input), 2);
    }

    #[test]
    fn test_part2_multiple_splitters_cascade() {
        // Test case where paths do hit multiple splitters
        let input = vec![
            "..S..".to_string(),
            ".....".to_string(),
            "..^..".to_string(),
            ".^.^.".to_string(),
        ];
        // Row 0: 1 path at col 2
        // Row 1: path continues to col 2
        // Row 2: splitter at col 2 -> paths go to col 1 and 3
        // Row 3:
        //   - Path at col 1 hits splitter at col 1 -> paths at col 0 and 2
        //   - Path at col 3 hits splitter at col 3 -> paths at col 2 and 4
        // Total: 4 paths (col 0, 2, 2, 4)
        assert_eq!(part2(&input), 4);
    }

    #[test]
    fn test_part2_converging_paths() {
        // Two different paths can converge on the same position
        let input = vec![
            "..S..".to_string(),
            "..^..".to_string(),
            ".^.^.".to_string(),
        ];
        // Row 0: 1 path at col 2
        // Row 1: splitter at col 2 -> 1 path each at col 1 and 3
        // Row 2:
        //   - Path at col 1 hits splitter -> paths at col 0 and 2
        //   - Path at col 3 hits splitter -> paths at col 2 and 4
        // Total: paths at col 0, 2 (from both), 4 = 4 paths
        assert_eq!(part2(&input), 4);
    }
}

/// Part 2: Count possible paths (beam takes either left or right at each splitter)
fn part2(input: &[String]) -> u64 {
    if input.is_empty() || input.len() == 1 {
        // Need at least 2 rows for a path to propagate
        return 0;
    }

    let rows = input.len();
    let cols = input[0].len();

    // Find the start position 'S'
    let mut start_col = 0;
    for (col, ch) in input[0].chars().enumerate() {
        if ch == 'S' {
            start_col = col;
            break;
        }
    }

    // Track the number of paths reaching each position (row, col)
    // Use u64 to handle large numbers of paths
    let mut paths = vec![vec![0u64; cols]; rows];

    // Initialize: one path starts at 'S' position
    paths[0][start_col] = 1;

    // Process each row, propagating paths downward
    for row in 0..rows - 1 {
        for col in 0..cols {
            if paths[row][col] > 0 {
                // Check what's in the next row at this column
                let next_char = input[row + 1].chars().nth(col).unwrap();

                match next_char {
                    '.' => {
                        // Path continues straight down
                        paths[row + 1][col] += paths[row][col];
                    }
                    '^' => {
                        // Path hits a splitter - splits into two paths (one left, one right)
                        // Each path from current position creates one path going left and one going right
                        if col > 0 {
                            paths[row + 1][col - 1] += paths[row][col];
                        }
                        if col < cols - 1 {
                            paths[row + 1][col + 1] += paths[row][col];
                        }
                    }
                    _ => {
                        // 'S' shouldn't appear in later rows, but handle gracefully
                    }
                }
            }
        }
    }

    // Sum all paths in the last row
    paths[rows - 1].iter().sum()
}
