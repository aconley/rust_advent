use rayon::prelude::*;
use rust_advent::Point;
use std::collections::BinaryHeap;

fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_points("08")?;
    println!("Part 1: {}", part1(1000, 3, &inputs));
    println!("Part 2: {}", part2(&inputs));
    Ok(())
}

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Dsu {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
            true
        } else {
            false
        }
    }
}

fn part1(n: usize, m: usize, inputs: &[Point]) -> usize {
    if n == 0 {
        return calculate_result(m, inputs.len(), &Dsu::new(inputs.len()));
    }

    let num_points = inputs.len();
    let max_edges = if num_points > 0 {
        num_points.saturating_mul(num_points - 1) / 2
    } else {
        0
    };
    let heap_capacity = n.min(max_edges).saturating_add(1);

    let final_heap = (0..num_points)
        .into_par_iter()
        .fold(
            || BinaryHeap::with_capacity(heap_capacity),
            |mut local_heap, i| {
                let p1 = &inputs[i];
                for (j, p2) in inputs.iter().enumerate().skip(i + 1) {
                    let dist_sq = (p1.x as i64 - p2.x as i64).pow(2)
                        + (p1.y as i64 - p2.y as i64).pow(2)
                        + (p1.z as i64 - p2.z as i64).pow(2);

                    if local_heap.len() < n {
                        local_heap.push((dist_sq, i, j));
                    } else if let Some(&(max_dist, _, _)) = local_heap.peek()
                        && dist_sq < max_dist
                    {
                        local_heap.pop();
                        local_heap.push((dist_sq, i, j));
                    }
                }
                local_heap
            },
        )
        .reduce(
            BinaryHeap::new,
            |mut h1, mut h2| {
                if h1.is_empty() {
                    return h2;
                }
                if h2.is_empty() {
                    return h1;
                }
                while let Some(item) = h2.pop() {
                    if h1.len() < n {
                        h1.push(item);
                    } else if let Some(&max) = h1.peek()
                        && item.0 < max.0
                    {
                        h1.pop();
                        h1.push(item);
                    }
                }
                h1
            },
        );

    let mut dsu = Dsu::new(num_points);
    for (_, u, v) in final_heap {
        dsu.union(u, v);
    }

    calculate_result(m, num_points, &dsu)
}

fn calculate_result(m: usize, num_points: usize, dsu: &Dsu) -> usize {
    let mut component_sizes: Vec<usize> = (0..num_points)
        .filter(|&i| dsu.parent[i] == i)
        .map(|i| dsu.size[i])
        .collect();

    component_sizes.sort_unstable_by(|a, b| b.cmp(a));
    component_sizes.iter().take(m).product()
}

#[derive(Clone, Copy, Default)]
struct Edge {
    u: usize,
    v: usize,
    dist_sq: i64,
}

fn radix_sort_edges_safe(edges: &mut Vec<Edge>) {
    let mut buffer = vec![Edge::default(); edges.len()];
    let mut src = edges;
    let mut dst = &mut buffer;

    for byte in 0..8 {
        let mut counts = [0usize; 256];
        for e in src.iter() {
            let b = ((e.dist_sq as u64) >> (byte * 8)) as u8;
            counts[b as usize] += 1;
        }
        let mut pos = [0usize; 256];
        let mut total = 0;
        for i in 0..256 {
            pos[i] = total;
            total += counts[i];
        }
        for e in src.iter() {
            let b = ((e.dist_sq as u64) >> (byte * 8)) as u8;
            dst[pos[b as usize]] = *e;
            pos[b as usize] += 1;
        }
        std::mem::swap(&mut src, &mut dst);
    }
}

