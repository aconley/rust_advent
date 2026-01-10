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

    // Pre-convert grid to 2D byte array for efficient access
    // Since input only contains ASCII characters (@ and .), bytes are more efficient than chars
    let grid: Vec<&[u8]> = inputs.iter().map(|line| line.as_bytes()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == b'@' && has_fewer_than_n_neighbors(&grid, row, col, rows, cols, 4)
            {
                count += 1;
            }
        }
    }

    count
}

/// Core neighbor counting logic using a closure to access grid cells.
/// Returns early once 4 neighbors are found for efficiency.
/// This is used by both immutable and mutable grid checking functions.
fn count_neighbors<F>(get_cell: F, row: usize, col: usize, rows: usize, cols: usize) -> usize
where
    F: Fn(usize, usize) -> u8,
{
    const DIRECTIONS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut count = 0;
    for (dr, dc) in DIRECTIONS.iter() {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        if new_row >= 0
            && new_row < rows as i32
            && new_col >= 0
            && new_col < cols as i32
            && get_cell(new_row as usize, new_col as usize) == b'@'
        {
            count += 1;
            // Early exit optimization: stop counting after 4
            if count >= 4 {
                return count;
            }
        }
    }
    count
}

/// Check if a position has fewer than `threshold` adjacent objects.
/// Returns early once threshold is reached for efficiency.
fn has_fewer_than_n_neighbors(
    grid: &[&[u8]],
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
    threshold: usize,
) -> bool {
    count_neighbors(|r, c| grid[r][c], row, col, rows, cols) < threshold
}

/// Part 2: Count the number of objects (@) that can be removed.
///
/// Given a grid represented as a &[String], where @ is an object and .
/// an empty space, remove objects that have fewer than 4 adjacent
/// objects, not including itself, and where grid positions are adjacent
/// horizontally, vertically, and diagonally.  Removing objects may make
/// it possible to remove additional objects -- which should also be removed.
///
/// The return value should be the number removed.
fn part2(inputs: &[String]) -> usize {
    if inputs.is_empty() {
        return 0;
    }

    // Create mutable grid for iterative removal
    let mut grid: Vec<Vec<u8>> = inputs.iter().map(|line| line.as_bytes().to_vec()).collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_removed = 0;

    loop {
        // Find all positions to remove in this iteration
        let mut to_remove = Vec::new();

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == b'@'
                    && has_fewer_than_n_neighbors_mut(&grid, row, col, rows, cols, 4)
                {
                    to_remove.push((row, col));
                }
            }
        }

        // If no objects can be removed, we're done
        if to_remove.is_empty() {
            break;
        }

        // Remove all marked objects
        for (row, col) in &to_remove {
            grid[*row][*col] = b'.';
        }

        total_removed += to_remove.len();
    }

    total_removed
}

