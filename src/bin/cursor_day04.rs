// Directions for 8 neighbors: (row_offset, col_offset)
const NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1), // top row
    (0, -1),
    (0, 1), // left, right
    (1, -1),
    (1, 0),
    (1, 1), // bottom row
];

/// Counts the number of adjacent '@' objects for a given position.
/// Returns the count, stopping early once it reaches 4 for performance.
fn count_adjacent_objects(grid: &[&[u8]], i: usize, j: usize, rows: usize, cols: usize) -> u32 {
    let mut count = 0;
    for (di, dj) in NEIGHBOR_OFFSETS {
        let ni = i as i32 + di;
        let nj = j as i32 + dj;

        // Check bounds before converting to usize
        if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
            if grid[ni as usize][nj as usize] == b'@' {
                count += 1;
                // Early exit: once we have 4 neighbors, no need to check more
                if count >= 4 {
                    break;
                }
            }
        }
    }
    count
}

/// Counts adjacent objects for a mutable grid (same logic as above but for Vec<Vec<u8>>).
fn count_adjacent_objects_mut(
    grid: &[Vec<u8>],
    i: usize,
    j: usize,
    rows: usize,
    cols: usize,
) -> u32 {
    let mut count = 0;
    for (di, dj) in NEIGHBOR_OFFSETS {
        let ni = i as i32 + di;
        let nj = j as i32 + dj;

        // Check bounds before converting to usize
        if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
            if grid[ni as usize][nj as usize] == b'@' {
                count += 1;
                // Early exit: once we have 4 neighbors, no need to check more
                if count >= 4 {
                    break;
                }
            }
        }
    }
    count
}

fn main() -> std::io::Result<()> {
    let inputs: Vec<String> = rust_advent::read_file_as_lines("04")?;
    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
    Ok(())
}

/// Part 1: Count the number of objects (@) with fewer than 4 adjacent objects.
///
/// Given a grid represented as a &[String], where @ is an object and .
/// an empty space, count the number of objects that have fewer than 4 adjacent
/// objects, not including itself, and where grid positions are adjacent
/// horizontally, vertically, and diagonally.
fn part1(inputs: &[String]) -> usize {
    if inputs.is_empty() {
        return 0;
    }

    let rows = inputs.len();
    let cols = inputs[0].len();

    // Convert to byte slices for efficient indexing (since '@' and '.' are ASCII)
    let grid: Vec<&[u8]> = inputs.iter().map(|s| s.as_bytes()).collect();

    let mut count = 0;

    for i in 0..rows {
        let row = grid[i];
        for j in 0..cols {
            // Only process '@' characters
            if row[j] != b'@' {
                continue;
            }

            let adjacent_count = count_adjacent_objects(&grid, i, j, rows, cols);
            if adjacent_count < 4 {
                count += 1;
            }
        }
    }

    count
}

