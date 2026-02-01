fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_file_as_lines("10")?;
    match part1(&inputs) {
        Ok(value) => println!("Part 1: {}", value),
        Err(err) => println!("Part 1 error: {}", err),
    }
    match part2(&inputs) {
        Ok(value) => println!("Part 2: {}", value),
        Err(err) => println!("Part 2 error: {}", err),
    }
    Ok(())
}

/// Part 1: Beam splitter
fn part1(input: &[String]) -> Result<u64, String> {
    let mut total = 0u64;
    for (line_idx, line) in input.iter().enumerate() {
        let (end_mask, step_masks, _targets, positions) =
            parse_configuration(line).map_err(|err| format!("line {}: {}", line_idx + 1, err))?;
        if step_masks.len() > 63 {
            return Err(format!("line {}: too many steps", line_idx + 1));
        }
        let steps = min_steps(end_mask, &step_masks, positions)
            .ok_or_else(|| format!("line {}: no solution found", line_idx + 1))?;
        total = total
            .checked_add(steps)
            .ok_or_else(|| format!("line {}: total overflow", line_idx + 1))?;
    }
    Ok(total)
}

fn part2(input: &[String]) -> Result<u64, String> {
    let mut total = 0u64;
    for (line_idx, line) in input.iter().enumerate() {
        let (_end_mask, step_masks, targets, positions) =
            parse_configuration(line).map_err(|err| format!("line {}: {}", line_idx + 1, err))?;
        if step_masks.len() > 64 {
            return Err(format!("line {}: too many steps", line_idx + 1));
        }
        if targets.len() != positions {
            return Err(format!(
                "line {}: target length {} does not match positions {}",
                line_idx + 1,
                targets.len(),
                positions
            ));
        }
        let steps = min_steps_part2_seeded(&step_masks, &targets, positions)
            .ok_or_else(|| format!("line {}: no solution found", line_idx + 1))?;
        total = total
            .checked_add(steps)
            .ok_or_else(|| format!("line {}: total overflow", line_idx + 1))?;
    }
    Ok(total)
}

fn parse_configuration(line: &str) -> Result<(u32, Vec<u32>, Vec<u32>, usize), String> {
    let start = line.find('[').ok_or("missing '['")?;
    let end = line[start + 1..]
        .find(']')
        .map(|idx| start + 1 + idx)
        .ok_or("missing ']'")?;
    let endstate = &line[start + 1..end];
    if endstate.is_empty() {
        return Err("endstate is empty".into());
    }
    let positions = endstate.len();
    if positions > 32 {
        return Err(format!("too many positions: {}", positions));
    }
    let mut end_mask = 0u32;
    for (idx, ch) in endstate.chars().enumerate() {
        match ch {
            '.' => {}
            '#' => end_mask |= 1u32 << idx,
            _ => return Err(format!("invalid endstate char '{}'", ch)),
        }
    }

    let rest = &line[end + 1..];
    let steps_section_end = rest.find('{').unwrap_or(rest.len());
    let steps_section = &rest[..steps_section_end];
    let mut step_masks = Vec::new();
    let mut cursor = 0usize;
    while let Some(open_idx) = steps_section[cursor..].find('(') {
        let open_idx = cursor + open_idx;
        let close_idx = steps_section[open_idx + 1..]
            .find(')')
            .map(|idx| open_idx + 1 + idx)
            .ok_or("missing ')' in step")?;
        let step_body = steps_section[open_idx + 1..close_idx].trim();
        if step_body.is_empty() {
            return Err("empty step".into());
        }
        let mut mask = 0u32;
        for token in step_body.split(',') {
            let token = token.trim();
            if token.is_empty() {
                return Err("empty index in step".into());
            }
            let idx: usize = token
                .parse()
                .map_err(|_| format!("invalid index '{}'", token))?;
            if idx >= positions {
                return Err(format!("index {} out of range", idx));
            }
            let bit = 1u32 << idx;
            if mask & bit != 0 {
                return Err(format!("duplicate index {} in step", idx));
            }
            mask |= bit;
        }
        step_masks.push(mask);
        cursor = close_idx + 1;
    }

    if step_masks.is_empty() {
        return Err("no steps provided".into());
    }

    let targets = parse_targets(rest, positions)?;
    Ok((end_mask, step_masks, targets, positions))
}