fn part2(inputs: &[Point]) -> usize {
    let num_points = inputs.len();
    if num_points < 2 {
        return 0;
    }

    let mut edges: Vec<Edge> = (0..num_points)
        .into_par_iter()
        .flat_map(|i| {
            let p1 = &inputs[i];
            (i + 1..num_points).into_par_iter().map(move |j| {
                let p2 = &inputs[j];
                let dist_sq = (p1.x as i64 - p2.x as i64).pow(2)
                    + (p1.y as i64 - p2.y as i64).pow(2)
                    + (p1.z as i64 - p2.z as i64).pow(2);
                Edge {
                    u: i,
                    v: j,
                    dist_sq,
                }
            })
        })
        .collect();

    radix_sort_edges_safe(&mut edges);

    let mut dsu = Dsu::new(num_points);
    let mut components = num_points;

    for edge in edges {
        if dsu.union(edge.u, edge.v) {
            components -= 1;
            if components == 1 {
                return (inputs[edge.u].x as usize) * (inputs[edge.v].x as usize);
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_small_example() {
        let inputs = vec![
            Point { x: 0, y: 0, z: 0 },
            Point { x: 2, y: 2, z: 2 },
            Point { x: 2, y: 3, z: 2 },
        ];
        assert_eq!(part1(1, 1, &inputs), 2);
        assert_eq!(part1(1, 2, &inputs), 2);
    }

    #[test]
    fn test_part1_large_example() {
        let inputs = vec![
            Point {
                x: 162,
                y: 817,
                z: 812,
            },
            Point {
                x: 57,
                y: 618,
                z: 57,
            },
            Point {
                x: 906,
                y: 360,
                z: 560,
            },
            Point {
                x: 592,
                y: 479,
                z: 940,
            },
            Point {
                x: 352,
                y: 342,
                z: 300,
            },
            Point {
                x: 466,
                y: 668,
                z: 158,
            },
            Point {
                x: 542,
                y: 29,
                z: 236,
            },
            Point {
                x: 431,
                y: 825,
                z: 988,
            },
            Point {
                x: 739,
                y: 650,
                z: 466,
            },
            Point {
                x: 52,
                y: 470,
                z: 668,
            },
            Point {
                x: 216,
                y: 146,
                z: 977,
            },
            Point {
                x: 819,
                y: 987,
                z: 18,
            },
            Point {
                x: 117,
                y: 168,
                z: 530,
            },
            Point {
                x: 805,
                y: 96,
                z: 715,
            },
            Point {
                x: 346,
                y: 949,
                z: 466,
            },
            Point {
                x: 970,
                y: 615,
                z: 88,
            },
            Point {
                x: 941,
                y: 993,
                z: 340,
            },
            Point {
                x: 862,
                y: 61,
                z: 35,
            },
            Point {
                x: 984,
                y: 92,
                z: 344,
            },
            Point {
                x: 425,
                y: 690,
                z: 689,
            },
        ];
        assert_eq!(part1(3, 1, &inputs), 3);
        assert_eq!(part1(3, 2, &inputs), 6);
    }

    #[test]
    fn test_part2_small_example() {
        let inputs = vec![
            Point { x: 1, y: 1, z: 1 },
            Point { x: 2, y: 3, z: 4 },
            Point { x: 3, y: 5, z: 6 },
        ];
        assert_eq!(part2(&inputs), 2);
    }

    #[test]
    fn test_part1_n_greater_than_edges() {
        let inputs = vec![
            Point { x: 0, y: 0, z: 0 },
            Point { x: 2, y: 2, z: 2 },
            Point { x: 5, y: 5, z: 5 },
        ];
        assert_eq!(part1(10, 1, &inputs), 3);
    }

    #[test]
    fn test_part1_m_zero() {
        let inputs = vec![Point { x: 0, y: 0, z: 0 }, Point { x: 1, y: 0, z: 0 }];
        assert_eq!(part1(0, 0, &inputs), 1);
        assert_eq!(part1(1, 0, &inputs), 1);
    }

    #[test]
    fn test_part1_n_max_usize() {
        let inputs = vec![
            Point { x: 0, y: 0, z: 0 },
            Point { x: 1, y: 0, z: 0 },
            Point { x: 2, y: 0, z: 0 },
        ];
        assert_eq!(part1(usize::MAX, 1, &inputs), 3);
    }

    #[test]
    fn test_part1_m_max_usize() {
        let inputs = vec![
            Point { x: 0, y: 0, z: 0 },
            Point { x: 10, y: 0, z: 0 },
            Point { x: 20, y: 0, z: 0 },
        ];
        assert_eq!(part1(0, usize::MAX, &inputs), 1);
    }

    #[test]
    fn test_part1_single_point() {
        let inputs = vec![Point { x: 0, y: 0, z: 0 }];
        assert_eq!(part1(10, 1, &inputs), 1);
        assert_eq!(part1(0, 5, &inputs), 1);
    }

    #[test]
    fn test_part2_large_example() {
        let inputs = vec![
            Point {
                x: 162,
                y: 817,
                z: 812,
            },
            Point {
                x: 57,
                y: 618,
                z: 57,
            },
            Point {
                x: 906,
                y: 360,
                z: 560,
            },
            Point {
                x: 592,
                y: 479,
                z: 940,
            },
            Point {
                x: 352,
                y: 342,
                z: 300,
            },
            Point {
                x: 466,
                y: 668,
                z: 158,
            },
            Point {
                x: 542,
                y: 29,
                z: 236,
            },
            Point {
                x: 431,
                y: 825,
                z: 988,
            },
            Point {
                x: 739,
                y: 650,
                z: 466,
            },
            Point {
                x: 52,
                y: 470,
                z: 668,
            },
            Point {
                x: 216,
                y: 146,
                z: 977,
            },
            Point {
                x: 819,
                y: 987,
                z: 18,
            },
            Point {
                x: 117,
                y: 168,
                z: 530,
            },
            Point {
                x: 805,
                y: 96,
                z: 715,
            },
            Point {
                x: 346,
                y: 949,
                z: 466,
            },
            Point {
                x: 970,
                y: 615,
                z: 88,
            },
            Point {
                x: 941,
                y: 993,
                z: 340,
            },
            Point {
                x: 862,
                y: 61,
                z: 35,
            },
            Point {
                x: 984,
                y: 92,
                z: 344,
            },
            Point {
                x: 425,
                y: 690,
                z: 689,
            },
        ];
        assert_eq!(part2(&inputs), 25272);
    }

    #[test]
    fn test_part1_duplicate_points() {
        let inputs = vec![
            Point { x: 0, y: 0, z: 0 },
            Point { x: 0, y: 0, z: 0 },
            Point { x: 10, y: 0, z: 0 },
        ];
        assert_eq!(part1(1, 1, &inputs), 2);
    }

    #[test]
    fn test_part1_zero_n() {
        let inputs = vec![
            Point { x: 0, y: 0, z: 0 },
            Point { x: 2, y: 2, z: 2 },
            Point { x: 5, y: 5, z: 5 },
        ];
        assert_eq!(part1(0, 1, &inputs), 1);
        assert_eq!(part1(0, 3, &inputs), 1);
    }

    #[test]
    fn test_part1_insufficient_m() {
        let inputs = vec![Point { x: 0, y: 0, z: 0 }, Point { x: 10, y: 0, z: 0 }];
        assert_eq!(part1(0, 5, &inputs), 1);
    }

    #[test]
    fn test_part2_two_clusters() {
        let inputs = vec![
            Point { x: 0, y: 0, z: 0 },
            Point { x: 1, y: 0, z: 0 },
            Point { x: 100, y: 0, z: 0 },
            Point { x: 101, y: 0, z: 0 },
        ];
        assert_eq!(part2(&inputs), 100);
    }

    #[test]
    fn test_part2_collinear() {
        let inputs = vec![
            Point { x: 10, y: 0, z: 0 },
            Point { x: 20, y: 0, z: 0 },
            Point { x: 40, y: 0, z: 0 },
        ];
        assert_eq!(part2(&inputs), 800);
    }
}