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

    let grid: Vec<&[u8]> = inputs.iter().map(|s| s.as_bytes()).collect();
    let rows = grid.len();
    let mut total_count = 0;

    for r in 0..rows {
        let current_row = grid[r];
        for c in 0..current_row.len() {
            if current_row[c] == b'@' {
                if has_fewer_than_n_neighbors(&grid, r, c, 4) {
                    total_count += 1;
                }
            }
        }
    }
    total_count
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

    let rows = inputs.len();
    let cols = inputs[0].len();
    let mut grid: Vec<Vec<u8>> = inputs.iter().map(|s| s.as_bytes().to_vec()).collect();
    let mut neighbor_counts = vec![vec![0u8; cols]; rows];
    let mut queued = vec![vec![false; cols]; rows];
    let mut queue = std::collections::VecDeque::new();

    // Initialize neighbor counts and find initial removable objects
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == b'@' {
                let mut count = 0;
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        let nr = r as isize + dr;
                        let nc = c as isize + dc;
                        if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                            if grid[nr as usize][nc as usize] == b'@' {
                                count += 1;
                            }
                        }
                    }
                }
                neighbor_counts[r][c] = count;
                if count < 4 {
                    queue.push_back((r, c));
                    queued[r][c] = true;
                }
            }
        }
    }

    let mut removed_count = 0;
    while let Some((r, c)) = queue.pop_front() {
        if grid[r][c] == b'.' {
            continue;
        }

        grid[r][c] = b'.';
        removed_count += 1;

        // Update neighbors
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if grid[nr][nc] == b'@' {
                        neighbor_counts[nr][nc] -= 1;
                        if neighbor_counts[nr][nc] < 4 && !queued[nr][nc] {
                            queue.push_back((nr, nc));
                            queued[nr][nc] = true;
                        }
                    }
                }
            }
        }
    }

    removed_count
}

/// Returns true if the position (r, c) has strictly fewer than `n` adjacent '@' objects.
fn has_fewer_than_n_neighbors(grid: &[&[u8]], r: usize, c: usize, n: usize) -> bool {
    let mut count = 0;
    let rows = grid.len();

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }

            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if let Some(row_idx) = nr.try_into().ok().filter(|&idx: &usize| idx < rows) {
                let neighbor_row = grid[row_idx];
                if let Some(col_idx) = nc
                    .try_into()
                    .ok()
                    .filter(|&idx: &usize| idx < neighbor_row.len())
                {
                    if neighbor_row[col_idx] == b'@' {
                        count += 1;
                        if count >= n {
                            return false;
                        }
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
        assert_eq!(part1(&[]), 0);
    }

    #[test]
    fn test_no_objects() {
        let grid = vec!["...".to_string(), "...".to_string()];
        assert_eq!(part1(&grid), 0);
    }

    #[test]
    fn test_single_object() {
        let grid = vec!["...".to_string(), ".@.".to_string(), "...".to_string()];
        assert_eq!(part1(&grid), 1);
    }

    #[test]
    fn test_full_grid_3x3() {
        let grid = vec!["@@@".to_string(), "@@@".to_string(), "@@@".to_string()];
        // Center has 8 neighbors, Corners have 3, Edges have 5.
        // Only corners (4) have < 4 neighbors.
        assert_eq!(part1(&grid), 4);
    }

    #[test]
    fn test_line_horizontal() {
        let grid = vec!["@@@".to_string()];
        // Ends have 1 neighbor, middle has 2. All have < 4.
        assert_eq!(part1(&grid), 3);
    }

    #[test]
    fn test_line_vertical() {
        let grid = vec!["@".to_string(), "@".to_string(), "@".to_string()];
        // Ends have 1 neighbor, middle has 2. All have < 4.
        assert_eq!(part1(&grid), 3);
    }

    #[test]
    fn test_part2_example() {
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
    fn test_part2_single_object() {
        let grid = vec!["@".to_string()];
        assert_eq!(part2(&grid), 1);
    }

    #[test]
    fn test_part2_3x3_dense() {
        let grid = vec!["@@@".to_string(), "@@@".to_string(), "@@@".to_string()];
        // All objects should eventually be removed.
        assert_eq!(part2(&grid), 9);
    }

    #[test]
    fn test_part2_empty() {
        assert_eq!(part2(&[]), 0);
    }

    #[test]
    fn test_part2_no_objects() {
        let grid = vec!["...".to_string(), "...".to_string()];
        assert_eq!(part2(&grid), 0);
    }

    #[test]
    fn test_part2_sparse() {
        let grid = vec!["@.@".to_string()];
        assert_eq!(part2(&grid), 2);
    }

    #[test]
    fn test_part2_multi_step() {
        // Here, the '@' at (0,2) and (2,0) have < 4 neighbors initially.
        // Once they are removed, others might become removable.
        let grid = vec!["@@@".to_string(), "@@@".to_string(), "@@.".to_string()];
        // (0,0): 3 (corner) -> removable
        // (0,1): 5 (edge)
        // (0,2): 2 (corner) -> removable
        // (1,0): 5 (edge)
        // (1,1): 7 (center)
        // (1,2): 4 (edge)
        // (2,0): 2 (corner) -> removable
        // (2,1): 4 (edge)

        // Step 1: remove (0,0), (0,2), (2,0). (3 total)
        // Grid now:
        // .@.
        // @@@
        // .@.
        // (0,1) now has 2 neighbors -> removable
        // (1,0) now has 2 neighbors -> removable
        // (1,1) now has 4 neighbors (still safe? wait, (0,1), (1,0), (2,1), (1,2) are its neighbors)
        // (1,2) now has 2 neighbors -> removable
        // (2,1) now has 2 neighbors -> removable

        // Step 2: remove (0,1), (1,0), (1,2), (2,1). (4 total, sum 7)
        // Grid now:
        // ...
        // .@.
        // ...
        // (1,1) now has 0 neighbors -> removable

        // Step 3: remove (1,1). (1 total, sum 8)
        assert_eq!(part2(&grid), 8);
    }
}