fn min_steps(end_mask: u32, step_masks: &[u32], positions: usize) -> Option<u64> {
    if end_mask == 0 {
        return Some(0);
    }
    let _ = positions;
    let mut dist_forward = std::collections::HashMap::new();
    let mut dist_backward = std::collections::HashMap::new();
    let mut queue_forward = std::collections::VecDeque::new();
    let mut queue_backward = std::collections::VecDeque::new();

    dist_forward.insert(0u32, 0u64);
    dist_backward.insert(end_mask, 0u64);
    queue_forward.push_back(0u32);
    queue_backward.push_back(end_mask);

    while !queue_forward.is_empty() && !queue_backward.is_empty() {
        if queue_forward.len() <= queue_backward.len() {
            if let Some(result) = expand_bfs_layer(
                &mut queue_forward,
                &mut dist_forward,
                &dist_backward,
                step_masks,
            ) {
                return Some(result);
            }
        } else if let Some(result) = expand_bfs_layer(
            &mut queue_backward,
            &mut dist_backward,
            &dist_forward,
            step_masks,
        ) {
            return Some(result);
        }
    }
    None
}

fn parse_targets(rest: &str, positions: usize) -> Result<Vec<u32>, String> {
    let open = rest.find('{').ok_or("missing '{'")?;
    let close = rest[open + 1..]
        .find('}')
        .map(|idx| open + 1 + idx)
        .ok_or("missing '}'")?;
    let body = rest[open + 1..close].trim();
    if body.is_empty() {
        return Err("empty target list".into());
    }
    let mut targets = Vec::new();
    for token in body.split(',') {
        let token = token.trim();
        if token.is_empty() {
            return Err("empty target value".into());
        }
        let value: u32 = token
            .parse()
            .map_err(|_| format!("invalid target '{}'", token))?;
        targets.push(value);
    }
    if targets.len() != positions {
        return Err(format!(
            "target length {} does not match positions {}",
            targets.len(),
            positions
        ));
    }
    Ok(targets)
}

fn min_steps_part2(step_masks: &[u32], targets: &[u32], positions: usize) -> Option<u64> {
    if targets.iter().all(|&v| v == 0) {
        return Some(0);
    }
    let mut coverage = vec![0u32; positions];
    for &mask in step_masks {
        for idx in 0..positions {
            if (mask >> idx) & 1 == 1 {
                coverage[idx] += 1;
            }
        }
    }
    for (idx, &target) in targets.iter().enumerate() {
        if target > 0 && coverage[idx] == 0 {
            return None;
        }
    }

    let target_mask = targets
        .iter()
        .enumerate()
        .fold(0u32, |acc, (idx, &v)| acc | ((v & 1) << idx));
    if !reachable_mod2(step_masks, target_mask) {
        return None;
    }

    let step_indices = step_indices(step_masks, positions);
    let mut steps_order: Vec<usize> = (0..step_masks.len()).collect();
    steps_order.sort_by_key(|&i| std::cmp::Reverse(step_indices[i].len()));

    let max_step_size = step_indices
        .iter()
        .map(|v| v.len() as u64)
        .max()
        .unwrap_or(1);

    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashMap};

    let start = targets.to_vec();
    let mut heap = BinaryHeap::new();
    let h0 = heuristic(&start, max_step_size);
    heap.push(Reverse(Node {
        f: h0,
        g: 0,
        state: start.clone(),
    }));
    let mut best_g: HashMap<Vec<u32>, u64> = HashMap::new();
    best_g.insert(start, 0);

    let mut best_solution: Option<u64> = None;
    while let Some(Reverse(node)) = heap.pop() {
        if let Some(best) = best_solution {
            if node.f >= best {
                continue;
            }
        }
        if node.state.iter().all(|&v| v == 0) {
            best_solution = Some(node.g);
            break;
        }
        if let Some(&known) = best_g.get(&node.state) {
            if node.g != known {
                continue;
            }
        }
        for &step_idx in &steps_order {
            let indices = &step_indices[step_idx];
            if indices.is_empty() {
                continue;
            }
            let mut can_apply = true;
            for &idx in indices {
                if node.state[idx] == 0 {
                    can_apply = false;
                    break;
                }
            }
            if !can_apply {
                continue;
            }
            let mut next = node.state.clone();
            for &idx in indices {
                next[idx] -= 1;
            }
            let next_g = node.g + 1;
            if let Some(best) = best_solution {
                let est = next_g + heuristic(&next, max_step_size);
                if est >= best {
                    continue;
                }
            }
            let entry = best_g.entry(next.clone()).or_insert(u64::MAX);
            if next_g < *entry {
                *entry = next_g;
                let f = next_g + heuristic(&next, max_step_size);
                heap.push(Reverse(Node {
                    f,
                    g: next_g,
                    state: next,
                }));
            }
        }
    }
    best_solution
}

