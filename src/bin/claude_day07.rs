use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_file_as_lines("07")?;
    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
    Ok(())
}

/// Part 1: Beam splitter
///
/// Simulates a beam starting at 'S' moving downward through a grid.
/// When a beam hits a '^' splitter, it splits into two beams that continue
/// downward from positions left and right of the splitter.
/// Returns the total number of splits that occur.
fn part1(input: &[String]) -> u64 {
    if input.is_empty() {
        return 0;
    }

    // Find the starting position 'S' in the first row
    let start_col = match input[0].chars().position(|c| c == 'S') {
        Some(col) => col,
        None => return 0,
    };

    let width = input[0].len();

    // Use bitmask for efficient beam tracking (works for grids up to 64 columns)
    if width <= 64 {
        part1_bitmask(input, start_col, width)
    } else {
        // Fallback for very wide grids
        part1_vec(input, start_col, width)
    }
}

/// Efficient implementation using bitmask for beam positions
fn part1_bitmask(input: &[String], start_col: usize, width: usize) -> u64 {
    let mut active_beams = 1u64 << start_col;
    let mut split_count = 0u64;

    for row in input.iter().skip(1) {
        let row_chars: Vec<char> = row.chars().collect();
        let mut next_beams = 0u64;

        for col in 0..width.min(row_chars.len()) {
            if (active_beams & (1u64 << col)) != 0 {
                // Beam at this column
                if row_chars[col] == '^' {
                    split_count += 1;
                    if col > 0 {
                        next_beams |= 1u64 << (col - 1);
                    }
                    if col + 1 < width {
                        next_beams |= 1u64 << (col + 1);
                    }
                } else {
                    next_beams |= 1u64 << col;
                }
            }
        }

        active_beams = next_beams;
    }

    split_count
}

/// Fallback implementation using Vec for wide grids
fn part1_vec(input: &[String], start_col: usize, width: usize) -> u64 {
    let mut active_beams = vec![start_col];
    let mut next_beams = Vec::new();
    let mut split_count = 0u64;

    for row in input.iter().skip(1) {
        let row_chars: Vec<char> = row.chars().collect();
        next_beams.clear();

        for &col in &active_beams {
            if col < row_chars.len() {
                if row_chars[col] == '^' {
                    split_count += 1;
                    if col > 0 {
                        next_beams.push(col - 1);
                    }
                    if col + 1 < width {
                        next_beams.push(col + 1);
                    }
                } else {
                    next_beams.push(col);
                }
            }
        }

        next_beams.sort_unstable();
        next_beams.dedup();
        std::mem::swap(&mut active_beams, &mut next_beams);
    }

    split_count
}

/// Part 2: Count possible paths when beams make binary choices at splitters
///
/// When a beam hits a '^' splitter, it takes EITHER the left path OR the right path
/// (not both). We need to count all possible distinct paths the beam might take.
fn part2(input: &[String]) -> u64 {
    if input.is_empty() {
        return 0;
    }

    // Find the starting position 'S' in the first row
    let start_col = match input[0].chars().position(|c| c == 'S') {
        Some(col) => col,
        None => return 0,
    };

    let width = input[0].len();

    // Use bitmask for efficient state representation (works for grids up to 64 columns)
    if width <= 64 {
        part2_bitmask(input, start_col, width)
    } else {
        // Fallback for very wide grids
        part2_vec(input, start_col, width)
    }
}

/// Efficient implementation using bitmask to represent beam configurations
fn part2_bitmask(input: &[String], start_col: usize, width: usize) -> u64 {
    // State: bitmask where bit i = 1 means beam at column i
    // Map from bitmask to count of paths reaching that configuration
    let mut current_states: HashMap<u64, u64> = HashMap::new();
    current_states.insert(1u64 << start_col, 1);

    for row in input.iter().skip(1) {
        let row_chars: Vec<char> = row.chars().collect();
        let mut next_states: HashMap<u64, u64> = HashMap::new();

        for (&beams_mask, &path_count) in &current_states {
            generate_next_bitmask(beams_mask, &row_chars, path_count, width, &mut next_states);
        }

        current_states = next_states;
    }

    current_states.values().sum()
}

