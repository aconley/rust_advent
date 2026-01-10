fn main() -> std::io::Result<()> {
    let inputs: rust_advent::RangeData = rust_advent::read_range_data("05")?;
    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
    Ok(())
}

/// Merges overlapping and adjacent ranges.
///
/// Takes a slice of ranges, sorts them, and returns a new Vec with
/// ranges merged. E.g., `[(1, 5), (3, 7), (9, 10)]` becomes `[(1, 7), (9, 10)]`.
fn merge_ranges(ranges: &[(isize, isize)]) -> Vec<(isize, isize)> {
    if ranges.is_empty() {
        return vec![];
    }

    let mut ranges = ranges.to_vec();
    ranges.sort_unstable_by_key(|r| r.0);

    let mut merged = Vec::with_capacity(ranges.len());
    merged.push(ranges[0]);

    for &(start, end) in &ranges[1..] {
        let last = merged.last_mut().unwrap();
        // If the new range overlaps or is adjacent to the last one, merge them.
        if start <= last.1 + 1 {
            last.1 = std::cmp::max(last.1, end);
        } else {
            merged.push((start, end));
        }
    }
    merged
}

/// Part 1
///
/// Given RangeData input, determine how many input.values are in at least
/// one input.range, where each range is an inclusive interval [start, end].
/// Ranges may overlap, but a value that is in multiple ranges should only
/// count once.
fn part1(input: &rust_advent::RangeData) -> usize {
    let merged_ranges = merge_ranges(&input.ranges);

    input
        .values
        .iter()
        .filter(|&&value| {
            // Find the insertion point for the value in the sorted merged ranges.
            match merged_ranges.binary_search_by_key(&value, |r| r.0) {
                // The value is the start of a range, so it's in.
                Ok(_) => true,
                // The value is not a start. Check if it's within the previous range.
                Err(i) => {
                    i > 0 && value >= merged_ranges[i - 1].0 && value <= merged_ranges[i - 1].1
                }
            }
        })
        .count()
}

fn part2(input: &rust_advent::RangeData) -> usize {
    if input.ranges.is_empty() {
        return 0;
    }

    let mut ranges = input.ranges.clone();
    ranges.sort_unstable_by_key(|r| r.0);

    let mut total_length = 0;
    let (mut current_start, mut current_end) = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= current_end + 1 {
            current_end = std::cmp::max(current_end, end);
        } else {
            total_length += (current_end - current_start + 1) as usize;
            current_start = start;
            current_end = end;
        }
    }
    // Add the last merged range
    total_length += (current_end - current_start + 1) as usize;

    total_length
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_advent::RangeData;

    #[test]
    fn test_part1_example() {
        let input = RangeData {
            ranges: vec![(3, 5), (10, 14), (16, 20), (12, 18)],
            values: vec![1, 5, 8, 11, 17, 32],
        };
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part1_no_values() {
        let input = RangeData {
            ranges: vec![(1, 10)],
            values: vec![],
        };
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part1_no_ranges() {
        let input = RangeData {
            ranges: vec![],
            values: vec![1, 2, 3],
        };
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part1_all_values_in() {
        let input = RangeData {
            ranges: vec![(0, 100)],
            values: vec![10, 20, 30],
        };
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part1_no_values_in() {
        let input = RangeData {
            ranges: vec![(10, 20)],
            values: vec![1, 5, 25],
        };
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part1_overlapping_ranges() {
        let input = RangeData {
            ranges: vec![(10, 20), (15, 25)],
            values: vec![12, 22, 18, 28],
        };
        assert_eq!(part1(&input), 3); // 12, 22, 18 are in. 28 is out.
    }

    #[test]
    fn test_part1_value_at_boundary() {
        let input = RangeData {
            ranges: vec![(10, 20)],
            values: vec![10, 20, 9, 21],
        };
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part1_disjoint_ranges() {
        let input = RangeData {
            ranges: vec![(1, 5), (10, 15)],
            values: vec![3, 12, 7],
        };
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part2_example() {
        let input = RangeData {
            ranges: vec![(3, 5), (10, 14), (16, 20), (12, 18)],
            values: vec![], // Values are ignored in part 2
        };
        assert_eq!(part2(&input), 14);
    }

    #[test]
    fn test_part2_no_ranges() {
        let input = RangeData {
            ranges: vec![],
            values: vec![1, 2, 3],
        };
        assert_eq!(part2(&input), 0);
    }

    #[test]
    fn test_part2_single_range() {
        let input = RangeData {
            ranges: vec![(10, 20)],
            values: vec![],
        };
        assert_eq!(part2(&input), 11);
    }

    #[test]
    fn test_part2_overlapping_ranges() {
        let input = RangeData {
            ranges: vec![(10, 20), (15, 25)],
            values: vec![],
        };
        assert_eq!(part2(&input), 16); // Merged: (10, 25), length 16
    }

    #[test]
    fn test_part2_contained_range() {
        let input = RangeData {
            ranges: vec![(10, 30), (15, 25)],
            values: vec![],
        };
        assert_eq!(part2(&input), 21); // Merged: (10, 30), length 21
    }

    #[test]
    fn test_part2_adjacent_ranges() {
        let input = RangeData {
            ranges: vec![(1, 5), (6, 10)],
            values: vec![],
        };
        assert_eq!(part2(&input), 10);
    }
}
