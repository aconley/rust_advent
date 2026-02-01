use rayon::prelude::*;
use std::collections::{HashMap, VecDeque};

fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_file_as_lines("10")?;
    match part1(&inputs) {
        Ok(v) => println!("Part 1: {}", v),
        Err(e) => eprintln!("Part 1 Error: {}", e),
    }
    match part2(&inputs) {
        Ok(v) => println!("Part 2: {}", v),
        Err(e) => eprintln!("Part 2 Error: {}", e),
    }
    Ok(())
}

/// Represents a single configuration of the beam splitter system.
struct Problem {
    /// Number of positions in the system (up to 32).
    num_positions: usize,
    /// Bitmask representing the desired activation state (for Part 1).
    target: u32,
    /// List of bitmasks, where each bitmask represents a step's impact on positions.
    steps: Vec<u32>,
    /// Desired counts for each position (for Part 2).
    target_counts: Vec<u32>,
}

impl Problem {
    /// Parses a problem configuration from a string line.
    /// Format: [endstate] (step1) (step2) ... {target1,target2,...}
    fn parse(input: &str) -> Result<Self, String> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Err("Empty input".to_string());
        }

        // Parse endstate bitmask (Part 1)
        let endstate_str = parts[0];
        if !endstate_str.starts_with('[') || !endstate_str.ends_with(']') {
            return Err("Invalid endstate format".to_string());
        }
        let endstate_content = &endstate_str[1..endstate_str.len() - 1];
        if endstate_content.is_empty() {
            return Err("Endstate cannot be empty".to_string());
        }
        let num_positions = endstate_content.len();
        if num_positions > 32 {
            return Err(format!("Too many positions: {}", num_positions));
        }

        let mut target = 0u32;
        for (i, c) in endstate_content.chars().enumerate() {
            if c == '#' {
                target |= 1 << i;
            } else if c != '.' {
                return Err(format!("Invalid char in endstate: {}", c));
            }
        }

        let mut steps = Vec::new();
        let mut steps_found = false;
        let mut target_counts = Vec::new();

        // Parse steps and target counts
        for part in &parts[1..] {
            if part.starts_with('{') && part.ends_with('}') {
                let content = &part[1..part.len() - 1];
                for num_str in content.split(',') {
                    let val = num_str
                        .trim()
                        .parse::<u32>()
                        .map_err(|_| "Invalid number in target counts")?;
                    target_counts.push(val);
                }
                continue;
            }
            if part.starts_with('(') && part.ends_with(')') {
                steps_found = true;
                let content = &part[1..part.len() - 1];
                let mut step_mask = 0u32;
                if !content.is_empty() {
                    for num_str in content.split(',') {
                        let idx = num_str
                            .parse::<usize>()
                            .map_err(|_| "Invalid number in step")?;
                        if idx >= num_positions {
                            return Err(format!(
                                "Step index {} out of bounds (size {})",
                                idx, num_positions
                            ));
                        }
                        step_mask |= 1 << idx;
                    }
                }
                steps.push(step_mask);
            }
        }

        if !steps_found {
            return Err("No steps provided".to_string());
        }

        if steps.len() >= 64 {
            return Err(format!("Too many steps: {}", steps.len()));
        }

        if !target_counts.is_empty() && target_counts.len() != num_positions {
            return Err(format!(
                "Target counts length {} does not match positions {}",
                target_counts.len(),
                num_positions
            ));
        }

        Ok(Problem {
            num_positions,
            target,
            steps,
            target_counts,
        })
    }
}

/// Part 1: Minimum flips to reach endstate.
/// Each step can be used 0 or 1 times (GF(2) logic).
fn part1(input: &[String]) -> Result<u64, String> {
    let results: Result<Vec<u64>, String> = input
        .par_iter()
        .map(|line| {
            let p = Problem::parse(line)?;
            solve_part1(&p).ok_or_else(|| format!("No solution found for: {}", line))
        })
        .collect();

    Ok(results?.iter().sum())
}

