use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fmt;

fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_file_as_lines("10")?;
    println!("Part 1: {}", part1(&inputs).unwrap());
    println!("Part 2: {}", part2(&inputs).unwrap());
    Ok(())
}

/// Error type for parsing configuration strings
#[derive(Debug)]
enum ParseError {
    EmptyEndstate,
    InvalidBrackets,
    EmptySteps,
    InvalidPosition(usize, usize),
    ParseIntError(String),
    ConfigurationTooLarge(usize),
    MissingTargets,
    InvalidTargets,
    MismatchedLength,
    TooManySteps(usize),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::EmptyEndstate => write!(f, "Endstate cannot be empty"),
            ParseError::InvalidBrackets => write!(f, "Invalid or missing brackets"),
            ParseError::EmptySteps => write!(f, "No steps provided"),
            ParseError::InvalidPosition(pos, max) => {
                write!(f, "Invalid position {} (max: {})", pos, max)
            }
            ParseError::ParseIntError(s) => write!(f, "Failed to parse integer: {}", s),
            ParseError::ConfigurationTooLarge(size) => {
                write!(f, "Configuration too large: {} positions (max 32)", size)
            }
            ParseError::MissingTargets => write!(f, "Missing target values in braces"),
            ParseError::InvalidTargets => write!(f, "Invalid or missing target braces"),
            ParseError::MismatchedLength => {
                write!(f, "Number of targets doesn't match number of positions")
            }
            ParseError::TooManySteps(count) => {
                write!(f, "Too many steps: {} (max 64)", count)
            }
        }
    }
}

impl Error for ParseError {}

/// Configuration representing a puzzle instance
#[derive(Debug)]
struct Configuration {
    endstate: Vec<bool>,
    target_counts: Vec<u64>, // Target counts for Part 2
    steps: Vec<Vec<usize>>,
    step_masks: Vec<u32>, // Precomputed XOR mask for each step (Part 1)
}

/// Compute XOR masks for each step (precomputation for performance)
fn compute_step_masks(steps: &[Vec<usize>]) -> Vec<u32> {
    steps
        .iter()
        .map(|positions| {
            positions
                .iter()
                .fold(0u32, |mask, &pos| mask | (1u32 << pos))
        })
        .collect()
}

/// Parse endstate from configuration string
fn parse_endstate(line: &str) -> Result<(Vec<bool>, usize), ParseError> {
    let start = line.find('[').ok_or(ParseError::InvalidBrackets)?;
    let end = line.find(']').ok_or(ParseError::InvalidBrackets)?;

    if end <= start {
        return Err(ParseError::InvalidBrackets);
    }

    let endstate_str = &line[start + 1..end];
    if endstate_str.is_empty() {
        return Err(ParseError::EmptyEndstate);
    }

    let endstate: Vec<bool> = endstate_str
        .chars()
        .filter(|&c| c == '#' || c == '.')
        .map(|c| c == '#')
        .collect();

    if endstate.len() > 32 {
        return Err(ParseError::ConfigurationTooLarge(endstate.len()));
    }

    Ok((endstate, end))
}

/// Parse steps from configuration string
fn parse_steps(line: &str, end_bracket: usize, max_pos: usize) -> Result<Vec<Vec<usize>>, ParseError> {
    let steps_start = end_bracket + 1;
    let steps_end = line.find('{').unwrap_or(line.len());
    let steps_str = &line[steps_start..steps_end];

    let mut steps = Vec::new();
    for token in steps_str.split_whitespace() {
        if token.starts_with('(') && token.ends_with(')') {
            let positions_str = &token[1..token.len() - 1];
            let positions: Result<Vec<usize>, _> = positions_str
                .split(',')
                .map(|s| {
                    s.trim()
                        .parse::<usize>()
                        .map_err(|_| ParseError::ParseIntError(s.to_string()))
                })
                .collect();

            let positions = positions?;
            for &pos in &positions {
                if pos >= max_pos {
                    return Err(ParseError::InvalidPosition(pos, max_pos - 1));
                }
            }
            steps.push(positions);
        }
    }

    if steps.is_empty() {
        return Err(ParseError::EmptySteps);
    }

    Ok(steps)
}

/// Parse target counts from configuration string
fn parse_targets(line: &str) -> Result<Vec<u64>, ParseError> {
    let start = line.find('{').ok_or(ParseError::MissingTargets)?;
    let end = line.find('}').ok_or(ParseError::InvalidTargets)?;

    if end <= start {
        return Err(ParseError::InvalidTargets);
    }

    let targets_str = &line[start + 1..end];
    let targets: Result<Vec<u64>, _> = targets_str
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<u64>()
                .map_err(|_| ParseError::ParseIntError(s.to_string()))
        })
        .collect();

    targets
}

