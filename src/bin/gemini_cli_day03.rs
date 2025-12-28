/// Day 3.
use rayon::prelude::*;

fn main() -> std::io::Result<()> {
    let inputs: Vec<Vec<u8>> = rust_advent::read_number_grid("03")?;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "part1" => println!("Part 1: {}", part1(&inputs)),
            "part2" => println!("Part 2: {}", part2(&inputs)),
            _ => {
                println!("Part 1: {}", part1(&inputs));
                println!("Part 2: {}", part2(&inputs));
            }
        }
    } else {
        println!("Part 1: {}", part1(&inputs));
        println!("Part 2: {}", part2(&inputs));
    }
    Ok(())
}

/// Function for part 1.
fn part1(grid: &Vec<Vec<u8>>) -> u64 {
    grid.par_iter()
        .map(|row| find_largest_number(row, 2))
        .sum()
}

/// Function for part 2.
fn part2(grid: &Vec<Vec<u8>>) -> u64 {
    grid.par_iter()
        .map(|row| find_largest_number(row, 12))
        .sum()
}

/// generalizes finding the largest number formed by selecting k digits
fn find_largest_number(row: &[u8], k: usize) -> u64 {
    let len = row.len();
    if len < k {
        return 0;
    }

    let mut current_slice = row;
    let mut result: u64 = 0;

    for needed in (1..=k).rev() {
        // search_limit: how many elements from current_slice we can consider
        // while still leaving (needed - 1) elements for later.
        let search_limit = current_slice.len() - (needed - 1);
        
        if search_limit == 1 {
             result = result * 10 + (current_slice[0] as u64);
             current_slice = &current_slice[1..];
             continue;
        }

        let (digit, idx) = find_max_and_first_index(&current_slice[0..search_limit]);
        result = result * 10 + (digit as u64);
        current_slice = &current_slice[idx + 1..];
    }
    result
}

/// Finds the maximum value in a slice.
fn find_max_u8(slice: &[u8]) -> u8 {
    let mut max_val = 0;
    let chunks = slice.chunks_exact(32);
    let remainder = chunks.remainder();

    for chunk in chunks {
        let chunk_max = *chunk.iter().max().unwrap_or(&0);
        if chunk_max > max_val {
            max_val = chunk_max;
            if max_val == 9 { return 9; }
        }
    }

    for &val in remainder {
        if val > max_val {
            max_val = val;
        }
    }
    max_val
}

/// Finds the maximum value and its first index in a slice.
fn find_max_and_first_index(slice: &[u8]) -> (u8, usize) {
    let max_val = find_max_u8(slice);
    let first_idx = slice.iter().position(|&x| x == max_val).unwrap_or(0);
    (max_val, first_idx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let grid = vec![vec![1, 5, 3, 7]];
        assert_eq!(part1(&grid), 57);
    }

    #[test]
    fn test_part1_large_example() {
        let grid = vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ];
        assert_eq!(part1(&grid), 357);
    }

    #[test]
    fn test_part2_example() {
        let grid = vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ];
        assert_eq!(part2(&grid), 3121910778619);
    }

    #[test]
    fn test_part2_exact_and_short() {
        let grid = vec![
            // Exactly 12 digits
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2],
            // Fewer than 12 digits
            vec![9; 11],
        ];
        assert_eq!(part2(&grid), 123456789012 + 0);
    }

    #[test]
    fn test_part2_greedy_selection() {
        // row: [9, 1, 9, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1] (len 13)
        // k=12. 
        // 1st digit: search limit 13-11=2 -> [9, 1]. Max 9 at idx 0.
        // 2nd digit: search limit 12-10=2 -> [1, 9]. Max 9 at idx 1.
        // 3rd digit: search limit 11-9=2 -> [2, 3]. Max 3 at idx 1.
        // etc.
        let grid = vec![
            vec![9, 1, 9, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1],
        ];
        assert_eq!(part2(&grid), 992345678901);
    }
}