/// Solves Part 1 using a hybrid strategy of BFS and Meet-in-the-Middle on Kernel Basis.
fn solve_part1(p: &Problem) -> Option<u64> {
    let n = p.num_positions;
    let m = p.steps.len();

    let approx_rank = std::cmp::min(n, m);
    let k = m.saturating_sub(approx_rank);
    let bfs_log_cost = (n as f64) + (m as f64).log2();
    let mim_log_cost = k as f64;

    // Use BFS if the state space is small enough.
    if n <= 20 {
        return solve_part1_bfs(p);
    }

    // Otherwise use Meet-in-the-Middle on the kernel basis of the linear system.
    if mim_log_cost < bfs_log_cost || n > 26 {
        solve_part1_mim(p)
    } else {
        solve_part1_bfs(p)
    }
}

/// Simple BFS to find the shortest path in the $2^N$ state graph.
fn solve_part1_bfs(p: &Problem) -> Option<u64> {
    let n = p.num_positions;
    let size = 1 << n;
    let mut dist = vec![u8::MAX; size];

    let start = 0usize;
    let target = p.target as usize;

    dist[start] = 0;
    let mut queue = VecDeque::new();
    // Store (state, min_next_step_index)
    // We only try adding steps with index >= min_next_step_index.
    // Since we want strictly greater than the *previous* step,
    // if we used step `i` to get here, we push `i + 1`.
    queue.push_back((start, 0));

    while let Some((curr, start_idx)) = queue.pop_front() {
        if curr == target {
            return Some(dist[curr] as u64);
        }

        let d = dist[curr];
        if d == u8::MAX {
            continue;
        }

        // Optimization: Only try steps with index >= start_idx.
        // This enforces a canonical ordering (s1 < s2 < s3...), treating the search
        // as finding a combination of steps rather than a permutation.
        for (i, &step) in p.steps.iter().enumerate().skip(start_idx) {
            let next = curr ^ (step as usize);
            if dist[next] == u8::MAX {
                dist[next] = d + 1;
                // Enforce strictly increasing order: next step must be > i
                queue.push_back((next, i + 1));
            }
        }
    }

    None
}

/// Solves Part 1 by finding the kernel of the step matrix and searching for a minimum-weight combination.
fn solve_part1_mim(p: &Problem) -> Option<u64> {
    let n = p.num_positions;
    let m = p.steps.len();

    // Build GF(2) matrix
    let mut matrix = vec![0u64; n];
    for (r, row) in matrix.iter_mut().enumerate().take(n) {
        let mut row_val = 0u64;
        for (c, step) in p.steps.iter().enumerate() {
            if (step >> r) & 1 == 1 {
                row_val |= 1 << c;
            }
        }
        if (p.target >> r) & 1 == 1 {
            row_val |= 1 << m;
        }
        *row = row_val;
    }

    // Gaussian Elimination to find Basis and Kernel
    let mut pivots = Vec::new();
    let mut next_row = 0;
    let mut pivot_cols = vec![false; m];

    for (c, is_pivot) in pivot_cols.iter_mut().enumerate().take(m) {
        if next_row >= n {
            break;
        }
        let mut pivot_row = None;
        for (r, row) in matrix.iter().enumerate().take(n).skip(next_row) {
            if (row >> c) & 1 == 1 {
                pivot_row = Some(r);
                break;
            }
        }
        if let Some(pr) = pivot_row {
            matrix.swap(next_row, pr);
            let pivot_val = matrix[next_row];
            for (r, row) in matrix.iter_mut().enumerate().take(n) {
                if r != next_row && (*row >> c) & 1 == 1 {
                    *row ^= pivot_val;
                }
            }
            pivots.push((next_row, c));
            *is_pivot = true;
            next_row += 1;
        }
    }

    // Consistency check
    for row in matrix.iter().take(n).skip(next_row) {
        if (row >> m) & 1 == 1 {
            return None;
        }
    }

    // Identify free variables and construct kernel basis
    let mut free_cols = Vec::new();
    for (c, &is_pivot) in pivot_cols.iter().enumerate().take(m) {
        if !is_pivot {
            free_cols.push(c);
        }
    }

    let mut d_mask = 0u64;
    for &(r, c) in &pivots {
        if (matrix[r] >> m) & 1 == 1 {
            d_mask |= 1 << c;
        }
    }

    let mut kernel_basis = Vec::new();
    for &f in &free_cols {
        let mut vec = 1u64 << f;
        for &(r, c) in &pivots {
            if (matrix[r] >> f) & 1 == 1 {
                vec |= 1 << c;
            }
        }
        kernel_basis.push(vec);
    }

    // Meet-in-the-Middle search on the kernel subspace
    let k = kernel_basis.len();
    if k == 0 {
        return Some(d_mask.count_ones() as u64);
    }

    let k1 = k / 2;
    let k2 = k - k1;
    let basis1 = &kernel_basis[0..k1];
    let basis2 = &kernel_basis[k1..k];

    let mut sums2 = Vec::with_capacity(1 << k2);
    sums2.push(0u64);
    for &b in basis2 {
        let len = sums2.len();
        for i in 0..len {
            sums2.push(sums2[i] ^ b);
        }
    }

    let mut min_weight = u32::MAX;
    let mut sums1 = vec![0u64];
    for &b in basis1 {
        let len = sums1.len();
        for i in 0..len {
            sums1.push(sums1[i] ^ b);
        }
    }

    for val1 in sums1 {
        let target_for_part2 = d_mask ^ val1;
        for &val2 in &sums2 {
            let w = (target_for_part2 ^ val2).count_ones();
            if w < min_weight {
                min_weight = w;
            }
        }
    }

    Some(min_weight as u64)
}

