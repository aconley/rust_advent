use std::collections::{HashMap, HashSet};

fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_file_as_lines("11")?;
    println!("Part 1: {}", part1("you", "out", &inputs));
    println!("Part 2: {}", part2("svr", "out", &["dac", "fft"], &inputs));
    Ok(())
}

/// Parse input lines into a graph represented as an adjacency list
/// Format: "source: targ1 targ2 targ3"
/// Returns an error if any line is malformed
///
/// Generic over S: AsRef<str> to accept &[String], &[&str], or any string-like slice
fn parse_graph<S: AsRef<str>>(input: &[S]) -> Result<HashMap<String, Vec<String>>, String> {
    let mut graph = HashMap::new();

    for (line_num, line) in input.iter().enumerate() {
        let line = line.as_ref().trim();
        if line.is_empty() {
            continue;
        }

        // Split by ':' to separate source from targets (avoid collecting to Vec)
        let mut parts = line.split(':');
        let source = parts.next().ok_or_else(|| {
            format!(
                "Line {}: Expected format 'source: target1 target2...', got '{}'",
                line_num + 1,
                line
            )
        })?;
        let targets_str = parts.next().ok_or_else(|| {
            format!(
                "Line {}: Expected format 'source: target1 target2...', got '{}'",
                line_num + 1,
                line
            )
        })?;

        // Ensure no extra colons
        if parts.next().is_some() {
            return Err(format!(
                "Line {}: Too many ':' separators in '{}'",
                line_num + 1,
                line
            ));
        }

        let source = source.trim();
        if source.is_empty() {
            return Err(format!(
                "Line {}: Source vertex cannot be empty",
                line_num + 1
            ));
        }

        let targets: Vec<String> = targets_str
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        // Empty target list is valid - represents a vertex with no outgoing edges
        // (e.g., a dead-end that isn't the target)
        graph.insert(source.to_string(), targets);
    }

    Ok(graph)
}

/// Count all distinct paths from start vertex to target vertex
///
/// Uses DFS with memoization for O(V + E) time complexity.
/// Handles cycles correctly by tracking vertices on the current call stack.
fn count_paths(start: &str, target: &str, graph: &HashMap<String, Vec<String>>) -> u32 {
    let mut memo = HashMap::new();
    let mut visiting = HashSet::new();
    count_paths_impl(start, target, graph, &mut memo, &mut visiting)
}

/// Internal implementation of path counting with DFS and memoization
///
/// Why we need BOTH memo and visiting as parameters (not internal variables):
/// - `memo`: Must persist across ALL recursive calls to cache results (shared state)
/// - `visiting`: Must track the CURRENT call stack to detect cycles (shared state)
///
/// If these were local variables, each recursive call would get fresh empty collections,
/// breaking both memoization and cycle detection. They're parameters to share state
/// across the entire recursion tree while keeping them out of the public API.
///
/// The visiting set tracks vertices on the current call stack. When we encounter
/// a vertex already being visited, we've found a cycle and return 0 (no valid paths
/// through this cycle). Once we finish processing a vertex, we cache its result in
/// memo and can safely reuse it from other paths without the cycle restriction.
fn count_paths_impl(
    current: &str,
    target: &str,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, u32>,
    visiting: &mut HashSet<String>,
) -> u32 {
    // Base case: reached the target
    if current == target {
        return 1;
    }

    // Check memo cache (already computed from a previous path)
    if let Some(&count) = memo.get(current) {
        return count;
    }

    // Detect cycle: if currently on the call stack, return 0 to break the cycle
    if visiting.contains(current) {
        return 0;
    }

    // Get neighbors, handle missing vertex or dead-end
    let neighbors = match graph.get(current) {
        Some(n) if !n.is_empty() => n,
        _ => {
            // No outgoing edges: cache and return 0
            // Avoid allocation: use entry API
            memo.entry(current.to_string()).or_insert(0);
            return 0;
        }
    };

    // Mark as visiting (on the call stack) - unavoidable allocation
    visiting.insert(current.to_string());

    // Sum paths from all neighbors
    let mut total = 0;
    for neighbor in neighbors {
        total += count_paths_impl(neighbor, target, graph, memo, visiting);
    }

    // Unmark as visiting (remove from call stack)
    visiting.remove(current);

    // Cache result for future lookups - unavoidable allocation
    memo.entry(current.to_string()).or_insert(total);
    total
}

/// Part 1: Count distinct paths from start_vertex to target_vertex
fn part1<S: AsRef<str>>(start_vertex: &str, target_vertex: &str, input: &[S]) -> u32 {
    let graph = match parse_graph(input) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error parsing graph: {}", e);
            return 0;
        }
    };

    // Edge case: start equals target
    if start_vertex == target_vertex {
        return 1;
    }

    // Edge case: start vertex not in graph
    if !graph.contains_key(start_vertex) {
        return 0;
    }

    count_paths(start_vertex, target_vertex, &graph)
}