fn min_steps_part2_seeded(step_masks: &[u32], targets: &[u32], positions: usize) -> Option<u64> {
    const MAX_SEED_ENUM: usize = 20;
    if targets.iter().all(|&v| v == 0) {
        return Some(0);
    }
    let target_mask = targets
        .iter()
        .enumerate()
        .fold(0u32, |acc, (idx, &v)| acc | ((v & 1) << idx));

    let (particular, basis) = solve_gf2(step_masks, target_mask, positions)?;
    if basis.len() > MAX_SEED_ENUM {
        return min_steps_part2(step_masks, targets, positions);
    }

    let step_indices = step_indices(step_masks, positions);
    let mut cache: std::collections::HashMap<Vec<u32>, Option<u64>> =
        std::collections::HashMap::new();
    let mut best: Option<u64> = None;
    let total_seeds = 1u64 << basis.len();

    for seed_bits in 0..total_seeds {
        let mut seed_mask = particular;
        for (idx, basis_vec) in basis.iter().enumerate() {
            if (seed_bits >> idx) & 1 == 1 {
                seed_mask ^= basis_vec;
            }
        }
        let seed_steps = seed_mask.count_ones() as u64;
        if let Some(best_steps) = best {
            if seed_steps >= best_steps {
                continue;
            }
        }

        let mut residual: Vec<i64> = targets.iter().map(|&v| v as i64).collect();
        let mut feasible = true;
        for step_idx in 0..step_masks.len() {
            if ((seed_mask >> step_idx) & 1) == 1 {
                for &pos in &step_indices[step_idx] {
                    residual[pos] -= 1;
                    if residual[pos] < 0 {
                        feasible = false;
                        break;
                    }
                }
                if !feasible {
                    break;
                }
            }
        }
        if !feasible {
            continue;
        }

        let mut even_targets = Vec::with_capacity(residual.len());
        for &val in &residual {
            if (val & 1) != 0 {
                feasible = false;
                break;
            }
            even_targets.push((val / 2) as u32);
        }
        if !feasible {
            continue;
        }

        let sub = if even_targets.iter().all(|&v| v == 0) {
            Some(0)
        } else if let Some(cached) = cache.get(&even_targets) {
            *cached
        } else {
            let result = min_steps_part2(step_masks, &even_targets, positions);
            cache.insert(even_targets.clone(), result);
            result
        };

        if let Some(sub_steps) = sub {
            let total = seed_steps + 2 * sub_steps;
            if best.map_or(true, |b| total < b) {
                best = Some(total);
            }
        }
    }
    best
}

fn heuristic(state: &[u32], max_step_size: u64) -> u64 {
    let mut max_remaining = 0u64;
    let mut sum_remaining = 0u64;
    for &v in state {
        let val = v as u64;
        if val > max_remaining {
            max_remaining = val;
        }
        sum_remaining += val;
    }
    let sum_bound = (sum_remaining + max_step_size - 1) / max_step_size;
    std::cmp::max(max_remaining, sum_bound)
}

fn reachable_mod2(step_masks: &[u32], target_mask: u32) -> bool {
    let mut basis = [0u32; 32];
    for &mask in step_masks {
        let mut value = mask;
        while value != 0 {
            let pivot = 31usize.saturating_sub(value.leading_zeros() as usize);
            if basis[pivot] == 0 {
                basis[pivot] = value;
                break;
            } else {
                value ^= basis[pivot];
            }
        }
    }
    let mut value = target_mask;
    while value != 0 {
        let pivot = 31usize.saturating_sub(value.leading_zeros() as usize);
        if basis[pivot] == 0 {
            return false;
        }
        value ^= basis[pivot];
    }
    true
}

fn step_indices(step_masks: &[u32], positions: usize) -> Vec<Vec<usize>> {
    step_masks
        .iter()
        .map(|&mask| {
            (0..positions)
                .filter(|&idx| (mask >> idx) & 1 == 1)
                .collect::<Vec<_>>()
        })
        .collect()
}