/// Part 2: Minimum total steps to reach exact target counts.
/// Steps can be used any non-negative integral number of times (Diophantine system).
fn part2(input: &[String]) -> Result<u64, String> {
    let results: Result<Vec<u64>, String> = input
        .par_iter()
        .map(|line| {
            let p = Problem::parse(line)?;
            if p.target_counts.is_empty() {
                return Err("Missing target counts for Part 2".to_string());
            }
            solve_part2(&p).ok_or_else(|| format!("No solution found for: {}", line))
        })
        .collect();

    Ok(results?.iter().sum())
}

/// Solves Part 2 using a recursive approach based on parity decomposition.
///
/// The core insight is that for any valid solution where step $j$ is applied $x_j$ times:
/// $\sum x_j \cdot \text{step}_j = \text{target}$.
///
/// Considering this modulo 2, we must satisfy:
/// $\sum (x_j \pmod 2) \cdot \text{step}_j \equiv \text{target} \pmod 2$.
///
/// This allows us to break the problem down recursively:
/// 1. Find a binary configuration of steps (0 or 1) that satisfies the target's parity.
///    There may be multiple such configurations due to the kernel of the step matrix.
/// 2. For each valid configuration, subtract it from the target. The residual target
///    is guaranteed to be even at every position.
/// 3. Divide the residual by 2 and recurse.
/// 4. The total cost is (steps in configuration) + 2 * (cost of recursive subproblem).
fn solve_part2(p: &Problem) -> Option<u64> {
    // 1. Preprocess steps: remove 0s and duplicates to reduce search space.
    let mut distinct_steps = p.steps.clone();
    distinct_steps.retain(|&s| s != 0);
    distinct_steps.sort_unstable();
    distinct_steps.dedup();

    let m = distinct_steps.len();
    if m == 0 {
        if p.target_counts.iter().all(|&x| x == 0) {
            return Some(0);
        } else {
            return None;
        }
    }

    // 2. Initialize Solver and Memoization table.
    // The solver handles the linear algebra over GF(2) to find parity matches.
    let solver = GF2Solver::new(&distinct_steps, p.num_positions);
    let mut memo = HashMap::new();

    solve_part2_recursive_parity(p.target_counts.clone(), &solver, &mut memo)
}

struct GF2Solver {
    n: usize,
    m: usize,
    steps: Vec<u32>,
    kernel_basis: Vec<u64>,
}