/// Generate all possible next beam configurations using bitmask representation
fn generate_next_bitmask(
    beams_mask: u64,
    row_chars: &[char],
    path_count: u64,
    width: usize,
    next_states: &mut HashMap<u64, u64>,
) {
    // Identify splitters and their choices
    let mut splitter_choices = Vec::new();
    let mut base_next_mask = 0u64;

    for col in 0..width.min(row_chars.len()) {
        if (beams_mask & (1u64 << col)) != 0 {
            // Beam at this column
            if row_chars[col] == '^' {
                let can_left = col > 0;
                let can_right = col + 1 < width;

                if can_left && can_right {
                    // This is a choice point
                    splitter_choices.push((col, true, true));
                } else if can_left {
                    base_next_mask |= 1u64 << (col - 1);
                } else if can_right {
                    base_next_mask |= 1u64 << (col + 1);
                }
            } else {
                // Beam continues straight
                base_next_mask |= 1u64 << col;
            }
        }
    }

    let num_choices = splitter_choices.len();

    // Generate all 2^num_choices possible configurations
    for choice_mask in 0..(1 << num_choices) {
        let mut next_mask = base_next_mask;

        for (i, &(col, _, _)) in splitter_choices.iter().enumerate() {
            let go_left = (choice_mask & (1 << i)) == 0;
            if go_left {
                next_mask |= 1u64 << (col - 1);
            } else {
                next_mask |= 1u64 << (col + 1);
            }
        }

        *next_states.entry(next_mask).or_insert(0) += path_count;
    }
}

/// Fallback implementation using Vec for wide grids
fn part2_vec(input: &[String], start_col: usize, width: usize) -> u64 {
    let mut current_states: HashMap<Vec<usize>, u64> = HashMap::new();
    current_states.insert(vec![start_col], 1);

    for row in input.iter().skip(1) {
        let row_chars: Vec<char> = row.chars().collect();
        let mut next_states: HashMap<Vec<usize>, u64> = HashMap::new();

        for (beams, path_count) in current_states {
            generate_next_vec(&beams, &row_chars, path_count, width, &mut next_states);
        }

        current_states = next_states;
    }

    current_states.values().sum()
}

