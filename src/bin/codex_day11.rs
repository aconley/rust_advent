fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_file_as_lines("11")?;
    let part1_value = part1("you", "out", &inputs)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
    println!("Part 1: {}", part1_value);
    let part2_value = part2("svr", "out", &["dac", "fft"], &inputs)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
    println!("Part 2: {}", part2_value);
    Ok(())
}

/// Part 1: Beam splitter
fn part1(start_vertex: &str, target_vertex: &str, input: &[String]) -> Result<u64, String> {
    let mut graph = parse_graph(input)?;
    graph.entry(start_vertex.to_string()).or_default();
    graph.entry(target_vertex.to_string()).or_default();

    let (nodes, adj, index_map) = build_indexed_graph(&graph);
    detect_cycle(&adj, &nodes)?;

    let start_idx = *index_map
        .get(start_vertex)
        .ok_or_else(|| format!("missing start vertex: {start_vertex}"))?;
    let target_idx = *index_map
        .get(target_vertex)
        .ok_or_else(|| format!("missing target vertex: {target_vertex}"))?;

    let required_bits = vec![None; nodes.len()];
    let mut memo = std::collections::HashMap::new();
    count_paths_with_required(start_idx, target_idx, 0, 0, &adj, &required_bits, &mut memo)
}

fn parse_graph(input: &[String]) -> Result<std::collections::HashMap<String, Vec<String>>, String> {
    let mut graph: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    for (line_idx, line) in input.iter().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (src, rest) = line
            .split_once(':')
            .ok_or_else(|| format!("invalid line {}: missing ':'", line_idx + 1))?;
        let src = src.trim();
        if src.is_empty() {
            return Err(format!(
                "invalid line {}: empty source vertex",
                line_idx + 1
            ));
        }
        let targets: Vec<String> = rest
            .split_whitespace()
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect();
        {
            let edges = graph.entry(src.to_string()).or_default();
            edges.extend(targets.iter().cloned());
        }
        for target in targets {
            graph.entry(target).or_default();
        }
    }
    Ok(graph)
}

fn part2<R: AsRef<str>>(
    start_vertex: &str,
    target_vertex: &str,
    required_vertices: &[R],
    input: &[String],
) -> Result<u64, String> {
    let mut graph = parse_graph(input)?;
    graph.entry(start_vertex.to_string()).or_default();
    graph.entry(target_vertex.to_string()).or_default();
    for vertex in required_vertices {
        graph.entry(vertex.as_ref().to_string()).or_default();
    }

    let (nodes, adj, index_map) = build_indexed_graph(&graph);
    detect_cycle(&adj, &nodes)?;

    let mut required_bits = vec![None; nodes.len()];
    let mut next_bit = 0u8;
    for vertex in required_vertices {
        let name = vertex.as_ref();
        let idx = *index_map
            .get(name)
            .ok_or_else(|| format!("missing required vertex: {name}"))?;
        if required_bits[idx].is_none() {
            if next_bit >= 64 {
                return Err("too many required vertices for bitmask".to_string());
            }
            required_bits[idx] = Some(next_bit);
            next_bit += 1;
        }
    }

    let full_mask = if next_bit == 64 {
        u64::MAX
    } else {
        (1u64 << next_bit) - 1
    };
    let start_idx = *index_map
        .get(start_vertex)
        .ok_or_else(|| format!("missing start vertex: {start_vertex}"))?;
    let target_idx = *index_map
        .get(target_vertex)
        .ok_or_else(|| format!("missing target vertex: {target_vertex}"))?;

    let start_mask = apply_required_bit(0, start_idx, &required_bits);
    if start_idx == target_idx {
        return Ok(if start_mask == full_mask { 1 } else { 0 });
    }

    let mut memo = std::collections::HashMap::new();
    count_paths_with_required(
        start_idx,
        target_idx,
        start_mask,
        full_mask,
        &adj,
        &required_bits,
        &mut memo,
    )
}

fn build_indexed_graph(
    graph: &std::collections::HashMap<String, Vec<String>>,
) -> (
    Vec<String>,
    Vec<Vec<usize>>,
    std::collections::HashMap<String, usize>,
) {
    let mut nodes: Vec<String> = graph.keys().cloned().collect();
    nodes.sort();
    let mut index_map = std::collections::HashMap::new();
    for (idx, name) in nodes.iter().enumerate() {
        index_map.insert(name.clone(), idx);
    }
    let mut adj = vec![Vec::new(); nodes.len()];
    for (src, targets) in graph {
        let src_idx = index_map[src];
        let edges = &mut adj[src_idx];
        for target in targets {
            if let Some(&target_idx) = index_map.get(target) {
                edges.push(target_idx);
            }
        }
    }
    (nodes, adj, index_map)
}

fn detect_cycle(adj: &[Vec<usize>], nodes: &[String]) -> Result<(), String> {
    let mut state = vec![0u8; adj.len()];
    for node in 0..adj.len() {
        if state[node] == 0 {
            dfs_cycle(node, adj, nodes, &mut state)?;
        }
    }
    Ok(())
}