/// Parse a configuration string
fn parse_configuration(line: &str) -> Result<Configuration, ParseError> {
    let (endstate, end_bracket) = parse_endstate(line)?;
    let steps = parse_steps(line, end_bracket, endstate.len())?;
    let targets = parse_targets(line)?;
    let step_masks = compute_step_masks(&steps);

    if targets.len() != endstate.len() {
        return Err(ParseError::MismatchedLength);
    }

    if steps.len() > 64 {
        return Err(ParseError::TooManySteps(steps.len()));
    }

    Ok(Configuration {
        endstate,
        target_counts: targets,
        steps,
        step_masks,
    })
}

/// Convert endstate to u32 bitmask
fn endstate_to_bitmask(endstate: &[bool]) -> u32 {
    endstate
        .iter()
        .enumerate()
        .filter(|(_, active)| **active)
        .fold(0u32, |mask, (i, _)| mask | (1u32 << i))
}

/// Find minimum steps using BFS
fn find_minimum_steps(config: &Configuration) -> Result<Option<usize>, String> {
    if config.endstate.len() > 32 {
        return Err(format!(
            "Configuration too large: {} positions (max 32)",
            config.endstate.len()
        ));
    }

    let initial: u32 = 0; // All off
    let goal: u32 = endstate_to_bitmask(&config.endstate);

    // Check if already at goal
    if initial == goal {
        return Ok(Some(0));
    }

    let mut queue: VecDeque<(u32, usize)> = VecDeque::new();
    let mut visited: HashSet<u32> = HashSet::new();

    queue.push_back((initial, 0));
    visited.insert(initial);

    while let Some((state, step_count)) = queue.pop_front() {
        for (step_idx, _) in config.steps.iter().enumerate() {
            let next = state ^ config.step_masks[step_idx];

            if next == goal {
                return Ok(Some(step_count + 1));
            }

            if visited.insert(next) {
                queue.push_back((next, step_count + 1));
            }
        }
    }

    Ok(None) // No solution found - goal is unreachable
}

/// Check if target is potentially reachable (simple heuristic)
/// For each position, verify at least one step can increment it
fn is_potentially_reachable(config: &Configuration) -> bool {
    for (pos, &target) in config.target_counts.iter().enumerate() {
        if target > 0 {
            // Check if any step affects this position
            let has_step = config.steps.iter().any(|step| step.contains(&pos));
            if !has_step {
                return false; // No step can increment this position
            }
        }
    }
    true
}

/// Generate all ways to partition `total` among `num_slots` bins (stars and bars)
/// Calls the callback for each partition
fn generate_partitions<F>(total: usize, num_slots: usize, callback: &mut F) -> bool
where
    F: FnMut(&[usize]) -> bool,
{
    let mut partition = vec![0; num_slots];
    generate_partitions_recursive(total, 0, num_slots, &mut partition, callback)
}

fn generate_partitions_recursive<F>(
    remaining: usize,
    slot_idx: usize,
    num_slots: usize,
    partition: &mut [usize],
    callback: &mut F,
) -> bool
where
    F: FnMut(&[usize]) -> bool,
{
    if slot_idx == num_slots - 1 {
        // Last slot gets all remaining
        partition[slot_idx] = remaining;
        return callback(partition);
    }

    // Try all possible values for this slot
    for value in 0..=remaining {
        partition[slot_idx] = value;
        if generate_partitions_recursive(remaining - value, slot_idx + 1, num_slots, partition, callback) {
            return true; // Found solution, early exit
        }
    }

    false
}