/// Helper struct to manage state for path counting with required vertices
/// Groups related parameters to reduce function argument count
struct PathCounter<'a> {
    graph: &'a HashMap<String, Vec<String>>,
    target: &'a str,
    required_map: &'a HashMap<String, usize>,
    all_required_mask: u64,
    memo: HashMap<(String, u64), u64>,
    visiting: HashSet<String>,
}

impl<'a> PathCounter<'a> {
    fn new(
        graph: &'a HashMap<String, Vec<String>>,
        target: &'a str,
        required_map: &'a HashMap<String, usize>,
        all_required_mask: u64,
    ) -> Self {
        Self {
            graph,
            target,
            required_map,
            all_required_mask,
            memo: HashMap::new(),
            visiting: HashSet::new(),
        }
    }

    /// Count paths from current vertex to target with required vertices constraint
    fn count_paths(&mut self, current: &str, visited_required_mask: u64) -> u64 {
        // Update visited mask if current is a required vertex
        let current_mask = if let Some(&idx) = self.required_map.get(current) {
            visited_required_mask | (1u64 << idx)
        } else {
            visited_required_mask
        };

        // Base case: reached target
        if current == self.target {
            // Only count if all required vertices were visited
            return if current_mask == self.all_required_mask {
                1u64
            } else {
                0u64
            };
        }

        // Check memo cache
        let state = (current.to_string(), current_mask);
        if let Some(&count) = self.memo.get(&state) {
            return count;
        }

        // Cycle detection
        if self.visiting.contains(current) {
            return 0u64;
        }

        // Get neighbors
        let neighbors = match self.graph.get(current) {
            Some(n) if !n.is_empty() => n,
            _ => {
                self.memo.insert(state, 0u64);
                return 0u64;
            }
        };

        self.visiting.insert(current.to_string());

        let mut total = 0u64;
        for neighbor in neighbors {
            total += self.count_paths(neighbor, current_mask);
        }

        self.visiting.remove(current);
        self.memo.insert(state, total);
        total
    }
}