fn solve_gf2(step_masks: &[u32], target_mask: u32, positions: usize) -> Option<(u64, Vec<u64>)> {
    let m = step_masks.len();
    if m > 64 {
        return None;
    }
    let mut rows: Vec<(u64, u8)> = Vec::with_capacity(positions);
    for pos in 0..positions {
        let mut mask = 0u64;
        for (idx, &step_mask) in step_masks.iter().enumerate() {
            if ((step_mask >> pos) & 1) == 1 {
                mask |= 1u64 << idx;
            }
        }
        let rhs = ((target_mask >> pos) & 1) as u8;
        rows.push((mask, rhs));
    }

    let mut pivot_cols = Vec::new();
    let mut pivot_col_for_row: Vec<Option<usize>> = vec![None; rows.len()];
    let mut row = 0usize;
    for col in 0..m {
        let mut pivot_row = None;
        for r in row..rows.len() {
            if ((rows[r].0 >> col) & 1) == 1 {
                pivot_row = Some(r);
                break;
            }
        }
        if let Some(p) = pivot_row {
            rows.swap(row, p);
            pivot_col_for_row.swap(row, p);
            let (pivot_mask, pivot_rhs) = rows[row];
            for r in 0..rows.len() {
                if r != row && ((rows[r].0 >> col) & 1) == 1 {
                    rows[r].0 ^= pivot_mask;
                    rows[r].1 ^= pivot_rhs;
                }
            }
            pivot_cols.push(col);
            pivot_col_for_row[row] = Some(col);
            row += 1;
            if row == rows.len() {
                break;
            }
        }
    }

    for (mask, rhs) in &rows {
        if *mask == 0 && *rhs == 1 {
            return None;
        }
    }

    let mut pivot_rows: Vec<(usize, u64, u8)> = Vec::new();
    for (idx, (mask, rhs)) in rows.iter().enumerate() {
        let Some(pivot) = pivot_col_for_row[idx] else {
            continue;
        };
        if *mask == 0 {
            continue;
        }
        let mask_without_pivot = mask & !(1u64 << pivot);
        pivot_rows.push((pivot, mask_without_pivot, *rhs));
    }

    let mut particular = 0u64;
    for (pivot, mask_without, rhs) in &pivot_rows {
        let parity = (mask_without & particular).count_ones() & 1;
        let value = (rhs ^ parity as u8) & 1;
        if value == 1 {
            particular |= 1u64 << pivot;
        }
    }

    let mut is_pivot = vec![false; m];
    for (pivot, _, _) in &pivot_rows {
        if *pivot < m {
            is_pivot[*pivot] = true;
        }
    }
    let free_vars: Vec<usize> = (0..m).filter(|&i| !is_pivot[i]).collect();

    let mut basis = Vec::new();
    for &free in &free_vars {
        let mut vec = 0u64;
        vec |= 1u64 << free;
        for (pivot, mask_without, _) in &pivot_rows {
            let parity = (mask_without & vec).count_ones() & 1;
            if parity == 1 {
                vec |= 1u64 << pivot;
            }
        }
        basis.push(vec);
    }
    Some((particular, basis))
}

