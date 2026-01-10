fn main() -> std::io::Result<()> {
    let inputs: rust_advent::RangeData = rust_advent::read_range_data("05")?;
    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
    Ok(())
}

/// Part 1
///
/// Given RangeData input, determine how many input.values are in at least
/// one input.range, where each range is an inclusive interval [start, end].
/// Ranges may overlap, but a value that is in multiple ranges should only
/// count once.
fn part1(input: &rust_advent::RangeData) -> usize {
    // Merge overlapping ranges for efficiency
    let merged_ranges = merge_ranges(&input.ranges);

    // Count values in merged ranges
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

/// Merges overlapping ranges into a minimal set of non-overlapping ranges.
///
/// Time complexity: O(m log m) where m is the number of ranges
/// Space complexity: O(m)
fn merge_ranges(ranges: &[(isize, isize)]) -> Vec<(isize, isize)> {
    if ranges.is_empty() {
        return Vec::new();
    }

    // Sort ranges by start position
    let mut sorted_ranges = ranges.to_vec();
    sorted_ranges.sort_unstable_by_key(|&(start, _)| start);

    let mut merged = Vec::new();
    let mut current = sorted_ranges[0];

    for &(start, end) in &sorted_ranges[1..] {
        // Check if ranges overlap or are adjacent
        // Ranges [a, b] and [c, d] overlap if c <= b + 1
        if start <= current.1 + 1 {
            // Merge by extending the end if necessary
            current.1 = current.1.max(end);
        } else {
            // No overlap, save current and start a new range
            merged.push(current);
            current = (start, end);
        }
    }
    merged.push(current);

    merged
}

/// Part 2
///
/// Calculate the sum of lengths of all merged ranges.
/// Each range is inclusive, so the length of range [a, b] is b - a + 1.
fn part2(input: &rust_advent::RangeData) -> usize {
    let merged_ranges = merge_ranges(&input.ranges);

    merged_ranges
        .iter()
        .map(|&(start, end)| (end - start + 1) as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_advent::RangeData;

    #[test]
    fn test_part1_example() {
        // Example from problem statement:
        // Ranges: 3-5, 10-14, 16-20, 12-18
        // Values: 1, 5, 8, 11, 17, 32
        // Expected: 3 (values 5, 11, and 17 are in at least one range)
        let input = RangeData {
            ranges: vec![(3, 5), (10, 14), (16, 20), (12, 18)],
            values: vec![1, 5, 8, 11, 17, 32],
        };
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part1_no_values_in_ranges() {
        let input = RangeData {
            ranges: vec![(10, 20), (30, 40)],
            values: vec![1, 2, 3, 50, 60],
        };
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part1_all_values_in_ranges() {
        let input = RangeData {
            ranges: vec![(1, 10)],
            values: vec![1, 5, 10],
        };
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part1_boundary_values() {
        let input = RangeData {
            ranges: vec![(5, 10), (15, 20)],
            values: vec![4, 5, 10, 11, 15, 20, 21],
        };
        // Values 5, 10, 15, 20 are in ranges
        assert_eq!(part1(&input), 4);
    }

    #[test]
    fn test_part1_overlapping_ranges() {
        // Value 15 is in both ranges, but should only count once
        let input = RangeData {
            ranges: vec![(10, 20), (15, 25)],
            values: vec![5, 15, 30],
        };
        assert_eq!(part1(&input), 1);
    }

    #[test]
    fn test_part1_empty_values() {
        let input = RangeData {
            ranges: vec![(1, 10)],
            values: vec![],
        };
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part1_empty_ranges() {
        let input = RangeData {
            ranges: vec![],
            values: vec![1, 2, 3],
        };
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part1_negative_values() {
        let input = RangeData {
            ranges: vec![(-10, -5), (0, 5)],
            values: vec![-15, -7, -5, 0, 3, 10],
        };
        // Values -7, -5, 0, 3 are in ranges
        assert_eq!(part1(&input), 4);
    }

    #[test]
    fn test_merge_ranges_no_overlap() {
        let ranges = vec![(1, 3), (5, 7), (10, 12)];
        let merged = merge_ranges(&ranges);
        assert_eq!(merged, vec![(1, 3), (5, 7), (10, 12)]);
    }

    #[test]
    fn test_merge_ranges_complete_overlap() {
        let ranges = vec![(1, 10), (3, 5), (2, 8)];
        let merged = merge_ranges(&ranges);
        assert_eq!(merged, vec![(1, 10)]);
    }

    #[test]
    fn test_merge_ranges_partial_overlap() {
        // Example from problem: 3-5, 10-14, 16-20, 12-18
        let ranges = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
        let merged = merge_ranges(&ranges);
        // Should merge 10-14 and 12-18 into 10-18, and 16-20 into that
        assert_eq!(merged, vec![(3, 5), (10, 20)]);
    }

    #[test]
    fn test_merge_ranges_adjacent() {
        // Adjacent ranges [1,5] and [6,10] should merge to [1,10]
        let ranges = vec![(1, 5), (6, 10)];
        let merged = merge_ranges(&ranges);
        assert_eq!(merged, vec![(1, 10)]);
    }

    #[test]
    fn test_merge_ranges_empty() {
        let ranges = vec![];
        let merged = merge_ranges(&ranges);
        assert_eq!(merged, vec![]);
    }

    #[test]
    fn test_merge_ranges_single() {
        let ranges = vec![(5, 10)];
        let merged = merge_ranges(&ranges);
        assert_eq!(merged, vec![(5, 10)]);
    }

    #[test]
    fn test_merge_ranges_unsorted() {
        let ranges = vec![(10, 15), (1, 5), (3, 8), (20, 25)];
        let merged = merge_ranges(&ranges);
        // Should sort first then merge: [1,5] and [3,8] → [1,8]
        assert_eq!(merged, vec![(1, 8), (10, 15), (20, 25)]);
    }

    #[test]
    fn test_part2_example() {
        // Example from problem statement:
        // Ranges: 3-5, 10-14, 16-20, 12-18
        // After merging: 3-5 (length 3) and 10-20 (length 11)
        // Total: 3 + 11 = 14
        let input = RangeData {
            ranges: vec![(3, 5), (10, 14), (16, 20), (12, 18)],
            values: vec![], // Ignored in part2
        };
        assert_eq!(part2(&input), 14);
    }

    #[test]
    fn test_part2_no_overlap() {
        // Three separate ranges: [1,3], [5,7], [10,12]
        // Lengths: 3, 3, 3 = 9
        let input = RangeData {
            ranges: vec![(1, 3), (5, 7), (10, 12)],
            values: vec![],
        };
        assert_eq!(part2(&input), 9);
    }

    #[test]
    fn test_part2_complete_overlap() {
        // All ranges merge into one: [1,10]
        // Length: 10
        let input = RangeData {
            ranges: vec![(1, 10), (3, 5), (2, 8)],
            values: vec![],
        };
        assert_eq!(part2(&input), 10);
    }

    #[test]
    fn test_part2_single_range() {
        // Single range [5,10]
        // Length: 6
        let input = RangeData {
            ranges: vec![(5, 10)],
            values: vec![],
        };
        assert_eq!(part2(&input), 6);
    }

    #[test]
    fn test_part2_empty_ranges() {
        let input = RangeData {
            ranges: vec![],
            values: vec![],
        };
        assert_eq!(part2(&input), 0);
    }

    #[test]
    fn test_part2_adjacent_ranges() {
        // Adjacent ranges [1,5] and [6,10] merge to [1,10]
        // Length: 10
        let input = RangeData {
            ranges: vec![(1, 5), (6, 10)],
            values: vec![],
        };
        assert_eq!(part2(&input), 10);
    }

    #[test]
    fn test_part2_single_element_ranges() {
        // Three single-element ranges: [1,1], [3,3], [5,5]
        // Lengths: 1, 1, 1 = 3
        let input = RangeData {
            ranges: vec![(1, 1), (3, 3), (5, 5)],
            values: vec![],
        };
        assert_eq!(part2(&input), 3);
    }

    #[test]
    fn test_part2_negative_ranges() {
        // Ranges: [-10,-5], [0,5]
        // Lengths: 6, 6 = 12
        let input = RangeData {
            ranges: vec![(-10, -5), (0, 5)],
            values: vec![],
        };
        assert_eq!(part2(&input), 12);
    }
}
