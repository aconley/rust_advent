use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_file_as_lines("11")?;
    match part1("you", "out", &inputs) {
        Ok(count) => println!("Part 1: {}", count),
        Err(e) => eprintln!("Part 1 Error: {}", e),
    }
    match part2("svr", "out", &["dac", "fft"], &inputs) {
        Ok(count) => println!("Part 2: {}", count),
        Err(e) => eprintln!("Part 2 Error: {}", e),
    }
    Ok(())
}

struct Graph {
    adjacency_list: Vec<Vec<usize>>,
}

impl Graph {
    fn len(&self) -> usize {
        self.adjacency_list.len()
    }

    fn neighbors(&self, id: usize) -> &[usize] {
        &self.adjacency_list[id]
    }
}

struct SearchState {
    memo: Vec<Option<u64>>,
    visiting: Vec<bool>,
    allowed: Vec<bool>,
}

impl SearchState {
    fn new(size: usize) -> Self {
        Self {
            memo: vec![None; size],
            visiting: vec![false; size],
            allowed: vec![true; size],
        }
    }

    fn prepare_for_search(&mut self) {
        self.memo.fill(None);
        // visiting is assumed to be all false (maintained by DFS invariant)
    }

    fn reset_allowed(&mut self) {
        self.allowed.fill(true);
    }

    fn count_paths(&mut self, start: usize, target: usize, graph: &Graph) -> Result<u64, String> {
        self.prepare_for_search();
        self.dfs(start, target, graph)
    }

    fn dfs(&mut self, current: usize, target: usize, graph: &Graph) -> Result<u64, String> {
        if !self.allowed[current] {
            return Ok(0);
        }
        if current == target {
            return Ok(1);
        }
        if let Some(count) = self.memo[current] {
            return Ok(count);
        }
        if self.visiting[current] {
            return Err("Cycle detected in graph".to_string());
        }

        self.visiting[current] = true;

        let mut total_paths = 0;
        for &neighbor in graph.neighbors(current) {
            total_paths += self.dfs(neighbor, target, graph)?;
        }

        self.visiting[current] = false;
        self.memo[current] = Some(total_paths);
        Ok(total_paths)
    }
}

/// Part 1: Beam splitter
fn part1(start_vertex: &str, target_vertex: &str, input: &[String]) -> Result<u64, String> {
    let (graph, name_to_id) = parse_graph(input)?;

    let start_id = match name_to_id.get(start_vertex) {
        Some(&id) => id,
        None => {
            return if start_vertex == target_vertex {
                Ok(1)
            } else {
                Ok(0)
            };
        }
    };

    let target_id = match name_to_id.get(target_vertex) {
        Some(&id) => id,
        None => usize::MAX,
    };

    let mut state = SearchState::new(graph.len());
    state.count_paths(start_id, target_id, &graph)
}

fn part2<R: AsRef<str>>(
    start_vertex: &str,
    target_vertex: &str,
    required_vertices: &[R],
    input: &[String],
) -> Result<u64, String> {
    let (graph, name_to_id) = parse_graph(input)?;

    let start_id =
        match resolve_start_id(start_vertex, target_vertex, required_vertices, &name_to_id) {
            Ok(Some(id)) => id,
            Ok(None) => return Ok(1), // start == target, reqs met
            Err(_) => return Ok(0),   // start not in graph, reqs not met or start != target
        };

    let target_id = match name_to_id.get(target_vertex) {
        Some(&id) => id,
        None => usize::MAX,
    };

    let required_ids =
        match get_required_ids_or_fail(required_vertices, &name_to_id, start_vertex, target_vertex)
        {
            Some(ids) => ids,
            None => return Ok(0),
        };

    let k = required_ids.len();
    if k > 20 {
        return Err("Too many required vertices (limit 20)".to_string());
    }

    let mut state = SearchState::new(graph.len());
    let mut total_pos: u64 = 0;
    let mut total_neg: u64 = 0;

    // Inclusion-Exclusion Principle
    for i in 0..(1 << k) {
        state.reset_allowed();

        let mut subset_size = 0;
        for bit in 0..k {
            if (i >> bit) & 1 == 1 {
                state.allowed[required_ids[bit]] = false;
                subset_size += 1;
            }
        }

        let count = state.count_paths(start_id, target_id, &graph)?;

        if subset_size % 2 == 1 {
            total_neg += count;
        } else {
            total_pos += count;
        }
    }

    if total_pos >= total_neg {
        Ok(total_pos - total_neg)
    } else {
        Err("Calculation error: negative path count (overflow?)".to_string())
    }
}