/// Find minimum steps for Part 2 by enumerating solutions
///
/// Algorithm: Instead of exploring states (exponential in target values),
/// enumerate all possible distributions of k step applications among m steps,
/// for k = 0, 1, 2, ... This is much more efficient when targets are large.
///
/// Complexity: O(sum over k of C(k+m-1, m-1)) where m = num_steps
fn find_minimum_steps_part2(config: &Configuration) -> Result<Option<usize>, String> {
    let n = config.target_counts.len();
    let m = config.steps.len();

    // Early termination: check if already at goal
    if config.target_counts.iter().all(|&t| t == 0) {
        return Ok(Some(0));
    }

    // Early detection: check if target is potentially reachable
    if !is_potentially_reachable(config) {
        return Ok(None);
    }

    // Upper bound: sum of all targets (worst case, each position needs individual steps)
    let upper_bound = config.target_counts.iter().sum::<u64>() as usize;
    let reasonable_limit = upper_bound.min(10000); // Cap search to prevent infinite loops

    let show_progress = upper_bound > 100;
    let mut last_progress = 0;

    if show_progress {
        eprintln!(
            "Part 2: Enumerating solutions (targets: {:?}, max_search: {})",
            config.target_counts, reasonable_limit
        );
    }

    // Try each total step count k = 0, 1, 2, ...
    for k in 0..=reasonable_limit {
        if show_progress && k > 0 && k % 10 == 0 && k != last_progress {
            eprintln!("  Trying k={} step applications...", k);
            last_progress = k;
        }

        // Generate all ways to partition k among m steps
        let mut found = false;
        generate_partitions(k, m, &mut |partition| {
            // partition[i] = number of times to apply step i
            let mut counts = vec![0u64; n];

            // Apply each step the specified number of times
            for (step_idx, &times) in partition.iter().enumerate() {
                for &pos in &config.steps[step_idx] {
                    counts[pos] += times as u64;
                }
            }

            // Check if this partition produces the target counts
            if counts == config.target_counts {
                found = true;
                return true; // Signal early exit
            }

            false // Continue searching
        });

        if found {
            if show_progress {
                eprintln!("  Solution found at k={}", k);
            }
            return Ok(Some(k));
        }
    }

    if show_progress {
        eprintln!("  No solution found within search limit");
    }

    Ok(None) // No solution found within reasonable limit
}

/// Part 1: Find minimum steps for each configuration and sum
fn part1(input: &[String]) -> Result<u64, Box<dyn Error>> {
    let mut total = 0u64;

    for (line_num, line) in input.iter().enumerate() {
        let config = parse_configuration(line)?;

        match find_minimum_steps(&config)? {
            Some(steps) => total += steps as u64,
            None => {
                return Err(format!(
                    "No solution found for line {}: target state is unreachable with given steps",
                    line_num + 1
                )
                .into())
            }
        }
    }

    Ok(total)
}

