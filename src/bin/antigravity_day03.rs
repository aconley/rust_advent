/// Day 3.
fn main() -> std::io::Result<()> {
    let inputs: Vec<Vec<u8>> = rust_advent::read_number_grid("03")?;
    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
    Ok(())
}

/// Function for part 1 (single-threaded).
///
/// Given a grid of numbers, for each row find the largest number that
/// can be formed by selecting two numbers from the row in order.
/// For example, in the row [1, 2, 5, 2, 1] the largest number is 52.
/// This function returns the sum of the largest numbers for each row
/// over all provided rows.
fn part1(grid: &Vec<Vec<u8>>) -> u64 {
    let mut total_sum: u64 = 0;

    for row in grid {
        if row.len() < 2 {
            continue;
        }

        let mut row_best: u64 = 0;
        let mut current_max_right: i32 = -1;

        for &d in row.iter().rev() {
            if current_max_right != -1 {
                let score = (d as u64) * 10 + (current_max_right as u64);
                if score > row_best {
                    row_best = score;
                }
            }
            if (d as i32) > current_max_right {
                current_max_right = d as i32;
            }
        }
        total_sum += row_best;
    }

    total_sum
}

/// Function for part 2.
///
/// Find the largest 12-digit number that can be formed by selecting
/// twelve digits from each row in order, and return their sum.
fn part2(grid: &Vec<Vec<u8>>) -> u64 {
    let mut total_sum: u64 = 0;
    let k = 12;

    for row in grid {
        if row.len() < k {
            continue;
        }

        let mut stack: Vec<u8> = Vec::with_capacity(row.len());
        let mut to_remove = row.len() - k;

        for &d in row {
            while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < d {
                stack.pop();
                to_remove -= 1;
            }
            stack.push(d);
        }

        // Convert the first k digits of the stack to u64
        let mut val: u64 = 0;
        for &d in &stack[..k] {
            val = val * 10 + (d as u64);
        }
        total_sum += val;
    }

    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let grid = vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ];
        assert_eq!(part1(&grid), 357);
    }

    #[test]
    fn test_example_part2() {
        let grid = vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ];
        assert_eq!(part2(&grid), 3121910778619);
    }

    #[test]
    fn test_part2_edge_cases() {
        // Exactly 12
        assert_eq!(
            part2(&vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2]]),
            123456789012
        );

        // Large at start
        assert_eq!(
            part2(&vec![vec![9, 8, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]),
            987000000000
        );

        // Large at end
        assert_eq!(
            part2(&vec![vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]]),
            111111111119
        );

        // Leading zeros
        assert_eq!(
            part2(&vec![vec![0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2]]),
            123456789012
        );
    }
}
