fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_range_data("05")?;
    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
    Ok(())
}

/// Function for part 1.
/// Counts the number of values that are present in at least one range.
/// Ranges may overlap, but each value is counted only once regardless of how many ranges it appears in.
///
/// Efficiency: Merges overlapping ranges first to reduce the number of range checks per value.
/// This is particularly beneficial when many ranges overlap.
fn part1(input: &rust_advent::RangeData) -> usize {
    if input.ranges.is_empty() || input.values.is_empty() {
        return 0;
    }

    // Merge overlapping ranges to reduce the number of checks needed
    let merged_ranges = merge_overlapping_ranges(&input.ranges);

    input
        .values
        .iter()
        .filter(|&&value| {
            merged_ranges
                .iter()
                .any(|&(start, end)| value >= start && value <= end)
        })
        .count()
}

/// Function for part 2.
/// Calculates the sum of lengths of all ranges after merging overlapping and adjacent ranges.
/// Each range is inclusive, so the length of range [a, b] is b - a + 1.
///
/// Efficiency: Merges and sums in a single pass without allocating a vector for merged ranges,
/// saving O(m) space and eliminating a second iteration over the merged ranges.
fn part2(input: &rust_advent::RangeData) -> usize {
    if input.ranges.is_empty() {
        return 0;
    }

    // Sort ranges by start position
    let mut sorted_ranges = input.ranges.to_vec();
    sorted_ranges.sort_unstable_by_key(|r| r.0);

    let mut sum = 0;
    let mut current = sorted_ranges[0];

    for &(start, end) in &sorted_ranges[1..] {
        if start <= current.1 {
            // Overlap: merge by extending the end if necessary
            current.1 = current.1.max(end);
        } else {
            // No overlap: add current range length to sum and start a new range
            sum += (current.1 - current.0 + 1) as usize;
            current = (start, end);
        }
    }
    // Add the final range length
    sum += (current.1 - current.0 + 1) as usize;

    sum
}