/// Part 2: Find minimum step applications to reach target counts and sum
fn part2(input: &[String]) -> Result<u64, Box<dyn Error>> {
    let mut total = 0u64;

    for (line_num, line) in input.iter().enumerate() {
        let config = parse_configuration(line)?;

        match find_minimum_steps_part2(&config)? {
            Some(steps) => total += steps as u64,
            None => {
                return Err(format!(
                    "No solution found for line {}: target counts cannot be reached with given steps",
                    line_num + 1
                )
                .into())
            }
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = vec!["[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string()];
        assert_eq!(part1(&input).unwrap(), 2);
    }

    #[test]
    fn test_example_2() {
        let input =
            vec!["[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}".to_string()];
        assert_eq!(part1(&input).unwrap(), 3);
    }

    #[test]
    fn test_example_3() {
        let input =
            vec!["[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string()];
        assert_eq!(part1(&input).unwrap(), 2);
    }

    #[test]
    fn test_all_examples_combined() {
        let input = vec![
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string(),
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}".to_string(),
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string(),
        ];
        assert_eq!(part1(&input).unwrap(), 7); // 2 + 3 + 2
    }

    #[test]
    fn test_already_at_goal() {
        let input = vec!["[....] (0) (1) (2,3) {0,0,0,0}".to_string()];
        assert_eq!(part1(&input).unwrap(), 0);
    }

    #[test]
    fn test_single_position() {
        let input = vec!["[#] (0) {1}".to_string()];
        assert_eq!(part1(&input).unwrap(), 1);
    }

    #[test]
    fn test_single_step_needed() {
        let input = vec!["[##] (0,1) {1,1}".to_string()];
        assert_eq!(part1(&input).unwrap(), 1);
    }

    #[test]
    fn test_unreachable_state() {
        let input = vec!["[.#.] (0) (2) {1}".to_string()];
        assert!(part1(&input).is_err());
    }

    #[test]
    fn test_multiple_paths_same_length() {
        let input = vec!["[##..] (0,1) (0) (1) {1,1,0,0}".to_string()];
        assert_eq!(part1(&input).unwrap(), 1); // (0,1) is optimal
    }

    #[test]
    fn test_all_on() {
        let input = vec!["[####] (0,1,2,3) {1,1,1,1}".to_string()];
        assert_eq!(part1(&input).unwrap(), 1);
    }

    #[test]
    fn test_parse_empty_endstate() {
        let input = vec!["[] (0) {1}".to_string()];
        assert!(part1(&input).is_err());
    }

    #[test]
    fn test_parse_no_steps() {
        let input = vec!["[#] {1}".to_string()];
        assert!(part1(&input).is_err());
    }

    #[test]
    fn test_parse_invalid_position() {
        let input = vec!["[.#] (5) {1}".to_string()];
        assert!(part1(&input).is_err());
    }

    #[test]
    fn test_parse_missing_brackets() {
        let input = vec![".# (0) {1}".to_string()];
        assert!(part1(&input).is_err());
    }

    #[test]
    fn test_parse_malformed_step() {
        let input = vec!["[.#] (a,b) {1}".to_string()];
        assert!(part1(&input).is_err());
    }

    #[test]
    fn test_max_size_32_positions() {
        // Test with 15 positions to keep runtime reasonable
        let endstate = "#".repeat(15);
        let steps = (0..15)
            .map(|i| format!("({})", i))
            .collect::<Vec<_>>()
            .join(" ");
        let targets = vec!["1"; 15].join(",");
        let input = vec![format!("[{}] {} {{{}}}", endstate, steps, targets)];
        assert_eq!(part1(&input).unwrap(), 15);
    }

    #[test]
    fn test_at_size_limit() {
        // Test that 32 positions is accepted (but use a simple case)
        let endstate = ".".repeat(31) + "#";
        let targets = vec!["0"; 31].iter().chain(&["1"]).cloned().collect::<Vec<_>>().join(",");
        let input = vec![format!("[{}] (31) {{{}}}", endstate, targets)];
        assert_eq!(part1(&input).unwrap(), 1);
    }

    #[test]
    fn test_size_exceeds_limit() {
        let endstate = "#".repeat(33);
        let steps = (0..33)
            .map(|i| format!("({})", i))
            .collect::<Vec<_>>()
            .join(" ");
        let input = vec![format!("[{}] {} {{1}}", endstate, steps)];
        assert!(part1(&input).is_err());
    }

    #[test]
    fn test_complex_toggle_sequence() {
        let input = vec!["[.#.#] (0,1) (1,2) (2,3) {0,1,0,1}".to_string()];
        let result = part1(&input);
        assert!(result.is_ok());
        // With steps (0,1), (1,2), (2,3), we need to find a sequence
        // Start: [., ., ., .]  (0000)
        // Goal:  [., #, ., #]  (0101)
        // Possible: (1,2) -> [., #, #, .] (0110), then (2,3) -> [., #, ., #] (0101) = 2 steps
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_no_curly_braces() {
        // Test case missing curly braces - should error
        let input = vec!["[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1)".to_string()];
        assert!(part1(&input).is_err()); // Should fail due to missing targets
    }

    #[test]
    fn test_single_on_multiple_ways() {
        // Multiple steps can activate position 0
        let input = vec!["[#...] (0) (0,1) (0,2) {1,0,0,0}".to_string()];
        assert_eq!(part1(&input).unwrap(), 1); // Any single step works
    }

    // ===== Part 2 Tests =====

    #[test]
    fn test_part2_example_1() {
        let input = vec!["[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string()];
        assert_eq!(part2(&input).unwrap(), 10);
    }

    #[test]
    fn test_part2_example_2() {
        let input =
            vec!["[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}".to_string()];
        assert_eq!(part2(&input).unwrap(), 12);
    }

    #[test]
    fn test_part2_example_3() {
        let input =
            vec!["[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string()];
        assert_eq!(part2(&input).unwrap(), 11);
    }

    #[test]
    fn test_part2_all_examples_combined() {
        let input = vec![
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string(),
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}".to_string(),
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string(),
        ];
        assert_eq!(part2(&input).unwrap(), 33); // 10 + 12 + 11
    }

    #[test]
    fn test_part2_already_at_goal() {
        // Target is all zeros
        let input = vec!["[....] (0) (1) (2,3) {0,0,0,0}".to_string()];
        assert_eq!(part2(&input).unwrap(), 0);
    }

    #[test]
    fn test_part2_single_position() {
        // Need to apply step 5 times
        let input = vec!["[#] (0) {5}".to_string()];
        assert_eq!(part2(&input).unwrap(), 5);
    }

    #[test]
    fn test_part2_single_step_needed() {
        // Apply (0,1) once
        let input = vec!["[##] (0,1) {1,1}".to_string()];
        assert_eq!(part2(&input).unwrap(), 1);
    }

    #[test]
    fn test_part2_unreachable_target() {
        // Position 1 can't be reached (no step touches it)
        let input = vec!["[.#.] (0) (2) {1,1,1}".to_string()];
        assert!(part2(&input).is_err());
    }

    #[test]
    fn test_part2_multiple_applications() {
        // Need to apply steps multiple times
        let input = vec!["[##] (0) (1) {3,4}".to_string()];
        assert_eq!(part2(&input).unwrap(), 7); // 3 times (0) + 4 times (1)
    }

    #[test]
    fn test_part2_overlapping_steps() {
        // Steps that affect multiple positions
        let input = vec!["[###] (0,1) (1,2) {2,3,1}".to_string()];
        // One solution: (0,1) twice, (1,2) once -> {2,3,1}
        assert_eq!(part2(&input).unwrap(), 3);
    }

    #[test]
    fn test_part2_too_many_steps() {
        // Create 65 steps (> 64 limit)
        let endstate = ".".repeat(65);
        let steps = (0..65)
            .map(|i| format!("({})", i))
            .collect::<Vec<_>>()
            .join(" ");
        let targets = vec!["1"; 65].join(",");
        let input = vec![format!("[{}] {} {{{}}}", endstate, steps, targets)];
        assert!(part2(&input).is_err());
    }

    #[test]
    fn test_part2_mismatched_length() {
        // 4 positions but only 3 targets
        let input = vec!["[....] (0) (1) (2) (3) {1,2,3}".to_string()];
        assert!(part2(&input).is_err());
    }

    #[test]
    fn test_part2_larger_targets() {
        // Larger target values
        let input = vec!["[#] (0) {10}".to_string()];
        assert_eq!(part2(&input).unwrap(), 10);
    }

    #[test]
    fn test_part2_complex_combination() {
        // Multiple steps affecting overlapping positions
        let input = vec!["[####] (0,1) (1,2) (2,3) (0,3) {3,3,3,3}".to_string()];
        let result = part2(&input);
        assert!(result.is_ok());
        // Should find a valid combination
        assert!(result.unwrap() > 0);
    }

    #[test]
    fn test_part2_no_overlap() {
        // Steps don't overlap - straightforward solution
        let input = vec!["[##] (0) (1) {5,7}".to_string()];
        assert_eq!(part2(&input).unwrap(), 12); // 5 + 7
    }

    // ===== Error Handling Tests =====

    #[test]
    fn test_part1_error_message_includes_line_number() {
        // Second line is unsolvable
        let input = vec![
            "[#] (0) {1}".to_string(),
            "[.#.] (0) (2) {0,1,0}".to_string(), // Position 1 unreachable
        ];
        let result = part1(&input);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("line 2"));
        assert!(err_msg.contains("unreachable"));
    }

    #[test]
    fn test_part2_error_message_includes_line_number() {
        // Second line is unsolvable
        let input = vec![
            "[#] (0) {5}".to_string(),
            "[.#.] (0) (2) {1,1,1}".to_string(), // Position 1 unreachable
        ];
        let result = part2(&input);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("line 2"));
        assert!(err_msg.contains("cannot be reached"));
    }

    #[test]
    fn test_part1_stops_on_first_unsolvable() {
        // First line is solvable, second is not, third is solvable
        let input = vec![
            "[#] (0) {1}".to_string(),
            "[.#.] (0) (2) {0,1,0}".to_string(), // Unsolvable
            "[##] (0,1) {1,1}".to_string(),
        ];
        let result = part1(&input);
        assert!(result.is_err());
        // Should fail on line 2, not process line 3
    }

    #[test]
    fn test_part2_impossible_target_too_high() {
        // Target value is unreachable because no step affects position 1
        let input = vec!["[##] (0) (0) {1,5}".to_string()]; // Position 1 can't be reached
        assert!(part2(&input).is_err());
    }

    #[test]
    fn test_part2_early_detection_optimization() {
        // This should be caught by early detection (position 2 has no step)
        let input = vec!["[###] (0) (1) {1,1,5}".to_string()];
        let result = part2(&input);
        assert!(result.is_err());
        // Should fail quickly without exploring many states
    }

    #[test]
    fn test_part2_zero_targets_with_steps() {
        // All targets are zero but we have steps (should be 0)
        let input = vec!["[##] (0) (1) {0,0}".to_string()];
        assert_eq!(part2(&input).unwrap(), 0);
    }
}