/// Generate next configurations for Vec-based representation
fn generate_next_vec(
    beams: &[usize],
    row_chars: &[char],
    path_count: u64,
    width: usize,
    next_states: &mut HashMap<Vec<usize>, u64>,
) {
    let mut splitter_info = Vec::new();
    let mut non_splitter_next = Vec::new();

    for &col in beams {
        if col < row_chars.len() {
            if row_chars[col] == '^' {
                let can_go_left = col > 0;
                let can_go_right = col + 1 < width;
                splitter_info.push((col, can_go_left, can_go_right));
            } else {
                non_splitter_next.push(col);
            }
        }
    }

    let choice_splitters: Vec<_> = splitter_info
        .iter()
        .filter(|(_, left, right)| *left && *right)
        .collect();

    let num_choices = choice_splitters.len();

    for choice_mask in 0..(1 << num_choices) {
        let mut next_beams = non_splitter_next.clone();

        let mut choice_idx = 0;
        for &(col, can_go_left, can_go_right) in &splitter_info {
            if can_go_left && can_go_right {
                let go_left = (choice_mask & (1 << choice_idx)) == 0;
                choice_idx += 1;

                if go_left {
                    next_beams.push(col - 1);
                } else {
                    next_beams.push(col + 1);
                }
            } else if can_go_left {
                next_beams.push(col - 1);
            } else if can_go_right {
                next_beams.push(col + 1);
            }
        }

        next_beams.sort_unstable();
        next_beams.dedup();
        *next_states.entry(next_beams).or_insert(0) += path_count;
    }
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
    fn test_example3_beam_merging() {
        // Test case where beams merge (mentioned in problem statement)
        let input = vec![
            "..S..".to_string(),
            "..^..".to_string(),
            ".^.^.".to_string(),
        ];
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_large_example() {
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
    fn test_no_splitters() {
        let input = vec![
            "..S..".to_string(),
            ".....".to_string(),
            ".....".to_string(),
            ".....".to_string(),
        ];
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_splitter_not_hit() {
        let input = vec![
            "..S..".to_string(),
            ".....".to_string(),
            "^....".to_string(),
            ".....".to_string(),
        ];
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_single_row() {
        let input = vec!["..S..".to_string()];
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_empty_grid() {
        let input: Vec<String> = vec![];
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_cascade_splits() {
        // Each split creates beams that hit more splitters
        let input = vec![
            "...S...".to_string(),
            "...^...".to_string(),
            "..^.^..".to_string(),
            ".^.^.^.".to_string(),
        ];
        // Row 1: col 3 hits ^, splits = 1, beams at {2, 4}
        // Row 2: col 2 hits ^, col 4 hits ^, splits = 3, beams at {1, 3, 3, 5} = {1, 3, 5}
        // Row 3: col 1 hits ^, col 3 hits ^, col 5 hits ^, splits = 6
        assert_eq!(part1(&input), 6);
    }

    #[test]
    fn test_boundary_splits() {
        // Splitter at edge creates only one new beam
        let input = vec![
            "S....".to_string(),
            "^....".to_string(),
            ".....".to_string(),
        ];
        // Beam at col 0 hits ^, can only split right to col 1
        assert_eq!(part1(&input), 1);
    }

    #[test]
    fn test_complex_merging() {
        // Multiple beams converge to same column
        let input = vec![
            "....S....".to_string(),
            "....^....".to_string(),
            "...^.^...".to_string(),
            "....^....".to_string(),
        ];
        // Row 1: col 4 hits ^, splits = 1, beams at {3, 5}
        // Row 2: col 3 hits ^, col 5 hits ^, splits = 3, beams at {2, 4, 4, 6} = {2, 4, 6}
        // Row 3: col 4 hits ^, splits = 4
        assert_eq!(part1(&input), 4);
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
        // 1 splitter hit -> 2 choices (left or right)
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
        // First splitter: 2 choices
        // - Left path hits another splitter: 2 choices
        // - Right path doesn't hit splitter: 1 choice
        // Total: 2 + 1 = 3
        assert_eq!(part2(&input), 3);
    }

    #[test]
    fn test_part2_large_example() {
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
    fn test_part2_no_splitters() {
        let input = vec![
            "..S..".to_string(),
            ".....".to_string(),
            ".....".to_string(),
        ];
        // No splitters -> only 1 path
        assert_eq!(part2(&input), 1);
    }

    #[test]
    fn test_part2_two_splitters_same_row() {
        let input = vec![
            "....S....".to_string(),
            "....^....".to_string(),
            "...^.^...".to_string(),
        ];
        // First row: 1 splitter -> 2 choices (left to col 3, right to col 5)
        // Second row:
        //   - If left (col 3): hits splitter, 2 choices
        //   - If right (col 5): hits splitter, 2 choices
        // Total: 2 + 2 = 4
        assert_eq!(part2(&input), 4);
    }

    #[test]
    fn test_part2_beam_merging() {
        // Test that when beams merge, we count paths correctly
        let input = vec![
            "...S...".to_string(),
            "..^.^..".to_string(),
            "...^...".to_string(),
        ];
        // Row 1: beam at col 3
        // Row 2: two splitters at cols 2 and 4
        //   Left splitter: beam continues to col 3
        //   Right splitter: beam doesn't hit (beam is at col 3, splitters at 2 and 4)
        // Actually, beam at col 3 doesn't hit either splitter on row 2
        // So only 1 path, then hits splitter on row 3: 2 paths
        assert_eq!(part2(&input), 2);
    }

    #[test]
    fn test_part2_boundary() {
        let input = vec![
            "S....".to_string(),
            "^....".to_string(),
            ".....".to_string(),
        ];
        // Splitter at col 0 can only go right (boundary)
        assert_eq!(part2(&input), 1);
    }

    #[test]
    fn test_part2_multiple_simultaneous_choices() {
        let input = vec![
            "..S..".to_string(),
            "..^..".to_string(),
            ".^.^.".to_string(),
        ];
        // Row 1: beam at col 2 hits splitter -> 2 paths (col 1, col 3)
        // Row 2:
        //   - Path with beam at col 1: hits splitter at col 1 -> 2 subpaths
        //   - Path with beam at col 3: hits splitter at col 3 -> 2 subpaths
        // Total: 2 + 2 = 4
        assert_eq!(part2(&input), 4);
    }

    #[test]
    fn test_part2_exponential_growth() {
        // Test with multiple sequential splitters (exponential growth)
        let input = vec![
            "..S..".to_string(),
            "..^..".to_string(),
            ".^.^.".to_string(),
            "^.^.^".to_string(),
        ];
        // This tests exponential path growth through multiple layers
        // Row 1: 1 beam -> 2 paths
        // Row 2: each path can split into 2 -> 4 paths (some may merge)
        // Row 3: further splitting
        let result = part2(&input);
        assert!(result > 4); // Should have significant path count
    }

    #[test]
    fn test_part2_single_row() {
        let input = vec!["..S..".to_string()];
        assert_eq!(part2(&input), 1);
    }

    #[test]
    fn test_part2_empty() {
        let input: Vec<String> = vec![];
        assert_eq!(part2(&input), 0);
    }
}
