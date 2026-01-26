use rayon::prelude::*;
use rust_advent::Point;
use std::collections::{BinaryHeap, HashMap};
use std::sync::Mutex;

fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_points("08")?;
    println!("Part 1: {}", part1(1000, 3, &inputs));
    println!("Part 2: {}", part2(&inputs));
    Ok(())
}

/// Union-Find data structure for tracking connected components
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return;
        }

        // union by rank
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
    }
}

/// Calculate squared Euclidean distance between two points
fn squared_distance(p1: &Point, p2: &Point) -> i64 {
    let dx = (p1.x as i64) - (p2.x as i64);
    let dy = (p1.y as i64) - (p2.y as i64);
    let dz = (p1.z as i64) - (p2.z as i64);
    dx * dx + dy * dy + dz * dz
}

/// Find the n closest pairs of points globally (parallelized with early termination)
fn find_n_closest_pairs(points: &[Point], n: usize) -> Vec<(usize, usize)> {
    if n == 0 || points.len() < 2 {
        return Vec::new();
    }

    // Thread-safe heap for parallel updates
    let heap = Mutex::new(BinaryHeap::<(i64, usize, usize)>::new());

    // Parallel examination of all pairs
    (0..points.len()).into_par_iter().for_each(|i| {
        let mut local_candidates = Vec::new();

        for j in (i + 1)..points.len() {
            // Early termination heuristic: check if this pair could possibly be close enough
            let mut should_compute = true;
            if let Ok(guard) = heap.lock() {
                if guard.len() >= n {
                    if let Some(&(max_dist, _, _)) = guard.peek() {
                        // Quick check: if coordinate differences are too large, skip
                        let dx = (points[i].x - points[j].x).abs() as i64;
                        let dy = (points[i].y - points[j].y).abs() as i64;
                        let dz = (points[i].z - points[j].z).abs() as i64;

                        // If any single coordinate difference squared exceeds max_dist, skip
                        if dx * dx > max_dist || dy * dy > max_dist || dz * dz > max_dist {
                            should_compute = false;
                        }
                    }
                }
            }

            if should_compute {
                let dist = squared_distance(&points[i], &points[j]);
                local_candidates.push((dist, i, j));
            }
        }

        // Update global heap with local candidates
        if !local_candidates.is_empty() {
            if let Ok(mut guard) = heap.lock() {
                for candidate in local_candidates {
                    if guard.len() < n {
                        guard.push(candidate);
                    } else if let Some(&(max_dist, _, _)) = guard.peek() {
                        if candidate.0 < max_dist {
                            guard.pop();
                            guard.push(candidate);
                        }
                    }
                }
            }
        }
    });

    // Extract pairs (discard distances)
    heap.into_inner()
        .unwrap()
        .into_iter()
        .map(|(_, i, j)| (i, j))
        .collect()
}

/// Count the size of each connected component
fn count_component_sizes(uf: &mut UnionFind, n: usize) -> Vec<usize> {
    let mut component_counts: HashMap<usize, usize> = HashMap::new();

    for i in 0..n {
        let root = uf.find(i);
        *component_counts.entry(root).or_insert(0) += 1;
    }

    component_counts.into_values().collect()
}

/// Calculate product of the m largest values in the vector
fn product_of_largest(mut sizes: Vec<usize>, m: usize) -> usize {
    if m == 0 || sizes.is_empty() {
        return 1;
    }

    // Sort in descending order
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    // Multiply the first min(m, sizes.len()) elements
    sizes.iter().take(m).product()
}

fn part1(n: usize, m: usize, inputs: &[Point]) -> usize {
    // Handle edge cases
    if inputs.is_empty() || m == 0 {
        return 1;
    }

    // Find n closest pairs globally
    let pairs = find_n_closest_pairs(inputs, n);

    // Build Union-Find and connect pairs
    let mut uf = UnionFind::new(inputs.len());
    for (i, j) in pairs {
        uf.union(i, j);
    }

    // Count component sizes
    let sizes = count_component_sizes(&mut uf, inputs.len());

    // Return product of m largest
    product_of_largest(sizes, m)
}

