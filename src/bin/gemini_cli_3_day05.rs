fn main() -> std::io::Result<()> {
    let inputs: rust_advent::RangeData = rust_advent::read_range_data("05")?;
    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
    Ok(())
}

/// Solves the problem by counting how many values are present in at least one range.
///
/// The algorithm first merges overlapping ranges into a set of disjoint, sorted ranges.
/// Then, it checks each value against these disjoint ranges using binary search for efficiency.
/// Complexity: O(R log R + V log R), where R is the number of ranges and V is the number of values.
fn part1(input: &rust_advent::RangeData) -> usize {
    if input.ranges.is_empty() || input.values.is_empty() {
        return 0;
    }

    // 1. Merge overlapping ranges
    let mut sorted_ranges = input.ranges.clone();
    sorted_ranges.sort_unstable_by_key(|r| r.0);

    let mut merged_ranges: Vec<(isize, isize)> = Vec::with_capacity(sorted_ranges.len());
    let mut current_range = sorted_ranges[0];

    for &next_range in &sorted_ranges[1..] {
        if next_range.0 <= current_range.1 {
            // Overlap or touch (inclusive)
            if next_range.1 > current_range.1 {
                current_range.1 = next_range.1;
            }
        } else {
            merged_ranges.push(current_range);
            current_range = next_range;
        }
    }
    merged_ranges.push(current_range);

    // 2. Count values in at least one range
    let mut count = 0;
    for &value in &input.values {
        // Use binary search to find a range that might contain the value.
        // We search for the first range whose start is <= value.
        // binary_search_by returns the index where the value could be inserted.
        let result = merged_ranges.binary_search_by(|range| {
            if value < range.0 {
                std::cmp::Ordering::Greater
            } else if value > range.1 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });

        if result.is_ok() {
            count += 1;
        }
    }

    count
}

/// Solves the problem by summing the lengths of all ranges after merging overlapping ones.
///
/// The algorithm merges overlapping ranges and calculates the sum "on the fly".
/// This avoids allocating a secondary vector for merged ranges.
/// Complexity: O(R log R), where R is the number of ranges.
fn part2(input: &rust_advent::RangeData) -> usize {
    if input.ranges.is_empty() {
        return 0;
    }

    // 1. Sort ranges by start position
    let mut sorted_ranges = input.ranges.clone();
    sorted_ranges.sort_unstable_by_key(|r| r.0);

    let mut total_length = 0;
    let mut current_start = sorted_ranges[0].0;
    let mut current_end = sorted_ranges[0].1;

    for &next_range in &sorted_ranges[1..] {
        if next_range.0 <= current_end {
            // Overlap or touch: extend the current merged range
            if next_range.1 > current_end {
                current_end = next_range.1;
            }
        } else {
            // Gap found: finalize the current merged range and start a new one
            total_length += (current_end - current_start + 1) as usize;
            current_start = next_range.0;
            current_end = next_range.1;
        }
    }
    // Add the final range
    total_length += (current_end - current_start + 1) as usize;

    total_length
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_advent::RangeData;

    #[test]
    fn test_example_part1() {
        let input = RangeData {
            ranges: vec![(3, 5), (10, 14), (16, 20), (12, 18)],
            values: vec![1, 5, 8, 11, 17, 32],
        };
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_example_part2() {
        let input = RangeData {
            ranges: vec![(3, 5), (10, 14), (16, 20), (12, 18)],
            values: vec![],
        };
        assert_eq!(part2(&input), 14);
    }

    #[test]
    fn test_no_ranges() {
        let input = RangeData {
            ranges: vec![],
            values: vec![1, 2, 3],
        };
        assert_eq!(part1(&input), 0);
        assert_eq!(part2(&input), 0);
    }

    #[test]
    fn test_no_values() {
        let input = RangeData {
            ranges: vec![(1, 10)],
            values: vec![],
        };
        assert_eq!(part1(&input), 0);
        assert_eq!(part2(&input), 10);
    }

    #[test]
    fn test_overlapping_identical_ranges() {
        let input = RangeData {
            ranges: vec![(1, 5), (1, 5), (1, 5)],
            values: vec![0, 1, 3, 5, 6],
        };
        assert_eq!(part1(&input), 3);
        assert_eq!(part2(&input), 5);
    }

    #[test]
    fn test_adjacent_ranges() {
        let input = RangeData {
            ranges: vec![(3, 5), (6, 10)],
            values: vec![5, 6],
        };
        assert_eq!(part1(&input), 2);
        assert_eq!(part2(&input), 8);
    }
}