#[derive(Clone, Eq, PartialEq)]
struct Node {
    f: u64,
    g: u64,
    state: Vec<u32>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f.cmp(&other.f).then_with(|| self.g.cmp(&other.g))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn expand_bfs_layer(
    queue: &mut std::collections::VecDeque<u32>,
    dist_this: &mut std::collections::HashMap<u32, u64>,
    dist_other: &std::collections::HashMap<u32, u64>,
    step_masks: &[u32],
) -> Option<u64> {
    let layer_size = queue.len();
    for _ in 0..layer_size {
        let state = queue.pop_front().expect("layer size checked");
        let base = dist_this.get(&state).copied().unwrap_or(0);
        for &mask in step_masks {
            let next = state ^ mask;
            if dist_this.contains_key(&next) {
                continue;
            }
            let next_dist = base + 1;
            if let Some(&other_dist) = dist_other.get(&next) {
                return Some(next_dist + other_dist);
            }
            dist_this.insert(next, next_dist);
            queue.push_back(next);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{min_steps, min_steps_part2, parse_configuration, part1, part2};

    #[test]
    fn examples_from_prompt() {
        let input = vec![
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string(),
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}".to_string(),
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string(),
        ];
        let result = part1(&input).expect("part1 ok");
        assert_eq!(result, 7);
    }

    #[test]
    fn zero_steps_needed() {
        let input = vec!["[....] (0) {0,0,0,0}".to_string()];
        let result = part1(&input).expect("part1 ok");
        assert_eq!(result, 0);
    }

    #[test]
    fn no_solution_returns_error() {
        let input = vec!["[#.] (1) {0,0}".to_string()];
        let err = part1(&input).unwrap_err();
        assert!(err.contains("no solution"));
    }

    #[test]
    fn parse_rejects_invalid_index() {
        let err = parse_configuration("[.#] (2) {0}").unwrap_err();
        assert!(err.contains("out of range"));
    }

    #[test]
    fn parse_rejects_empty_steps() {
        let err = parse_configuration("[#.] {1}").unwrap_err();
        assert!(err.contains("no steps"));
    }

    #[test]
    fn parse_rejects_too_many_positions() {
        let line = format!("[{}] (0) {{1}}", "#".repeat(33));
        let err = parse_configuration(&line).unwrap_err();
        assert!(err.contains("too many positions"));
    }

    #[test]
    fn min_steps_simple() {
        let (end_mask, steps, _targets, positions) =
            parse_configuration("[#] (0) {1}").expect("parse ok");
        let steps_needed = min_steps(end_mask, &steps, positions).expect("solution exists");
        assert_eq!(steps_needed, 1);
    }

    #[test]
    fn part2_examples_from_prompt() {
        let input = vec![
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string(),
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}".to_string(),
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string(),
        ];
        let result = part2(&input).expect("part2 ok");
        assert_eq!(result, 33);
    }

    #[test]
    fn part2_simple_case() {
        let input = vec!["[#] (0) {4}".to_string()];
        let result = part2(&input).expect("part2 ok");
        assert_eq!(result, 4);
    }

    #[test]
    fn part2_overlap_prefers_combo() {
        let input = vec!["[..] (0) (1) (0,1) {2,2}".to_string()];
        let result = part2(&input).expect("part2 ok");
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_no_solution_due_to_missing_coverage() {
        let input = vec!["[..] (0) {0,1}".to_string()];
        let err = part2(&input).unwrap_err();
        assert!(err.contains("no solution"));
    }

    #[test]
    fn part2_no_solution_parity() {
        let input = vec!["[..] (0,1) {1,0}".to_string()];
        let err = part2(&input).unwrap_err();
        assert!(err.contains("no solution"));
    }

    #[test]
    fn part2_even_target_with_odd_counts() {
        let input = vec!["[...] (0,1) (1,2) (0,2) {2,2,2}".to_string()];
        let result = part2(&input).expect("part2 ok");
        assert_eq!(result, 3);
    }

    #[test]
    #[ignore]
    fn part2_seeded_worst_case_demo() {
        let input = vec![
            ("[..........] \
(0) (1) (2) (3) (4) (5) (6) (7) (8) (9) \
(0,1) (1,2) (2,3) (3,4) (4,5) (5,6) (6,7) (7,8) (8,9) (0,9) \
(0,2) (1,3) (2,4) (3,5) (4,6) (5,7) (6,8) (7,9) (0,5) (1,6) \
{2,2,2,2,2,2,2,2,2,2}")
                .to_string(),
        ];
        let _ = part2(&input);
    }

    // Codex's solution is much too slow here.
    #[test]
    #[ignore]
    fn part2_hard_example_runs() {
        let input = vec!["[#..##.###.] (0,1,2,3,5,6,7,8) (0,1,2,4,6,7,8,9) (5,8,9) (3,4,6,7) (3,5,6) (1,4,8,9) (2,3,7,8,9) (0,1,2,6,7,8) (0,6,9) (0,5,7,8,9) (0,2,3,4,6,7,8,9) (1,4,6,9) (1,2,5,6) {225,56,230,208,204,28,256,231,235,246}".to_string()];
        let _ = part2(&input).expect("part2 ok");
    }

    #[test]
    fn min_steps_part2_direct() {
        let (_end_mask, step_masks, targets, positions) =
            parse_configuration("[..] (0) (1) {1,2}").expect("parse ok");
        let steps = min_steps_part2(&step_masks, &targets, positions).expect("solution exists");
        assert_eq!(steps, 3);
    }
}
