fn main() -> std::io::Result<()> {
    let inputs: rust_advent::RangeData = rust_advent::read_range_data("05")?;
    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
    Ok(())
}

/// Helper to sort and merge overlapping/adjacent ranges.
fn merge_ranges(ranges: &[(isize, isize)]) -> Vec<(isize, isize)> {
    if ranges.is_empty() {
        return Vec::new();
    }

    let mut sorted = ranges.to_vec();
    sorted.sort_unstable_by_key(|r| r.0);

    let mut merged: Vec<(isize, isize)> = Vec::with_capacity(sorted.len());
    let mut current = sorted[0];

    for &next in sorted.iter().skip(1) {
        if next.0 <= current.1 {
            // Overlapping or adjacent ranges
            current.1 = current.1.max(next.1);
        } else {
            // Disjoint range
            merged.push(current);
            current = next;
        }
    }
    merged.push(current);
    merged
}

/// Part 1: Count the number of values that are present in any range.
/// Ranges may overlap, but each value is counted once per occurrence in input.values.
fn part1(input: &rust_advent::RangeData) -> usize {
    if input.ranges.is_empty() || input.values.is_empty() {
        return 0;
    }

    let merged = merge_ranges(&input.ranges);

    // Count values in ranges. O(V log R_merged)
    input
        .values
        .iter()
        .filter(|&&v| {
            let idx = merged.partition_point(|r| r.1 < v);
            merged.get(idx).map_or(false, |r| v >= r.0)
        })
        .count()
}

/// Part 2: Sum the lengths of all intervals after merging overlapping ranges.
fn part2(input: &rust_advent::RangeData) -> usize {
    if input.ranges.is_empty() {
        return 0;
    }

    let mut sorted = input.ranges.clone();
    sorted.sort_unstable_by_key(|r| r.0);

    let mut sum = 0;
    let mut current = sorted[0];

    for &next in sorted.iter().skip(1) {
        if next.0 <= current.1 {
            // Overlapping or adjacent ranges
            current.1 = current.1.max(next.1);
        } else {
            // Disjoint range
            sum += (current.1 - current.0 + 1) as usize;
            current = next;
        }
    }
    sum += (current.1 - current.0 + 1) as usize;
    sum
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
    fn test_part2_example() {
        let input = RangeData {
            ranges: vec![(3, 5), (10, 14), (16, 20), (12, 18)],
            values: vec![],
        };
        // Merged ranges: (3, 5) and (10, 20)
        // Lengths: (5-3+1) = 3, (20-10+1) = 11. Sum = 14.
        assert_eq!(part2(&input), 14);
    }

    #[test]
    fn test_empty_ranges() {
        let input = RangeData {
            ranges: vec![],
            values: vec![1, 2, 3],
        };
        assert_eq!(part1(&input), 0);
        assert_eq!(part2(&input), 0);
    }

    #[test]
    fn test_part1_no_overlap() {
        let input = RangeData {
            ranges: vec![(1, 3), (10, 15)],
            values: vec![0, 2, 5, 12, 20],
        };
        assert_eq!(part1(&input), 2); // 2 and 12
    }

    #[test]
    fn test_part1_completely_overlapping() {
        let input = RangeData {
            ranges: vec![(1, 10), (2, 5), (0, 15)],
            values: vec![-1, 0, 5, 15, 16],
        };
        assert_eq!(part1(&input), 3); // 0, 5, 15
    }
}