/// Merges overlapping ranges into a sorted vector of disjoint ranges.
/// Ranges are inclusive and are merged if they overlap (including boundary overlap where end == next start).
fn merge_overlapping_ranges(ranges: &[(isize, isize)]) -> Vec<(isize, isize)> {
    if ranges.is_empty() {
        return Vec::new();
    }

    let mut sorted: Vec<(isize, isize)> = ranges.to_vec();
    sorted.sort_unstable_by_key(|r| r.0);

    let mut merged = Vec::with_capacity(sorted.len());
    let mut current = sorted[0];

    for &(start, end) in &sorted[1..] {
        if start <= current.1 {
            // Overlap or touch: merge into current range
            current.1 = current.1.max(end);
        } else {
            // Gap: save current and start a new range
            merged.push(current);
            current = (start, end);
        }
    }
    merged.push(current);
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example_from_prompt() {
        // Example: ranges 3-5, 10-14, 16-20, 12-18
        // Values: 1, 5, 8, 11, 17, 32
        // Expected: 5, 11, and 17 are in ranges (count = 3)
        let input = rust_advent::RangeData {
            ranges: vec![(3, 5), (10, 14), (16, 20), (12, 18)],
            values: vec![1, 5, 8, 11, 17, 32],
        };
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part1_value_at_range_boundaries() {
        let input = rust_advent::RangeData {
            ranges: vec![(5, 10)],
            values: vec![4, 5, 10, 11],
        };
        // 5 and 10 are at boundaries (inclusive), so count = 2
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part1_overlapping_ranges_same_value() {
        // Value appears in multiple overlapping ranges, should count once
        let input = rust_advent::RangeData {
            ranges: vec![(1, 10), (5, 15), (8, 20)],
            values: vec![9],
        };
        // 9 is in all three ranges, but counts only once
        assert_eq!(part1(&input), 1);
    }

    #[test]
    fn test_part1_no_values_in_ranges() {
        let input = rust_advent::RangeData {
            ranges: vec![(10, 20), (30, 40)],
            values: vec![1, 5, 25, 50],
        };
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part1_all_values_in_ranges() {
        let input = rust_advent::RangeData {
            ranges: vec![(1, 100)],
            values: vec![10, 20, 30, 40, 50],
        };
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn test_part1_single_value_ranges() {
        let input = rust_advent::RangeData {
            ranges: vec![(5, 5), (10, 10)],
            values: vec![5, 10, 15],
        };
        // 5 and 10 match single-value ranges
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part1_empty_values() {
        let input = rust_advent::RangeData {
            ranges: vec![(1, 10)],
            values: vec![],
        };
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part1_empty_ranges() {
        let input = rust_advent::RangeData {
            ranges: vec![],
            values: vec![1, 2, 3],
        };
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part1_negative_values() {
        let input = rust_advent::RangeData {
            ranges: vec![(-10, -5), (0, 5)],
            values: vec![-7, -3, 0, 3, 10],
        };
        // -7 is in first range, 0 and 3 are in second range
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part1_large_numbers() {
        let input = rust_advent::RangeData {
            ranges: vec![(1000, 2000), (5000, 6000)],
            values: vec![1500, 2500, 5500, 7000],
        };
        // 1500 and 5500 are in ranges
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part2_example_from_prompt() {
        // Example: ranges 3-5, 10-14, 16-20, 12-18
        // After merging: 3-5 (length 3) and 10-20 (length 11)
        // Total: 3 + 11 = 14
        let input = rust_advent::RangeData {
            ranges: vec![(3, 5), (10, 14), (16, 20), (12, 18)],
            values: vec![], // Ignored in part2
        };
        assert_eq!(part2(&input), 14);
    }

    #[test]
    fn test_part2_no_overlap() {
        // Three separate ranges: [1,3], [5,7], [10,12]
        // Lengths: 3, 3, 3 = 9
        let input = rust_advent::RangeData {
            ranges: vec![(1, 3), (5, 7), (10, 12)],
            values: vec![],
        };
        assert_eq!(part2(&input), 9);
    }

    #[test]
    fn test_part2_complete_overlap() {
        // All ranges merge into one: [1,10]
        // Length: 10
        let input = rust_advent::RangeData {
            ranges: vec![(1, 10), (3, 5), (2, 8)],
            values: vec![],
        };
        assert_eq!(part2(&input), 10);
    }

    #[test]
    fn test_part2_single_range() {
        // Single range [5,10]
        // Length: 6
        let input = rust_advent::RangeData {
            ranges: vec![(5, 10)],
            values: vec![],
        };
        assert_eq!(part2(&input), 6);
    }

    #[test]
    fn test_part2_empty_ranges() {
        let input = rust_advent::RangeData {
            ranges: vec![],
            values: vec![],
        };
        assert_eq!(part2(&input), 0);
    }

    #[test]
    fn test_part2_boundary_overlap() {
        // Ranges that share an endpoint should merge: [1,5] and [5,10] → [1,10]
        // Length: 10
        let input = rust_advent::RangeData {
            ranges: vec![(1, 5), (5, 10)],
            values: vec![],
        };
        assert_eq!(part2(&input), 10);
    }

    #[test]
    fn test_part2_single_element_ranges() {
        // Three single-element ranges: [1,1], [3,3], [5,5]
        // Lengths: 1, 1, 1 = 3
        let input = rust_advent::RangeData {
            ranges: vec![(1, 1), (3, 3), (5, 5)],
            values: vec![],
        };
        assert_eq!(part2(&input), 3);
    }

    #[test]
    fn test_part2_negative_ranges() {
        // Ranges: [-10,-5], [0,5]
        // Lengths: 6, 6 = 12
        let input = rust_advent::RangeData {
            ranges: vec![(-10, -5), (0, 5)],
            values: vec![],
        };
        assert_eq!(part2(&input), 12);
    }

    #[test]
    fn test_part2_unsorted_ranges() {
        // Unsorted ranges should still work correctly
        let input = rust_advent::RangeData {
            ranges: vec![(20, 25), (1, 5), (3, 8), (10, 15)],
            values: vec![],
        };
        // After sorting and merging: [1,8], [10,15], [20,25]
        // Lengths: 8, 6, 6 = 20
        assert_eq!(part2(&input), 20);
    }

    #[test]
    fn test_part2_many_overlapping() {
        // Many overlapping ranges should merge efficiently
        let input = rust_advent::RangeData {
            ranges: vec![
                (1, 5),
                (3, 7),
                (5, 10),
                (8, 12),
                (11, 15),
                (14, 18),
                (17, 20),
            ],
            values: vec![],
        };
        // Should merge to [1,20], length 20
        assert_eq!(part2(&input), 20);
    }

    #[test]
    fn test_merge_overlapping_ranges() {
        // Test that overlapping ranges are merged correctly
        let ranges = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
        let merged = merge_overlapping_ranges(&ranges);
        // After sorting and merging: (3,5), (10,20) - since 12-18 overlaps with both 10-14 and 16-20
        assert_eq!(merged, vec![(3, 5), (10, 20)]);
    }

    #[test]
    fn test_merge_boundary_overlap() {
        // Ranges that share an endpoint (overlap at boundary) should be merged
        let ranges = vec![(1, 5), (5, 10)];
        let merged = merge_overlapping_ranges(&ranges);
        assert_eq!(merged, vec![(1, 10)]);
    }

    #[test]
    fn test_merge_unsorted_ranges() {
        // Merging should work even if ranges are unsorted
        let ranges = vec![(20, 30), (5, 15), (10, 25)];
        let merged = merge_overlapping_ranges(&ranges);
        // After sorting and merging: (5, 30)
        assert_eq!(merged, vec![(5, 30)]);
    }
}