impl GF2Solver {
    /// Constructs a solver for the given steps.
    /// Performs Gaussian Elimination to find the basis of the kernel (null space).
    fn new(steps: &[u32], n: usize) -> Self {
        let m = steps.len();
        // Compute Kernel Basis using Gaussian Elimination on rows
        let mut matrix = vec![0u64; n];
        for (r, row) in matrix.iter_mut().enumerate().take(n) {
            let mut row_val = 0u64;
            for (c, &step) in steps.iter().enumerate() {
                if (step >> r) & 1 == 1 {
                    row_val |= 1 << c;
                }
            }
            *row = row_val;
        }

        let mut pivots = Vec::new(); // (row, col)
        let mut next_row = 0;
        let mut pivot_cols = vec![false; m];

        // Make a copy for kernel computation
        let mut k_matrix = matrix.clone();

        for (c, is_pivot) in pivot_cols.iter_mut().enumerate().take(m) {
            if next_row >= n {
                break;
            }
            let mut pivot_row = None;
            for (r, row) in k_matrix.iter().enumerate().take(n).skip(next_row) {
                if (row >> c) & 1 == 1 {
                    pivot_row = Some(r);
                    break;
                }
            }
            if let Some(pr) = pivot_row {
                k_matrix.swap(next_row, pr);
                let pivot_val = k_matrix[next_row];
                for (r, row) in k_matrix.iter_mut().enumerate().take(n) {
                    if r != next_row && (*row >> c) & 1 == 1 {
                        *row ^= pivot_val;
                    }
                }
                pivots.push((next_row, c));
                *is_pivot = true;
                next_row += 1;
            }
        }

        let mut kernel_basis = Vec::new();
        for (c, &is_pivot) in pivot_cols.iter().enumerate().take(m) {
            if !is_pivot {
                // Free variable c.
                // Basis vector has 1 at c, and for each pivot col p_c,
                // value is determined by the row equation.
                let mut vec = 1u64 << c;
                for &(r, p_c) in &pivots {
                    if (k_matrix[r] >> c) & 1 == 1 {
                        vec |= 1u64 << p_c;
                    }
                }
                kernel_basis.push(vec);
            }
        }

        Self {
            n,
            m,
            steps: steps.to_vec(),
            kernel_basis,
        }
    }

    /// Returns all step combinations (bitmasks) `c` such that `Matrix * c = target_pattern` (mod 2).
    /// Returns an empty vector if the system is inconsistent.
    fn solve(&self, target_pattern: u32) -> Vec<u64> {
        let n = self.n;
        let m = self.m;

        // Build augmented matrix for GE: M | target
        let mut matrix = vec![0u64; n];
        for (r, row) in matrix.iter_mut().enumerate().take(n) {
            let mut row_val = 0u64;
            for (c, &step) in self.steps.iter().enumerate() {
                if (step >> r) & 1 == 1 {
                    row_val |= 1 << c;
                }
            }
            if (target_pattern >> r) & 1 == 1 {
                row_val |= 1 << m; // Augment column
            }
            *row = row_val;
        }

        // GE
        let mut pivots = Vec::new();
        let mut next_row = 0;

        for c in 0..m {
            if next_row >= n {
                break;
            }
            let mut pivot_row = None;
            for (r, row) in matrix.iter().enumerate().take(n).skip(next_row) {
                if (row >> c) & 1 == 1 {
                    pivot_row = Some(r);
                    break;
                }
            }
            if let Some(pr) = pivot_row {
                matrix.swap(next_row, pr);
                let pivot_val = matrix[next_row];
                for (r, row) in matrix.iter_mut().enumerate().take(n) {
                    if r != next_row && (*row >> c) & 1 == 1 {
                        *row ^= pivot_val;
                    }
                }
                pivots.push((next_row, c));
                next_row += 1;
            }
        }

        // Check consistency
        for row in matrix.iter().take(n).skip(next_row) {
            if (row >> m) & 1 == 1 {
                return Vec::new(); // Inconsistent
            }
        }

        // Find particular solution (set free vars to 0)
        let mut particular = 0u64;
        for &(r, c) in pivots.iter().rev() {
            if (matrix[r] >> m) & 1 == 1 {
                particular |= 1 << c;
            }
        }

        // Generate all solutions
        let mut solutions = Vec::new();
        solutions.push(particular);

        // Expand kernel to find all valid parity patterns
        for &k_vec in &self.kernel_basis {
            let len = solutions.len();
            for i in 0..len {
                solutions.push(solutions[i] ^ k_vec);
            }
        }

        solutions
    }
}