fn resolve_start_id<R: AsRef<str>>(
    start_vertex: &str,
    target_vertex: &str,
    required_vertices: &[R],
    name_to_id: &HashMap<String, usize>,
) -> Result<Option<usize>, ()> {
    match name_to_id.get(start_vertex) {
        Some(&id) => Ok(Some(id)),
        None => {
            if start_vertex != target_vertex {
                return Err(());
            }
            // start == target. Path is [start]. Check requirements.
            for r in required_vertices {
                if r.as_ref() != start_vertex {
                    return Err(());
                }
            }
            Ok(None)
        }
    }
}

fn get_required_ids_or_fail<R: AsRef<str>>(
    required_vertices: &[R],
    name_to_id: &HashMap<String, usize>,
    start_vertex: &str,
    target_vertex: &str,
) -> Option<Vec<usize>> {
    let mut ids = Vec::new();
    for r in required_vertices {
        let name = r.as_ref();
        if name == start_vertex || name == target_vertex {
            continue;
        }
        match name_to_id.get(name) {
            Some(&id) => ids.push(id),
            None => return None, // Signal impossible
        }
    }
    ids.sort_unstable();
    ids.dedup();
    Some(ids)
}

fn parse_graph(input: &[String]) -> Result<(Graph, HashMap<String, usize>), String> {
    let mut name_to_id: HashMap<String, usize> = HashMap::new();
    let mut adjacency_list: Vec<Vec<usize>> = Vec::new();

    for (line_idx, line) in input.iter().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let (src_str, targets_str) = line
            .split_once(": ")
            .ok_or_else(|| format!("Line {}: Invalid format (missing ': ')", line_idx + 1))?;

        let src_id = get_or_create_id(src_str.trim(), &mut name_to_id, &mut adjacency_list);

        for target_str in targets_str.split_whitespace() {
            let target_id = get_or_create_id(target_str, &mut name_to_id, &mut adjacency_list);
            adjacency_list[src_id].push(target_id);
        }
    }

    Ok((Graph { adjacency_list }, name_to_id))
}

fn get_or_create_id(
    name: &str,
    name_to_id: &mut HashMap<String, usize>,
    adjacency_list: &mut Vec<Vec<usize>>,
) -> usize {
    if let Some(&id) = name_to_id.get(name) {
        id
    } else {
        let id = name_to_id.len();
        name_to_id.insert(name.to_string(), id);
        adjacency_list.push(Vec::new());
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example_case() {
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
        assert_eq!(part1("you", "out", &input).unwrap(), 5);
    }

    #[test]
    fn test_part2_example_case() {
        let input = vec![
            "svr: aaa bbb".to_string(),
            "aaa: fft".to_string(),
            "fft: ccc".to_string(),
            "bbb: tty".to_string(),
            "tty: ccc".to_string(),
            "ccc: ddd eee".to_string(),
            "ddd: hub".to_string(),
            "hub: fff".to_string(),
            "eee: dac".to_string(),
            "dac: fff".to_string(),
            "fff: ggg hhh".to_string(),
            "ggg: out".to_string(),
            "hhh: out".to_string(),
        ];
        let req = vec!["fft", "dac"];
        assert_eq!(part2("svr", "out", &req, &input).unwrap(), 2);
    }

    #[test]
    fn test_part2_no_reqs() {
        // Should match part 1
        let input = vec![
            "start: a b".to_string(),
            "a: end".to_string(),
            "b: end".to_string(),
        ];
        let req: Vec<&str> = vec![];
        assert_eq!(part2("start", "end", &req, &input).unwrap(), 2);
    }

    #[test]
    fn test_part2_impossible_req() {
        let input = vec![
            "start: a b".to_string(),
            "a: end".to_string(),
            "b: end".to_string(),
        ];
        // 'c' is not in graph
        let req = vec!["c"];
        assert_eq!(part2("start", "end", &req, &input).unwrap(), 0);
    }

    #[test]
    fn test_part2_start_is_req() {
        let input = vec!["start: end".to_string()];
        let req = vec!["start"];
        // Start is implicitly visited
        assert_eq!(part2("start", "end", &req, &input).unwrap(), 1);
    }

    #[test]
    fn test_part2_req_blocks_path() {
        // If we require 'a' and 'b', but paths are disjoint?
        // start -> a -> end
        // start -> b -> end
        // No path visits both.
        let input = vec![
            "start: a b".to_string(),
            "a: end".to_string(),
            "b: end".to_string(),
        ];
        let req = vec!["a", "b"];
        assert_eq!(part2("start", "end", &req, &input).unwrap(), 0);
    }

    #[test]
    fn test_cycle_detection_part2() {
        let input = vec![
            "start: a".to_string(),
            "a: b".to_string(),
            "b: start".to_string(),
        ];
        let req: Vec<&str> = vec![];
        assert!(part2("start", "end", &req, &input).is_err());
    }
}
