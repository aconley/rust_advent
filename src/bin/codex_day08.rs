use rust_advent::Point;

fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_points("08")?;
    println!("Part 1: {}", part1(1000, 3, &inputs));
    println!("Part 2: {}", part2(&inputs));
    Ok(())
}

fn part1(n: usize, m: usize, inputs: &[Point]) -> usize {
    if n == 0 || m == 0 || inputs.is_empty() {
        return 0;
    }

    let mut heap = std::collections::BinaryHeap::new();
    for i in 0..inputs.len() {
        for j in (i + 1)..inputs.len() {
            let dist = squared_distance(&inputs[i], &inputs[j]);
            let entry = (dist, i, j);
            if heap.len() < n {
                heap.push(entry);
            } else if let Some(&top) = heap.peek() {
                if entry < top {
                    heap.pop();
                    heap.push(entry);
                }
            }
        }
    }

    let mut dsu = DisjointSet::new(inputs.len());
    for (_dist, a, b) in heap.into_iter() {
        dsu.union(a, b);
    }

    let mut comp_sizes = vec![0usize; inputs.len()];
    for i in 0..inputs.len() {
        let root = dsu.find(i);
        comp_sizes[root] += 1;
    }

    let mut sizes: Vec<usize> = comp_sizes.into_iter().filter(|&s| s > 0).collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    let take = m.min(sizes.len());
    if take == 0 {
        return 0;
    }
    sizes.into_iter().take(take).product()
}

fn part2(inputs: &[Point]) -> usize {
    if inputs.len() < 2 {
        return 0;
    }

    let mut edges = Vec::new();
    for i in 0..inputs.len() {
        for j in (i + 1)..inputs.len() {
            let dist = squared_distance(&inputs[i], &inputs[j]);
            edges.push((dist, i, j));
        }
    }
    edges.sort_unstable();

    let mut dsu = DisjointSet::new(inputs.len());
    let mut components = inputs.len();
    for (_dist, a, b) in edges {
        if dsu.union(a, b) {
            components -= 1;
            if components == 1 {
                let xa = inputs[a].x as i64;
                let xb = inputs[b].x as i64;
                return (xa * xb) as usize;
            }
        }
    }
    0
}

fn squared_distance(a: &Point, b: &Point) -> i64 {
    let dx = a.x as i64 - b.x as i64;
    let dy = a.y as i64 - b.y as i64;
    let dz = a.z as i64 - b.z as i64;
    dx * dx + dy * dy + dz * dz
}

struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        let mut size = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
            size.push(1);
        }
        Self { parent, size }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.size[ra] < self.size[rb] {
            self.parent[ra] = rb;
            self.size[rb] += self.size[ra];
        } else {
            self.parent[rb] = ra;
            self.size[ra] += self.size[rb];
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pt(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }

    #[test]
    fn example_small_n1() {
        let inputs = vec![pt(0, 0, 0), pt(2, 2, 2), pt(2, 3, 2)];
        assert_eq!(part1(1, 1, &inputs), 2);
        assert_eq!(part1(1, 2, &inputs), 2);
    }

    #[test]
    fn example_large_n3() {
        let inputs = vec![
            pt(162, 817, 812),
            pt(57, 618, 57),
            pt(906, 360, 560),
            pt(592, 479, 940),
            pt(352, 342, 300),
            pt(466, 668, 158),
            pt(542, 29, 236),
            pt(431, 825, 988),
            pt(739, 650, 466),
            pt(52, 470, 668),
            pt(216, 146, 977),
            pt(819, 987, 18),
            pt(117, 168, 530),
            pt(805, 96, 715),
            pt(346, 949, 466),
            pt(970, 615, 88),
            pt(941, 993, 340),
            pt(862, 61, 35),
            pt(984, 92, 344),
            pt(425, 690, 689),
        ];
        assert_eq!(part1(3, 1, &inputs), 3);
        assert_eq!(part1(3, 2, &inputs), 6);
    }

    #[test]
    fn connects_all_when_n_exceeds_pairs() {
        let inputs = vec![pt(0, 0, 0), pt(1, 0, 0), pt(2, 0, 0), pt(3, 0, 0)];
        assert_eq!(part1(10, 1, &inputs), 4);
    }

    #[test]
    fn multiple_components_product() {
        let inputs = vec![pt(0, 0, 0), pt(10, 0, 0), pt(20, 0, 0), pt(21, 0, 0)];
        assert_eq!(part1(1, 2, &inputs), 2);
        assert_eq!(part1(2, 2, &inputs), 4);
    }

    #[test]
    fn part2_example_small() {
        let inputs = vec![pt(1, 1, 1), pt(2, 3, 4), pt(3, 5, 6)];
        assert_eq!(part2(&inputs), 2);
    }

    #[test]
    fn part2_example_large() {
        let inputs = vec![
            pt(162, 817, 812),
            pt(57, 618, 57),
            pt(906, 360, 560),
            pt(592, 479, 940),
            pt(352, 342, 300),
            pt(466, 668, 158),
            pt(542, 29, 236),
            pt(431, 825, 988),
            pt(739, 650, 466),
            pt(52, 470, 668),
            pt(216, 146, 977),
            pt(819, 987, 18),
            pt(117, 168, 530),
            pt(805, 96, 715),
            pt(346, 949, 466),
            pt(970, 615, 88),
            pt(941, 993, 340),
            pt(862, 61, 35),
            pt(984, 92, 344),
            pt(425, 690, 689),
        ];
        assert_eq!(part2(&inputs), 25272);
    }

    #[test]
    fn part2_extra_case() {
        let inputs = vec![pt(0, 0, 0), pt(1, 0, 0), pt(0, 1, 0), pt(10, 0, 0)];
        assert_eq!(part2(&inputs), 10);
    }
}
