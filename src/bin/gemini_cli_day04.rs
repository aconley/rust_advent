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
const OBJECT: u8 = b'@';
const THRESHOLD: usize = 4;

fn part1(inputs: &[String]) -> usize {
    let rows = inputs.len();
    if rows == 0 {
        return 0;
    }
    let cols = inputs[0].len();
    let grid: Vec<&[u8]> = inputs.iter().map(|s| s.as_bytes()).collect();

    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == OBJECT && has_fewer_neighbors_than(&grid, r, c, THRESHOLD) {
                count += 1;
            }
        }
    }

    count
}

fn part2(inputs: &[String]) -> usize {
    let rows = inputs.len();
    if rows == 0 {
        return 0;
    }
    let cols = inputs[0].len();
    // We need a mutable grid for Part 2
    let mut grid: Vec<Vec<u8>> = inputs.iter().map(|s| s.as_bytes().to_vec()).collect();
    let mut total_removed = 0;
    let mut to_remove = Vec::new();

    loop {
        to_remove.clear();

        // Identify all objects to remove in this generation
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == OBJECT {
                    if has_fewer_neighbors_than(&grid, r, c, THRESHOLD) {
                        to_remove.push((r, c));
                    }
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        total_removed += to_remove.len();

        // Apply removals
        for &(r, c) in &to_remove {
            grid[r][c] = b'.';
        }
    }

    total_removed
}

/// Checks if an object has fewer than `threshold` neighbors.
fn has_fewer_neighbors_than<R: AsRef<[u8]>>(
    grid: &[R],
    r: usize,
    c: usize,
    threshold: usize,
) -> bool {
    let rows = grid.len();
    // Assume rectangular grid, safe because we check inputs.len() > 0 in callers
    // and callers ensure r is valid. However, accessing grid[0] is safe if rows > 0.
    // We can also just use the row length of the current row or 0.
    // But since the grid is rectangular, let's just grab the length of the first row.
    // If rows is 0, the loop below won't run anyway (nr/nc checks).
    if rows == 0 { return true; }
    let cols = grid[0].as_ref().len();

    let mut count = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }

            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                if grid[nr as usize].as_ref()[nc as usize] == OBJECT {
                    count += 1;
                    if count >= threshold {
                        return false;
                    }
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_small_example() {
        let input = vec![
            "..@".to_string(),
            ".@.".to_string(),
            ".@@".to_string(),
        ];
        assert_eq!(part1(&input), 4);
    }

    #[test]
    fn test_part1_large_example() {
        let input = vec![
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
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_empty_input() {
        let input: Vec<String> = vec![];
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_single_object() {
        let input = vec!["@".to_string()];
        assert_eq!(part1(&input), 1);
    }

    #[test]
    fn test_single_row_all_objects() {
        // @@@@@
        // Ends have 1 neighbor, middles have 2. All < 4.
        let input = vec!["@@@@@".to_string()];
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn test_fully_populated_grid() {
        // @@@
        // @@@
        // @@@
        // Center has 8 neighbors (>= 4, fail)
        // Edges (non-corner) have 5 neighbors (>= 4, fail)
        // Corners have 3 neighbors (< 4, pass)
        // Should return 4.
        let input = vec![
            "@@@".to_string(),
            "@@@".to_string(),
            "@@@".to_string(),
        ];
        assert_eq!(part1(&input), 4);
    }

    #[test]
    fn test_exactly_four_neighbors() {
        // .@.
        // @.@
        // .@.
        // Center has exactly 4 neighbors (up, down, left, right). Should NOT count.
        // Outer ones have 1 neighbor each. Should count.
        let input = vec![
            ".@.".to_string(),
            "@.@".to_string(),
            ".@.".to_string(),
        ];
        assert_eq!(part1(&input), 4);
    }

    #[test]
    fn test_no_objects() {
        let input = vec![
            "...".to_string(),
            "...".to_string(),
        ];
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part2_large_example() {
        let input = vec![
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
        assert_eq!(part2(&input), 43);
    }

    #[test]
    fn test_part2_empty() {
        let input: Vec<String> = vec![];
        assert_eq!(part2(&input), 0);
    }

    #[test]
    fn test_part2_stable() {
        // A block of 3x3 @s is not stable (center has 8).
        // Let's make a stable grid:
        // @.@
        // .@.
        // @.@
        // Each @ has 0 neighbors? No, center has 4.
        // Center has 4 neighbors -> stable (remains).
        // Corners have 1 neighbor -> removed.
        // Wait, "remains" if >= 4.
        // Center has 4. It stays.
        // Corners have 1. They go.
        // Next step: Center has 0 neighbors. It goes.
        // Total removed = 5.

        // Actually let's find a STABLE configuration.
        // Need >= 4 neighbors for everyone.
        // A 3x3 block of @s:
        // Center has 8. (Stable)
        // Edges have 5. (Stable)
        // Corners have 3. (Unstable -> Removed)
        // Next step:
        // .@.
        // @@@
        // .@.
        // Center has 4. (Stable)
        // Edges have 3. (Removed)
        // Next step:
        // ...
        // .@.
        // ...
        // Center has 0. (Removed).
        // So 3x3 block eventually clears completely?
        // Step 1: 4 corners removed.
        // Step 2: 4 edges removed.
        // Step 3: 1 center removed.
        // Total 9.
        let input = vec![
            "@@@".to_string(),
            "@@@".to_string(),
            "@@@".to_string(),
        ];
        assert_eq!(part2(&input), 9);
    }

    #[test]
    fn test_part2_single_item() {
        let input = vec!["@".to_string()];
        assert_eq!(part2(&input), 1);
    }
}
 