fn dfs_cycle(
    node: usize,
    adj: &[Vec<usize>],
    nodes: &[String],
    state: &mut [u8],
) -> Result<(), String> {
    state[node] = 1;
    for &next in &adj[node] {
        if state[next] == 1 {
            return Err(format!("cycle detected involving node: {}", nodes[next]));
        }
        if state[next] == 0 {
            dfs_cycle(next, adj, nodes, state)?;
        }
    }
    state[node] = 2;
    Ok(())
}

fn apply_required_bit(mask: u64, node: usize, required_bits: &[Option<u8>]) -> u64 {
    match required_bits[node] {
        Some(bit) => mask | (1u64 << bit),
        None => mask,
    }
}

fn count_paths_with_required(
    node: usize,
    target: usize,
    mask: u64,
    full_mask: u64,
    adj: &[Vec<usize>],
    required_bits: &[Option<u8>],
    memo: &mut std::collections::HashMap<(usize, u64), u64>,
) -> Result<u64, String> {
    let mask = apply_required_bit(mask, node, required_bits);
    if node == target {
        return Ok(if mask == full_mask { 1 } else { 0 });
    }
    if let Some(&cached) = memo.get(&(node, mask)) {
        return Ok(cached);
    }
    let mut total = 0u64;
    for &next in &adj[node] {
        let count =
            count_paths_with_required(next, target, mask, full_mask, adj, required_bits, memo)?;
        total = total
            .checked_add(count)
            .ok_or_else(|| "path count overflow".to_string())?;
    }
    memo.insert((node, mask), total);
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(input: &[&str]) -> Vec<String> {
        input.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_from_prompt() {
        let input = lines(&[
            "aaa: you hhh",
            "you: bbb ccc",
            "bbb: ddd eee",
            "ccc: ddd eee fff",
            "ddd: ggg",
            "eee: out",
            "fff: out",
            "ggg: out",
            "hhh: ccc fff iii",
            "iii: out",
        ]);
        assert_eq!(part1("you", "out", &input).unwrap(), 5);
    }

    #[test]
    fn start_equals_target_without_edges() {
        let input = lines(&["solo:"]);
        assert_eq!(part1("solo", "solo", &input).unwrap(), 1);
    }

    #[test]
    fn missing_start_vertex() {
        let input = lines(&["a: b", "b:"]);
        assert_eq!(part1("missing", "b", &input).unwrap(), 0);
    }

    #[test]
    fn simple_diamond() {
        let input = lines(&["a: b c", "b: d", "c: d", "d:"]);
        assert_eq!(part1("a", "d", &input).unwrap(), 2);
    }

    #[test]
    fn targets_only_nodes_are_included() {
        let input = lines(&["a: b"]);
        assert_eq!(part1("a", "b", &input).unwrap(), 1);
    }

    #[test]
    fn multi_layer_branching() {
        let input = lines(&["a: b c", "b: d e", "c: d e", "d: f", "e: f", "f:"]);
        assert_eq!(part1("a", "f", &input).unwrap(), 4);
    }

    #[test]
    fn disconnected_graph() {
        let input = lines(&["a: b", "b:", "x: y", "y:"]);
        assert_eq!(part1("a", "y", &input).unwrap(), 0);
    }

    #[test]
    fn parse_rejects_missing_colon() {
        let input = lines(&["a b"]);
        let err = parse_graph(&input).unwrap_err();
        assert!(err.contains("missing ':'"));
    }

    #[test]
    fn parse_rejects_empty_source() {
        let input = lines(&[": b"]);
        let err = parse_graph(&input).unwrap_err();
        assert!(err.contains("empty source vertex"));
    }

    #[test]
    fn cycle_is_error() {
        let input = lines(&["a: b", "b: a"]);
        let err = part1("a", "c", &input).unwrap_err();
        assert!(err.contains("cycle detected"));
    }

    #[test]
    fn part2_example_from_prompt() {
        let input = lines(&[
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
        ]);
        let result = part2("svr", "out", &["fft", "dac"], &input).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_no_required_matches_part1() {
        let input = lines(&["a: b c", "b: d", "c: d", "d:"]);
        let part1_result = part1("a", "d", &input).unwrap();
        let part2_result = part2::<&str>("a", "d", &[], &input).unwrap();
        assert_eq!(part1_result, part2_result);
    }

    #[test]
    fn part2_required_unreachable() {
        let input = lines(&["a: b", "b: out", "x: y"]);
        let result = part2("a", "out", &["x"], &input).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn part2_required_includes_start_and_target() {
        let input = lines(&["a: b", "b: c", "c:"]);
        let result = part2("a", "c", &["a", "c"], &input).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn part2_start_equals_target_without_required() {
        let input = lines(&["solo:"]);
        let result = part2::<&str>("solo", "solo", &[], &input).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn part2_start_equals_target_with_other_required() {
        let input = lines(&["solo:"]);
        let result = part2("solo", "solo", &["other"], &input).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn part2_duplicate_required_vertices() {
        let input = lines(&["a: b", "b: c", "c:"]);
        let result = part2("a", "c", &["b", "b"], &input).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn part2_cycle_is_error() {
        let input = lines(&["a: b", "b: a"]);
        let err = part2::<&str>("a", "b", &[], &input).unwrap_err();
        assert!(err.contains("cycle detected"));
    }
}