/// Part 2: Count the number of objects (@) that can be removed.
///
/// Given a grid represented as a &[String], where @ is an object and .
/// an empty space, remove objects that have fewer than 4 adjacent
/// objects, not including itself, and where grid positions are adjacent
/// horizontally, vertically, and diagonally.  Removing objects may make
/// it possible to remove additional objects -- which should also be removed.
///
/// The return value should be the total number of objects removed.
fn part2(inputs: &[String]) -> usize {
    if inputs.is_empty() {
        return 0;
    }

    let rows = inputs.len();
    let cols = inputs[0].len();

    // Create a mutable grid (copy the input)
    let mut grid: Vec<Vec<u8>> = inputs.iter().map(|s| s.as_bytes().to_vec()).collect();

    let mut total_removed = 0;

    // Track which cells need to be checked in the next iteration
    // Initially, we check all cells. After that, only neighbors of removed cells.
    let mut to_check: std::collections::HashSet<(usize, usize)> = (0..rows)
        .flat_map(|i| (0..cols).map(move |j| (i, j)))
        .collect();

    // Iteratively remove objects until no more can be removed
    loop {
        // Find all objects to remove in this iteration
        let mut to_remove = Vec::new();

        // Only check cells that might have changed (or all cells on first iteration)
        for &(i, j) in &to_check {
            // Only consider '@' characters
            if grid[i][j] != b'@' {
                continue;
            }

            let adjacent_count = count_adjacent_objects_mut(&grid, i, j, rows, cols);
            if adjacent_count < 4 {
                to_remove.push((i, j));
            }
        }

        // If nothing to remove, we're done
        if to_remove.is_empty() {
            break;
        }

        // Remove marked objects (two-phase approach: collect then remove for correctness)
        let removed_this_iteration = to_remove.len();
        for (i, j) in &to_remove {
            grid[*i][*j] = b'.';
        }

        total_removed += removed_this_iteration;

        // For next iteration, only check neighbors of removed cells
        // (these are the only cells whose neighbor count could have changed)
        to_check.clear();
        for (i, j) in &to_remove {
            for (di, dj) in NEIGHBOR_OFFSETS {
                let ni = *i as i32 + di;
                let nj = *j as i32 + dj;

                if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
                    to_check.insert((ni as usize, nj as usize));
                }
            }
        }
    }

    total_removed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() {
        let grid = vec!["..@".to_string(), ".@.".to_string(), ".@@".to_string()];
        assert_eq!(part1(&grid), 4);
    }

    #[test]
    fn test_larger_example() {
        let grid = vec![
            "..@@.@@@@.".to_string(),
            "@@@.@.@.@@".to_string(),
            "@@@@@.@.@@".to_string(),
            "@.@@@@..@.".to_string(),
            "@@.@@@@.@@".to_string(),
            ".@@@@@@@.@".to_string(),
            ".@.@.@.@@@".to_string(),
            "@.@@@.@@@@".to_string(),
            ".@@@@@@@@.".to_string(),
            "@.@.@@@.@.".to_string(),
        ];
        assert_eq!(part1(&grid), 13);
    }

    #[test]
    fn test_empty_grid() {
        let grid: Vec<String> = vec![];
        assert_eq!(part1(&grid), 0);
    }

    #[test]
    fn test_single_object() {
        let grid = vec!["@".to_string()];
        assert_eq!(part1(&grid), 1);
    }

    #[test]
    fn test_no_objects() {
        let grid = vec!["...".to_string(), "...".to_string()];
        assert_eq!(part1(&grid), 0);
    }

    #[test]
    fn test_object_with_4_neighbors() {
        // Create a grid where the center object has exactly 4 neighbors
        let grid = vec![".@.".to_string(), "@@@".to_string(), ".@.".to_string()];
        // The center @ has 4 neighbors, so it shouldn't be counted
        // The 4 corner @ have 2 neighbors each, so they should be counted
        assert_eq!(part1(&grid), 4);
    }

    #[test]
    fn test_part2_larger_example() {
        // The main example from the prompt - should remove 43 objects total
        let grid = vec![
            "..@@.@@@@.".to_string(),
            "@@@.@.@.@@".to_string(),
            "@@@@@.@.@@".to_string(),
            "@.@@@@..@.".to_string(),
            "@@.@@@@.@@".to_string(),
            ".@@@@@@@.@".to_string(),
            ".@.@.@.@@@".to_string(),
            "@.@@@.@@@@".to_string(),
            ".@@@@@@@@.".to_string(),
            "@.@.@@@.@.".to_string(),
        ];
        assert_eq!(part2(&grid), 43);
    }

    #[test]
    fn test_part2_empty_grid() {
        let grid: Vec<String> = vec![];
        assert_eq!(part2(&grid), 0);
    }

    #[test]
    fn test_part2_single_object() {
        // Single object has 0 neighbors, so it gets removed
        let grid = vec!["@".to_string()];
        assert_eq!(part2(&grid), 1);
    }

    #[test]
    fn test_part2_no_objects() {
        let grid = vec!["...".to_string(), "...".to_string()];
        assert_eq!(part2(&grid), 0);
    }

    #[test]
    fn test_part2_small_example() {
        // Small grid - all 4 objects should be removed since they all have < 4 neighbors
        let grid = vec!["..@".to_string(), ".@.".to_string(), ".@@".to_string()];
        assert_eq!(part2(&grid), 4);
    }

    #[test]
    fn test_part2_all_removed() {
        // Grid where all objects will eventually be removed
        // Initially, corner objects have 1 neighbor each, so they're removed
        // Then remaining objects may also become removable
        let grid = vec!["@.@".to_string(), "...".to_string(), "@.@".to_string()];
        // All 4 corner @ have 0 neighbors, so all 4 get removed
        assert_eq!(part2(&grid), 4);
    }

    #[test]
    fn test_part2_stable_structure() {
        // A 3x3 grid where corners have 3 neighbors (< 4), so they're removed first
        // Then edges have fewer neighbors, so they get removed
        // Finally center has 0 neighbors, so it gets removed too
        let grid = vec!["@@@".to_string(), "@@@".to_string(), "@@@".to_string()];
        // Iteration 1: corners (3 neighbors each) -> 4 removed
        // Iteration 2: edges (now have < 4) -> 4 removed
        // Iteration 3: center (now has 0) -> 1 removed
        // Total: 9 removed
        assert_eq!(part2(&grid), 9);
    }

    #[test]
    fn test_part2_iterative_removal() {
        // Test case that requires multiple iterations
        // First iteration: remove objects with < 4 neighbors
        // Subsequent iterations: remove objects that lost neighbors
        let grid = vec![
            "@.@.@".to_string(),
            "..@..".to_string(),
            "@.@.@".to_string(),
        ];
        // Iteration 1:
        //   - 6 outer @ have 1 neighbor each -> removed (top-left, top-middle, top-right, bottom-left, bottom-middle, bottom-right)
        //   - Center @ has 4 neighbors -> stays
        // Iteration 2:
        //   - Center @ now has 0 neighbors -> removed
        // Total: 7 removed
        assert_eq!(part2(&grid), 7);
    }

    #[test]
    fn test_part2_cluster_remains() {
        // A 2x2 cluster where each @ has 3 neighbors - all should be removed
        let grid = vec!["@@".to_string(), "@@".to_string()];
        // All 4 @ have 3 neighbors, so all 4 get removed
        assert_eq!(part2(&grid), 4);
    }
}