/// Check if a position in a mutable grid has fewer than `threshold` adjacent objects.
/// Uses the shared neighbor counting logic via closure.
fn has_fewer_than_n_neighbors_mut(
    grid: &[Vec<u8>],
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
    threshold: usize,
) -> bool {
    count_neighbors(|r, c| grid[r][c], row, col, rows, cols) < threshold
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() {
        let grid = vec!["..@".to_string(), ".@.".to_string(), ".@@".to_string()];
        // All 4 objects have fewer than 4 neighbors
        assert_eq!(part1(&grid), 4);
    }

    #[test]
    fn test_large_example() {
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
        // 13 objects have fewer than 4 neighbors
        assert_eq!(part1(&grid), 13);
    }

    #[test]
    fn test_empty_grid() {
        let grid: Vec<String> = vec![];
        assert_eq!(part1(&grid), 0);
    }

    #[test]
    fn test_no_objects() {
        let grid = vec!["...".to_string(), "...".to_string(), "...".to_string()];
        assert_eq!(part1(&grid), 0);
    }

    #[test]
    fn test_all_objects_qualify() {
        let grid = vec!["@.@".to_string(), "...".to_string(), "@.@".to_string()];
        // All 4 objects have 0 or 1 neighbors, all qualify
        assert_eq!(part1(&grid), 4);
    }

    #[test]
    fn test_exactly_four_neighbors() {
        let grid = vec![
            ".@@@.".to_string(),
            ".@@@.".to_string(),
            ".@@@.".to_string(),
            ".....".to_string(),
        ];
        // 9 objects total in a 3x3 block
        // Center (1,2) has 8 neighbors
        // Edge objects have 5 neighbors
        // Corner objects (0,1), (0,3), (2,1), (2,3) have 3 neighbors each
        // Only the 4 corner objects qualify
        assert_eq!(part1(&grid), 4);
    }

    #[test]
    fn test_more_than_four_neighbors() {
        let grid = vec!["@@@".to_string(), "@@@".to_string(), "@@@".to_string()];
        // The center object has 8 neighbors
        // Corner objects have 3 neighbors each (4 corners)
        // Edge objects have 5 neighbors each (4 edges)
        // Only corner objects (3 neighbors) qualify
        assert_eq!(part1(&grid), 4);
    }

    #[test]
    fn test_single_object() {
        let grid = vec!["...".to_string(), ".@.".to_string(), "...".to_string()];
        // Single object with 0 neighbors
        assert_eq!(part1(&grid), 1);
    }

    #[test]
    fn test_corner_objects() {
        let grid = vec![
            "@...@".to_string(),
            ".....".to_string(),
            ".....".to_string(),
            ".....".to_string(),
            "@...@".to_string(),
        ];
        // 4 corner objects, all isolated (0 neighbors each)
        assert_eq!(part1(&grid), 4);
    }

    #[test]
    fn test_edge_vs_center() {
        let grid = vec![".@.".to_string(), "@@@".to_string(), ".@.".to_string()];
        // Center object has 4 neighbors (exactly 4, not counted)
        // All edge objects have 2-3 neighbors (all counted)
        assert_eq!(part1(&grid), 4);
    }

    #[test]
    fn test_dense_grid() {
        let grid = vec![
            "@@@@".to_string(),
            "@@@@".to_string(),
            "@@@@".to_string(),
            "@@@@".to_string(),
        ];
        // 16 objects total
        // Only corner objects (3 neighbors) and edge objects (5 neighbors) don't qualify
        // Actually, only corners have < 4 neighbors
        assert_eq!(part1(&grid), 4);
    }

    // Part 2 tests

    #[test]
    fn test_part2_large_example() {
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
        // According to the problem, 43 objects should be removed
        assert_eq!(part2(&grid), 43);
    }

    #[test]
    fn test_part2_empty_grid() {
        let grid: Vec<String> = vec![];
        assert_eq!(part2(&grid), 0);
    }

    #[test]
    fn test_part2_dense_grid_limited_removal() {
        let grid = vec![
            "@@@@".to_string(),
            "@@@@".to_string(),
            "@@@@".to_string(),
            "@@@@".to_string(),
        ];
        // Initial: 16 objects
        // Corners have 3 neighbors -> removed in first pass (4 objects)
        // After removal, edge objects adjacent to corners now have exactly 4 neighbors
        // All remaining objects have >= 4 neighbors, so no more can be removed
        assert_eq!(part2(&grid), 4);
    }

    #[test]
    fn test_part2_all_removed_one_pass() {
        let grid = vec!["@.@".to_string(), "...".to_string(), "@.@".to_string()];
        // All 4 objects have 0-1 neighbors, all removed in one pass
        assert_eq!(part2(&grid), 4);
    }

    #[test]
    fn test_part2_cascading_removal() {
        let grid = vec![".@.".to_string(), "@@@".to_string(), ".@.".to_string()];
        // Center has 4 neighbors (not removed initially)
        // Edges have 2-3 neighbors (removed in first pass: 4 objects)
        // After first pass, center has 0 neighbors (removed in second pass: 1 object)
        // Total: 5 objects
        assert_eq!(part2(&grid), 5);
    }

    #[test]
    fn test_part2_small_example() {
        let grid = vec!["..@".to_string(), ".@.".to_string(), ".@@".to_string()];
        // All 4 objects have < 4 neighbors, all should be removed
        assert_eq!(part2(&grid), 4);
    }

    #[test]
    fn test_part2_line_removal() {
        let grid = vec!["@@@@@".to_string()];
        // 5 objects in a line
        // End objects have 1 neighbor each (removed first: 2)
        // Then next layer has 1 neighbor each (removed: 2)
        // Then center has 0 neighbors (removed: 1)
        // Total: 5
        assert_eq!(part2(&grid), 5);
    }

    #[test]
    fn test_part2_5x5_grid() {
        let grid = vec![
            "@@@@@".to_string(),
            "@@@@@".to_string(),
            "@@@@@".to_string(),
            "@@@@@".to_string(),
            "@@@@@".to_string(),
        ];
        // 5x5 grid: 25 total objects
        // Pass 1: Remove 4 corners (3 neighbors each)
        // After removal, all remaining objects have >= 4 neighbors
        // Stable core remains with 21 objects
        assert_eq!(part2(&grid), 4);
    }

    #[test]
    fn test_part2_single_object() {
        let grid = vec!["@".to_string()];
        // Single object with 0 neighbors - should be removed
        assert_eq!(part2(&grid), 1);
    }

    #[test]
    fn test_part2_2x2_grid() {
        let grid = vec!["@@".to_string(), "@@".to_string()];
        // All 4 objects have 3 neighbors - all removed in first pass
        assert_eq!(part2(&grid), 4);
    }

    #[test]
    fn test_part2_only_empty_spaces() {
        let grid = vec!["...".to_string(), "...".to_string(), "...".to_string()];
        // No objects to remove
        assert_eq!(part2(&grid), 0);
    }

    #[test]
    fn test_part2_3x3_all_objects() {
        let grid = vec!["@@@".to_string(), "@@@".to_string(), "@@@".to_string()];
        // 9 objects: corners (3 neighbors) + edges (5 neighbors) + center (8 neighbors)
        // Pass 1: Remove 4 corners
        // Pass 2: Remove 4 edges (now have 2-3 neighbors)
        // Pass 3: Remove center (now has 0 neighbors)
        // Total: 9 objects
        assert_eq!(part2(&grid), 9);
    }
}