/// Part 2: Count paths that pass through all required vertices (in any order)
fn part2<S: AsRef<str>, R: AsRef<str>>(
    start_vertex: &str,
    target_vertex: &str,
    required_vertices: &[R],
    input: &[S],
) -> u64 {
    let graph = match parse_graph(input) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error parsing graph: {}", e);
            return 0;
        }
    };

    // Edge case: start equals target
    if start_vertex == target_vertex {
        // Only valid if no required vertices (or all are start/target)
        return if required_vertices.is_empty() { 1 } else { 0 };
    }

    // Edge case: start vertex not in graph
    if !graph.contains_key(start_vertex) {
        return 0;
    }

    // Create mapping of required vertices to bit indices (for bitmask)
    let required_map: HashMap<String, usize> = required_vertices
        .iter()
        .enumerate()
        .map(|(i, v)| (v.as_ref().to_string(), i))
        .collect();

    let num_required = required_vertices.len();
    let all_required_mask = if num_required == 0 {
        0u64
    } else {
        (1u64 << num_required) - 1
    };

    let mut counter = PathCounter::new(&graph, target_vertex, &required_map, all_required_mask);
    counter.count_paths(start_vertex, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example_from_problem() {
        // The example from the problem statement
        let input = vec![
            "aaa: you hhh".to_string(),
            "you: bbb ccc".to_string(),
            "bbb: ddd eee".to_string(),
            "ccc: ddd eee fff".to_string(),
            "ddd: ggg".to_string(),
            "eee: out".to_string(),
            "fff: out".to_string(),
            "ggg: out".to_string(),
            "hhh: ccc fff iii".to_string(),
            "iii: out".to_string(),
        ];
        assert_eq!(part1("you", "out", &input), 5);
    }

    #[test]
    fn test_part1_empty_input() {
        let input: Vec<String> = vec![];
        assert_eq!(part1("start", "end", &input), 0);
    }

    #[test]
    fn test_part1_start_equals_target() {
        let input = vec!["a: b".to_string()];
        assert_eq!(part1("same", "same", &input), 1);
    }

    #[test]
    fn test_part1_single_direct_path() {
        let input = vec!["a: b".to_string()];
        assert_eq!(part1("a", "b", &input), 1);
    }

    #[test]
    fn test_part1_no_path_exists() {
        let input = vec!["a: b".to_string(), "c: d".to_string()];
        assert_eq!(part1("a", "d", &input), 0);
    }

    #[test]
    fn test_part1_multiple_paths_diamond() {
        // Diamond pattern: a -> b,c -> d (2 paths)
        let input = vec!["a: b c".to_string(), "b: d".to_string(), "c: d".to_string()];
        assert_eq!(part1("a", "d", &input), 2);
    }

    #[test]
    fn test_part1_three_paths() {
        // a -> b,c,d -> e (3 paths)
        let input = vec![
            "a: b c d".to_string(),
            "b: e".to_string(),
            "c: e".to_string(),
            "d: e".to_string(),
        ];
        assert_eq!(part1("a", "e", &input), 3);
    }

    #[test]
    fn test_part1_cycle_no_target() {
        // a -> b -> c -> b (cycle), no path to target
        let input = vec!["a: b".to_string(), "b: c".to_string(), "c: b".to_string()];
        assert_eq!(part1("a", "target", &input), 0);
    }

    #[test]
    fn test_part1_start_not_in_graph() {
        let input = vec!["a: b".to_string()];
        assert_eq!(part1("missing", "b", &input), 0);
    }

    #[test]
    fn test_part1_complex_branching() {
        // Tree with multiple branches
        // a -> b,c
        // b -> d,e
        // c -> f
        // d,e,f -> target
        // Total: 3 paths (a->b->d->target, a->b->e->target, a->c->f->target)
        let input = vec![
            "a: b c".to_string(),
            "b: d e".to_string(),
            "c: f".to_string(),
            "d: target".to_string(),
            "e: target".to_string(),
            "f: target".to_string(),
        ];
        assert_eq!(part1("a", "target", &input), 3);
    }

    #[test]
    fn test_part1_single_vertex_is_target() {
        // Graph with only target vertex, no path from elsewhere
        let input = vec!["other: somewhere".to_string()];
        assert_eq!(part1("start", "target", &input), 0);
    }

    #[test]
    fn test_part1_path_with_convergence() {
        // Multiple paths that converge and then diverge again
        // a -> b,c -> d -> e,f -> g
        // 4 paths: a->b->d->e->g, a->b->d->f->g, a->c->d->e->g, a->c->d->f->g
        let input = vec![
            "a: b c".to_string(),
            "b: d".to_string(),
            "c: d".to_string(),
            "d: e f".to_string(),
            "e: g".to_string(),
            "f: g".to_string(),
        ];
        assert_eq!(part1("a", "g", &input), 4);
    }

    #[test]
    fn test_part1_longer_chain() {
        // Simple chain: a -> b -> c -> d -> e
        let input = vec![
            "a: b".to_string(),
            "b: c".to_string(),
            "c: d".to_string(),
            "d: e".to_string(),
        ];
        assert_eq!(part1("a", "e", &input), 1);
    }

    #[test]
    fn test_part1_cycle_with_exit_to_target() {
        // a -> b -> c -> b (cycle), but also c -> target
        // Should count: a -> b -> c -> target
        let input = vec![
            "a: b".to_string(),
            "b: c".to_string(),
            "c: b target".to_string(),
        ];
        assert_eq!(part1("a", "target", &input), 1);
    }

    #[test]
    fn test_part1_malformed_input_no_colon() {
        // Malformed input should result in 0 paths (with error message)
        let input = vec!["a b c".to_string()];
        assert_eq!(part1("a", "c", &input), 0);
    }

    #[test]
    fn test_part1_malformed_input_empty_source() {
        // Empty source should result in 0 paths (with error message)
        let input = vec![": b c".to_string()];
        assert_eq!(part1("", "c", &input), 0);
    }

    #[test]
    fn test_parse_graph_valid() {
        let input = vec!["a: b c".to_string(), "b: d".to_string()];
        let graph = parse_graph(&input).unwrap();
        assert_eq!(graph.len(), 2);
        assert_eq!(
            graph.get("a").unwrap(),
            &vec!["b".to_string(), "c".to_string()]
        );
        assert_eq!(graph.get("b").unwrap(), &vec!["d".to_string()]);
    }

    #[test]
    fn test_parse_graph_error() {
        let input = vec!["invalid line without colon".to_string()];
        assert!(parse_graph(&input).is_err());
    }

    #[test]
    fn test_part1_with_str_slices() {
        // Demonstrate generic flexibility: can pass &str slices directly
        assert_eq!(part1("a", "b", &["a: b"]), 1);
        assert_eq!(part1("a", "c", &["a: b", "b: c"]), 1);

        // Diamond pattern with string literals
        let result = part1("a", "d", &["a: b c", "b: d", "c: d"]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2_example_from_problem() {
        // The example from the problem statement
        let input = vec![
            "svr: aaa bbb",
            "aaa: fft",
            "fft: ccc",
            "bbb: tty",
            "tty: ccc",
            "ccc: ddd eee",
            "ddd: hub",
            "hub: fff",
            "eee: dac",
            "dac: fff",
            "fff: ggg hhh",
            "ggg: out",
            "hhh: out",
        ];

        // All paths from svr to out (2 branches at start × 2 at ccc × 2 at fff = 8 paths):
        // 1. svr->aaa->fft->ccc->ddd->hub->fff->ggg->out
        // 2. svr->aaa->fft->ccc->ddd->hub->fff->hhh->out
        // 3. svr->aaa->fft->ccc->eee->dac->fff->ggg->out (has fft & dac)
        // 4. svr->aaa->fft->ccc->eee->dac->fff->hhh->out (has fft & dac)
        // 5. svr->bbb->tty->ccc->ddd->hub->fff->ggg->out
        // 6. svr->bbb->tty->ccc->ddd->hub->fff->hhh->out
        // 7. svr->bbb->tty->ccc->eee->dac->fff->ggg->out
        // 8. svr->bbb->tty->ccc->eee->dac->fff->hhh->out
        assert_eq!(part2("svr", "out", &[] as &[&str], &input), 8);

        // With required vertices fft and dac, only paths 3 and 4 qualify
        assert_eq!(part2("svr", "out", &["fft", "dac"], &input), 2);
    }

    #[test]
    fn test_part2_no_required_vertices() {
        // With no required vertices, should match part1
        let input = vec!["a: b c", "b: d", "c: d"];
        assert_eq!(part2("a", "d", &[] as &[&str], &input), 2);
        assert_eq!(part1("a", "d", &input), 2);
    }

    #[test]
    fn test_part2_single_required_vertex() {
        // Diamond with one required vertex
        let input = vec!["a: b c", "b: d", "c: d"];

        // Must pass through b (only 1 path: a->b->d)
        assert_eq!(part2("a", "d", &["b"], &input), 1);

        // Must pass through c (only 1 path: a->c->d)
        assert_eq!(part2("a", "d", &["c"], &input), 1);
    }

    #[test]
    fn test_part2_impossible_required_vertex() {
        // Required vertex not reachable
        let input = vec!["a: b", "b: c", "x: y"];
        assert_eq!(part2("a", "c", &["x"], &input), 0);
    }

    #[test]
    fn test_part2_required_vertex_is_start() {
        // Start vertex is in required list
        let input = vec!["a: b", "b: c"];
        assert_eq!(part2("a", "c", &["a"], &input), 1);
    }

    #[test]
    fn test_part2_required_vertex_is_target() {
        // Target vertex is in required list
        let input = vec!["a: b", "b: c"];
        assert_eq!(part2("a", "c", &["c"], &input), 1);
    }

    #[test]
    fn test_part2_complex_branching() {
        // More complex graph with multiple paths
        let input = vec![
            "a: b c",
            "b: d",
            "c: e",
            "d: f",
            "e: f",
            "f: g h",
            "g: target",
            "h: target",
        ];

        // 4 paths total: a->b->d->f->g->target, a->b->d->f->h->target,
        //                a->c->e->f->g->target, a->c->e->f->h->target
        assert_eq!(part2("a", "target", &[] as &[&str], &input), 4);

        // Require passing through d (eliminates c path) = 2 paths
        assert_eq!(part2("a", "target", &["d"], &input), 2);

        // Require passing through e (eliminates b path) = 2 paths
        assert_eq!(part2("a", "target", &["e"], &input), 2);

        // Require passing through both d and e = 0 paths (impossible)
        assert_eq!(part2("a", "target", &["d", "e"], &input), 0);
    }

    #[test]
    fn test_part2_linear_path() {
        // Simple linear path
        let input = vec!["a: b", "b: c", "c: d"];

        // Must pass through b and c (only 1 path)
        assert_eq!(part2("a", "d", &["b", "c"], &input), 1);

        // Must pass through b only
        assert_eq!(part2("a", "d", &["b"], &input), 1);
    }

    #[test]
    fn test_part2_empty_input() {
        let input: Vec<String> = vec![];
        assert_eq!(part2("a", "b", &[] as &[&str], &input), 0);
    }

    #[test]
    fn test_part2_required_vertices_any_order() {
        // Simpler demonstration: order in which we SPECIFY requirements doesn't matter
        let input = vec!["a: b", "b: c", "c: d"];

        // Specifying ["b", "c"] vs ["c", "b"] should give same result
        // (both b and c must be visited, order doesn't matter)
        assert_eq!(part2("a", "d", &["b", "c"], &input), 1);
        assert_eq!(part2("a", "d", &["c", "b"], &input), 1);

        // The bitmask approach means order of specification is irrelevant
        // Both create the same requirement: visit both b and c
    }
}