fn part2(inputs: &[Point]) -> usize {
    if inputs.len() < 2 {
        return 0;
    }

    // Generate all edges with distances (parallelized)
    let mut edges: Vec<(i64, usize, usize)> = (0..inputs.len())
        .into_par_iter()
        .flat_map(|i| {
            ((i + 1)..inputs.len())
                .map(|j| (squared_distance(&inputs[i], &inputs[j]), i, j))
                .collect::<Vec<_>>()
        })
        .collect();

    // Sort edges by distance (Kruskal's algorithm) - parallel sort
    edges.par_sort_unstable_by_key(|&(dist, _, _)| dist);

    // Use Union-Find to build MST
    let mut uf = UnionFind::new(inputs.len());
    let mut last_edge: Option<(usize, usize)> = None;
    let mut edges_added = 0;
    let target_edges = inputs.len() - 1;

    for (_, i, j) in edges {
        // Check if adding this edge would create a cycle
        if uf.find(i) != uf.find(j) {
            uf.union(i, j);
            last_edge = Some((i, j));
            edges_added += 1;

            // Stop when we have a spanning tree (n-1 edges for n nodes)
            if edges_added == target_edges {
                break;
            }
        }
    }

    // Return product of x coordinates of the last edge
    if let Some((i, j)) = last_edge {
        (inputs[i].x as usize) * (inputs[j].x as usize)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create a Point
    fn point(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }

    #[test]
    fn test_small_example_n1_m1() {
        let points = vec![point(0, 0, 0), point(2, 2, 2), point(2, 3, 2)];
        // n=1: Connect the closest pair (2,2,2)-(2,3,2)
        // Components: [1], [2]
        // m=1: largest component has size 2
        assert_eq!(part1(1, 1, &points), 2);
    }

    #[test]
    fn test_small_example_n1_m2() {
        let points = vec![point(0, 0, 0), point(2, 2, 2), point(2, 3, 2)];
        // n=1: Connect the closest pair (2,2,2)-(2,3,2)
        // Components: [1], [2]
        // m=2: product of two largest = 1 * 2 = 2
        assert_eq!(part1(1, 2, &points), 2);
    }

    #[test]
    fn test_large_example_n3_m1() {
        let points = vec![
            point(162, 817, 812),
            point(57, 618, 57),
            point(906, 360, 560),
            point(592, 479, 940),
            point(352, 342, 300),
            point(466, 668, 158),
            point(542, 29, 236),
            point(431, 825, 988),
            point(739, 650, 466),
            point(52, 470, 668),
            point(216, 146, 977),
            point(117, 168, 530),
            point(805, 96, 715),
            point(346, 949, 466),
            point(970, 615, 88),
            point(941, 993, 340),
            point(862, 61, 35),
            point(984, 92, 344),
            point(425, 690, 689),
            point(805, 96, 715),
        ];
        // n=3: Creates one component of size 3, one of size 2, and rest of size 1
        // m=1: largest component = 3
        assert_eq!(part1(3, 1, &points), 3);
    }

    #[test]
    fn test_large_example_n3_m2() {
        let points = vec![
            point(162, 817, 812),
            point(57, 618, 57),
            point(906, 360, 560),
            point(592, 479, 940),
            point(352, 342, 300),
            point(466, 668, 158),
            point(542, 29, 236),
            point(431, 825, 988),
            point(739, 650, 466),
            point(52, 470, 668),
            point(216, 146, 977),
            point(117, 168, 530),
            point(805, 96, 715),
            point(346, 949, 466),
            point(970, 615, 88),
            point(941, 993, 340),
            point(862, 61, 35),
            point(984, 92, 344),
            point(425, 690, 689),
            point(805, 96, 715),
        ];
        // n=3: Creates one component of size 3, one of size 2
        // m=2: product = 3 * 2 = 6
        assert_eq!(part1(3, 2, &points), 6);
    }

    #[test]
    fn test_empty_input() {
        let points: Vec<Point> = vec![];
        assert_eq!(part1(10, 3, &points), 1);
    }

    #[test]
    fn test_single_point() {
        let points = vec![point(5, 5, 5)];
        // Single point, one component of size 1
        assert_eq!(part1(10, 1, &points), 1);
    }

    #[test]
    fn test_m_zero() {
        let points = vec![point(0, 0, 0), point(1, 1, 1)];
        // m=0 means empty product = 1
        assert_eq!(part1(1, 0, &points), 1);
    }

    #[test]
    fn test_n_zero() {
        let points = vec![point(0, 0, 0), point(1, 1, 1), point(2, 2, 2)];
        // n=0: no connections, each point is its own component
        // m=2: multiply two largest = 1 * 1 = 1
        assert_eq!(part1(0, 2, &points), 1);
    }

    #[test]
    fn test_fully_connected() {
        let points = vec![point(0, 0, 0), point(1, 1, 1), point(2, 2, 2)];
        // n=10 exceeds total pairs, all points connected
        // One component of size 3
        assert_eq!(part1(10, 1, &points), 3);
    }

    #[test]
    fn test_m_exceeds_components() {
        let points = vec![point(0, 0, 0), point(1, 1, 1)];
        // n=0: two components of size 1 each
        // m=5 exceeds available components, multiply all = 1 * 1 = 1
        assert_eq!(part1(0, 5, &points), 1);
    }

    #[test]
    fn test_negative_coordinates() {
        let points = vec![
            point(-10, -20, -30),
            point(-11, -21, -31),
            point(100, 100, 100),
        ];
        // n=1: closest pair is the two negative points
        // Components: [2], [1]
        // m=1: largest = 2
        assert_eq!(part1(1, 1, &points), 2);
    }

    #[test]
    fn test_identical_points() {
        let points = vec![point(5, 5, 5), point(5, 5, 5), point(10, 10, 10)];
        // n=1: closest pair is the two identical points (distance 0)
        // Components: [2], [1]
        // m=1: largest = 2
        assert_eq!(part1(1, 1, &points), 2);
    }

    #[test]
    fn test_linear_chain() {
        let points = vec![
            point(0, 0, 0),
            point(1, 0, 0),
            point(2, 0, 0),
            point(3, 0, 0),
        ];
        // n=3: connects (0,1), (1,2), (2,3) - all connected in a chain
        // One component of size 4
        assert_eq!(part1(3, 1, &points), 4);
    }

    #[test]
    fn test_multiple_equal_components() {
        let points = vec![
            point(0, 0, 0),
            point(1, 0, 0),
            point(10, 0, 0),
            point(11, 0, 0),
        ];
        // n=2: connects (0,1) and (10,11)
        // Two components of size 2 each
        // m=2: product = 2 * 2 = 4
        assert_eq!(part1(2, 2, &points), 4);
    }

    #[test]
    fn test_squared_distance_calculation() {
        let p1 = point(1, 2, 3);
        let p2 = point(4, 6, 8);
        // dx=3, dy=4, dz=5
        // squared = 9 + 16 + 25 = 50
        assert_eq!(squared_distance(&p1, &p2), 50);
    }

    #[test]
    fn test_product_of_largest_basic() {
        assert_eq!(product_of_largest(vec![5, 3, 8, 1], 2), 8 * 5);
        assert_eq!(product_of_largest(vec![5, 3, 8, 1], 1), 8);
        assert_eq!(product_of_largest(vec![5, 3, 8, 1], 4), 5 * 3 * 8 * 1);
    }

    #[test]
    fn test_product_of_largest_edge_cases() {
        assert_eq!(product_of_largest(vec![], 3), 1);
        assert_eq!(product_of_largest(vec![5], 0), 1);
        assert_eq!(product_of_largest(vec![5, 3], 5), 5 * 3);
    }

    // Part 2 tests

    #[test]
    fn test_part2_small_example() {
        let points = vec![point(1, 1, 1), point(2, 3, 4), point(3, 5, 6)];
        // First edge: (2,3,4)-(3,5,6) - distance 9
        // Second edge: (1,1,1)-(2,3,4) - distance 14 (this is the final edge)
        // Product: 1 * 2 = 2
        assert_eq!(part2(&points), 2);
    }

    #[test]
    fn test_part2_large_example() {
        let points = vec![
            point(162, 817, 812),
            point(57, 618, 57),
            point(906, 360, 560),
            point(592, 479, 940),
            point(352, 342, 300),
            point(466, 668, 158),
            point(542, 29, 236),
            point(431, 825, 988),
            point(739, 650, 466),
            point(52, 470, 668),
            point(216, 146, 977),
            point(117, 168, 530),
            point(805, 96, 715),
            point(346, 949, 466),
            point(970, 615, 88),
            point(941, 993, 340),
            point(862, 61, 35),
            point(984, 92, 344),
            point(425, 690, 689),
        ];
        // Final edge: (216,146,977)-(117,168,530)
        // Product: 216 * 117 = 25272
        assert_eq!(part2(&points), 25272);
    }

    #[test]
    fn test_part2_empty_input() {
        let points: Vec<Point> = vec![];
        assert_eq!(part2(&points), 0);
    }

    #[test]
    fn test_part2_single_point() {
        let points = vec![point(5, 5, 5)];
        assert_eq!(part2(&points), 0);
    }

    #[test]
    fn test_part2_two_points() {
        let points = vec![point(3, 1, 1), point(7, 2, 2)];
        // Only one edge: (3,1,1)-(7,2,2)
        // Product: 3 * 7 = 21
        assert_eq!(part2(&points), 21);
    }

    #[test]
    fn test_part2_linear_arrangement() {
        let points = vec![
            point(1, 0, 0),
            point(2, 0, 0),
            point(3, 0, 0),
            point(10, 0, 0),
        ];
        // Edges in order: (1,2), (2,3), (3,10)
        // Last edge connects (3,0,0) to (10,0,0)
        // Product: 3 * 10 = 30
        assert_eq!(part2(&points), 30);
    }

    #[test]
    fn test_part2_varied_distances() {
        let points = vec![point(5, 0, 0), point(10, 0, 0), point(3, 0, 0)];
        // Edges in order by distance:
        // (5,3): distance 4, (3,10): distance 49, (5,10): distance 25
        // MST: (5,3) first, then (5,10)
        // Last edge: (5,10), product: 5 * 10 = 50
        assert_eq!(part2(&points), 50);
    }

    #[test]
    fn test_part2_square_configuration() {
        let points = vec![
            point(0, 0, 0),
            point(1, 0, 0),
            point(0, 1, 0),
            point(1, 1, 0),
        ];
        // All edges have distance 1 or sqrt(2)
        // Edges of distance 1: (0,1), (0,2), (1,3), (2,3)
        // MST would pick 3 edges, last one depends on ordering
        let result = part2(&points);
        assert!(result > 0);
    }

    #[test]
    fn test_part2_collinear_points() {
        let points = vec![
            point(5, 0, 0),
            point(10, 0, 0),
            point(15, 0, 0),
            point(20, 0, 0),
        ];
        // Connect adjacent points
        // Last edge should be (15,0,0)-(20,0,0)
        // Product: 15 * 20 = 300
        assert_eq!(part2(&points), 300);
    }
}