fn solve_part2_recursive_parity(
    target: Vec<u32>,
    solver: &GF2Solver,
    memo: &mut HashMap<Vec<u32>, Option<u64>>,
) -> Option<u64> {
    // Base case: target is all zeros, cost is 0.
    if target.iter().all(|&x| x == 0) {
        return Some(0);
    }
    // Memoization check
    if let Some(&res) = memo.get(&target) {
        return res;
    }

    // Determine target parity pattern
    let mut pattern = 0u32;
    for (i, &val) in target.iter().enumerate() {
        if val % 2 == 1 {
            pattern |= 1 << i;
        }
    }

    // Find all step combinations that match the target's parity
    let candidates = solver.solve(pattern);
    if candidates.is_empty() {
        memo.insert(target, None);
        return None;
    }

    let mut min_total = None;

    for c_mask in candidates {
        // Calculate the residual target after applying this candidate step mask
        let mut next_target = target.clone();
        let mut possible = true;
        let mut step_cost = 0;

        for i in 0..solver.m {
            if (c_mask >> i) & 1 == 1 {
                step_cost += 1;
                let step_vec = solver.steps[i];
                for (pos, target_val) in next_target.iter_mut().enumerate().take(solver.n) {
                    if (step_vec >> pos) & 1 == 1 {
                        if *target_val == 0 {
                            possible = false; // Cannot apply step if count is already 0
                            break;
                        }
                        *target_val -= 1;
                    }
                }
                if !possible {
                    break;
                }
            }
        }

        if possible {
            // Because the parity matched, all values in next_target must be even.
            // We can now divide the problem size by 2.
            for x in &mut next_target {
                *x /= 2;
            }

            // Recursive call
            if let Some(sub_cost) = solve_part2_recursive_parity(next_target, solver, memo) {
                let total = step_cost + 2 * sub_cost;
                if min_total.is_none_or(|m| total < m) {
                    min_total = Some(total);
                }
            }
        }
    }

    memo.insert(target, min_total);
    min_total
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Part 1 Tests ---

    #[test]
    fn test_part1_example_1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let p = Problem::parse(input).unwrap();
        assert_eq!(solve_part1(&p), Some(2));
    }

    #[test]
    fn test_part1_example_2() {
        let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let p = Problem::parse(input).unwrap();
        assert_eq!(solve_part1(&p), Some(3));
    }

    #[test]
    fn test_part1_example_3() {
        let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let p = Problem::parse(input).unwrap();
        assert_eq!(solve_part1(&p), Some(2));
    }

    #[test]
    fn test_part1_impossible_target() {
        let p = Problem::parse("[#.] (1) {0,0}").unwrap();
        assert_eq!(solve_part1(&p), None);
    }

    #[test]
    fn test_part1_trivial_empty_target() {
        let p = Problem::parse("[....] (0,1) (2,3) {0,0,0,0}").unwrap();
        assert_eq!(solve_part1(&p), Some(0));
    }

    #[test]
    fn test_part1_redundant_steps() {
        let p = Problem::parse("[.#] (0) (0) (0,1)").unwrap();
        assert_eq!(solve_part1(&p), Some(2));
    }

    #[test]
    fn test_part1_scaling_max_positions() {
        let mut s = String::from("[################################] ");
        for i in 0..32 {
            s.push_str(&format!("({}) ", i));
        }
        let p = Problem::parse(&s).unwrap();
        assert_eq!(solve_part1(&p), Some(32));
    }

    #[test]
    fn test_part1_scaling_bfs_limit() {
        let s = String::from(
            "[....................] (0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19)",
        );
        let p = Problem::parse(&s).unwrap();
        assert_eq!(solve_part1(&p), Some(0));
    }

    // --- Part 2 Tests ---

    #[test]
    fn test_part2_example_1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let p = Problem::parse(input).unwrap();
        assert_eq!(solve_part2(&p), Some(10));
    }

    #[test]
    fn test_part2_example_2() {
        let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let p = Problem::parse(input).unwrap();
        assert_eq!(solve_part2(&p), Some(12));
    }

    #[test]
    fn test_part2_example_3() {
        let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let p = Problem::parse(input).unwrap();
        assert_eq!(solve_part2(&p), Some(11));
    }

    #[test]
    fn test_part2_trivial_target() {
        let p = Problem::parse("[.] (0) {0}").unwrap();
        assert_eq!(solve_part2(&p), Some(0));
    }

    #[test]
    fn test_part2_no_solution() {
        let p = Problem::parse("[..] (0,1) {1,0}").unwrap();
        assert_eq!(solve_part2(&p), None);
    }

    #[test]
    fn test_part2_redundant_steps() {
        let p = Problem::parse("[.] (0) (0) {2}").unwrap();
        assert_eq!(solve_part2(&p), Some(2));
    }

    #[test]
    fn test_part2_optimization_free_variable() {
        let p = Problem::parse("[..] (0) (1) (0,1) {10,10}").unwrap();
        assert_eq!(solve_part2(&p), Some(10));
    }

    #[test]
    fn test_part2_large_target_scaling() {
        let p = Problem::parse("[.] (0) {100}").unwrap();
        assert_eq!(solve_part2(&p), Some(100));
    }

    #[test]
    fn test_part2_second_failure() {
        let input = "[#..#....#] (2,4,6,8) (1,3,4) (0,1,2,4,5,7,8) (4,5,6,8) (1,2,3,5,6) (2,6,7,8) (0,2,3,4,5,6,7) (0,1,2,4,6,7,8) (0,2,3,4,6,7) (0,3,7,8) {65,49,88,60,82,65,88,67,78}";
        let p = Problem::parse(input).unwrap();
        let result = solve_part2(&p);
        assert_eq!(result, Some(121));
    }

    #[test]
    fn test_part2_hard_case() {
        let input = "[#..##.###.] (0,1,2,3,5,6,7,8) (0,1,2,4,6,7,8,9) (5,8,9) (3,4,6,7) (3,5,6) (1,4,8,9) (2,3,7,8,9) (0,1,2,6,7,8) (0,6,9) (0,5,7,8,9) (0,2,3,4,6,7,8,9) (1,4,6,9) (1,2,5,6) {225,56,230,208,204,28,256,231,235,246}";
        let p = Problem::parse(input).unwrap();
        assert_eq!(solve_part2(&p), Some(283));
    }

    // Disabled due to poor performance.
    #[test]
    #[ignore]
    fn test_part2_seeded_worst_case_demo() {
        let input = "[..........] \
(0) (1) (2) (3) (4) (5) (6) (7) (8) (9) \
(0,1) (1,2) (2,3) (3,4) (4,5) (5,6) (6,7) (7,8) (8,9) (0,9) \
(0,2) (1,3) (2,4) (3,5) (4,6) (5,7) (6,8) (7,9) (0,5) (1,6) \
{2,2,2,2,2,2,2,2,2,2}";
        let p = Problem::parse(input).unwrap();
        assert!(solve_part2(&p).is_some());
    }

    // --- Parsing Tests ---

    #[test]
    fn test_parsing_empty() {
        assert!(Problem::parse("").is_err());
    }

    #[test]
    fn test_parsing_invalid_str() {
        assert!(Problem::parse("invalid").is_err());
    }

    #[test]
    fn test_parsing_missing_steps() {
        assert!(Problem::parse("[.#]").is_err());
    }

    #[test]
    fn test_parsing_bad_step_format() {
        assert!(Problem::parse("[.#] (a)").is_err());
    }
}
