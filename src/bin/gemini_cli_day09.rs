use rust_advent::Point2d;

fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_points2d("09")?;
    println!("Part 1: {}", part1(&inputs));
    match part2(&inputs) {
        Ok(res) => println!("Part 2: {}", res),
        Err(e) => eprintln!("Part 2 Error: {}", e),
    }
    Ok(())
}

fn cross_product(o: &Point2d, a: &Point2d, b: &Point2d) -> i64 {
    (a.x as i64 - o.x as i64) * (b.y as i64 - o.y as i64)
        - (a.y as i64 - o.y as i64) * (b.x as i64 - o.x as i64)
}

/// Computes the convex hull of the given points using the Monotone Chain algorithm.
/// Includes collinear points on the hull edges.
///
/// Why collinear points?
/// Unlike Euclidean distance (maximized at vertices), the area of an axis-aligned rectangle
/// formed by points on a line segment is a quadratic function.
/// If the segment has a negative slope (e.g., x increases, y decreases), the area function
/// opens downwards, meaning the maximum can occur at a point *inside* the segment,
/// not just at the endpoints.
fn get_convex_hull(points: &[Point2d]) -> Vec<Point2d> {
    if points.len() <= 2 {
        return points.iter().map(|p| Point2d { x: p.x, y: p.y }).collect();
    }

    let mut sorted_points: Vec<&Point2d> = points.iter().collect();
    sorted_points.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));
    sorted_points.dedup();

    // Build lower hull
    let mut lower = Vec::new();
    for &p in &sorted_points {
        while lower.len() >= 2 {
            let last = &lower[lower.len() - 1];
            let second_last = &lower[lower.len() - 2];
            // Remove point if it creates a clockwise turn (concave).
            // Keep if cross_product >= 0 (counter-clockwise or collinear).
            if cross_product(second_last, last, p) < 0 {
                lower.pop();
            } else {
                break;
            }
        }
        lower.push(Point2d { x: p.x, y: p.y });
    }

    // Build upper hull
    let mut upper = Vec::new();
    for &p in sorted_points.iter().rev() {
        while upper.len() >= 2 {
            let last = &upper[upper.len() - 1];
            let second_last = &upper[upper.len() - 2];
            if cross_product(second_last, last, p) < 0 {
                upper.pop();
            } else {
                break;
            }
        }
        upper.push(Point2d { x: p.x, y: p.y });
    }

    // Remove duplicates (start/end points)
    if !lower.is_empty() {
        lower.pop();
    }
    if !upper.is_empty() {
        upper.pop();
    }

    let mut hull = lower;
    hull.extend(upper);
    hull
}

fn part1(inputs: &[Point2d]) -> usize {
    if inputs.len() < 2 {
        return 0;
    }

    // Optimization: The pair of points forming the largest rectangle
    // must lie on the Convex Hull of the set of points.
    let hull = get_convex_hull(inputs);

    let mut max_area: u64 = 0;
    for i in 0..hull.len() {
        for j in i + 1..hull.len() {
            let p1 = &hull[i];
            let p2 = &hull[j];
            // Use u64 for calculation to prevent overflow on 32-bit systems
            // or with very large coordinates.
            // MUST cast to i64 before subtraction to avoid i32 overflow if points are far apart.
            let width = (p1.x as i64 - p2.x as i64).abs() as u64 + 1;
            let height = (p1.y as i64 - p2.y as i64).abs() as u64 + 1;
            let area = width * height;
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area as usize
}

fn part2(inputs: &[Point2d]) -> Result<usize, String> {
    if inputs.len() < 4 {
        return Ok(0);
    }

    // 1. Collect unique sorted coordinates (Coordinate Compression)
    let mut xs: Vec<i32> = inputs.iter().map(|p| p.x).collect();
    let mut ys: Vec<i32> = inputs.iter().map(|p| p.y).collect();
    xs.sort();
    xs.dedup();
    ys.sort();
    ys.dedup();

    // Map coordinate to index
    fn get_idx(val: i32, coords: &[i32]) -> usize {
        coords.binary_search(&val).unwrap()
    }

    let m = xs.len();
    let n = ys.len();
    if m < 2 || n < 2 {
        // Degenerate grid (line or point)
        return Ok(0);
    }

    // 2. Identify Vertical Edges of the Polygon
    // Store as (x, y_min, y_max). Vertices connect inputs[i] -> inputs[i+1].
    struct VEdge {
        x: i32,
        y_min: i32,
        y_max: i32,
    }
    let mut v_edges = Vec::new();
    let len = inputs.len();
    for i in 0..len {
        let p1 = &inputs[i];
        let p2 = &inputs[(i + 1) % len];
        if p1.x == p2.x {
            // Vertical edge
            let y_min = std::cmp::min(p1.y, p2.y);
            let y_max = std::cmp::max(p1.y, p2.y);
            v_edges.push(VEdge {
                x: p1.x,
                y_min,
                y_max,
            });
        } else if p1.y != p2.y {
            // Not rectilinear!
            return Err(format!(
                "Input polygon is not rectilinear: segment {:?} -> {:?}",
                p1, p2
            ));
        }
    }

    // 3. Build Grid Status (Sweep Line)
    // grid[x_idx][y_idx] is true if the cell [xs[x], xs[x+1]] x [ys[y], ys[y+1]] is INSIDE.
    // Dimensions: (m-1) x (n-1)
    let mut grid = vec![vec![0u8; n - 1]; m - 1];

    for j in 0..n - 1 {
        // Current Y band: ys[j] to ys[j+1]
        let y_start = ys[j];
        let y_end = ys[j + 1];

        // Find relevant edges covering this band.
        // A vertical edge covers this band if edge.y_min <= y_start AND edge.y_max >= y_end.
        // Since y coordinates are from the set of vertex coordinates, edges start/end exactly at band boundaries.
        let mut row_edges: Vec<&VEdge> = v_edges
            .iter()
            .filter(|e| e.y_min <= y_start && e.y_max >= y_end)
            .collect();
        row_edges.sort_by_key(|e| e.x);

        let mut parity = 0; // 0: outside, 1: inside
        let mut edge_idx = 0;

        for i in 0..m - 1 {
            // Current X cell: xs[i] to xs[i+1]
            // We need to check how many edges are to the LEFT of this cell.
            // Since edges are on grid lines, we process edges at x <= xs[i].

            while edge_idx < row_edges.len() && row_edges[edge_idx].x <= xs[i] {
                parity ^= 1;
                edge_idx += 1;
            }
            grid[i][j] = parity;
        }
    }

    // 4. Build 2D Prefix Sums
    // prefix[i][j] stores sum of grid[0..i][0..j]
    // Dimensions: m x n (padded with 0 row/col for convenience)
    let mut prefix = vec![vec![0u32; n]; m];
    for i in 0..m - 1 {
        for j in 0..n - 1 {
            prefix[i + 1][j + 1] =
                prefix[i][j + 1] + prefix[i + 1][j] - prefix[i][j] + (grid[i][j] as u32);
        }
    }

    // Helper to count valid cells in range [ix1, ix2) x [iy1, iy2)
    let count_valid = |ix1: usize, iy1: usize, ix2: usize, iy2: usize| -> u32 {
        if ix1 >= ix2 || iy1 >= iy2 {
            return 0;
        }
        let term_pos = prefix[ix2][iy2] + prefix[ix1][iy1];
        let term_neg = prefix[ix1][iy2] + prefix[ix2][iy1];
        term_pos - term_neg
    };

    // 5. Check all pairs
    let mut max_area: u64 = 0;
    for i in 0..len {
        for k in i + 1..len {
            let p1 = &inputs[i];
            let p2 = &inputs[k];

            // Map to grid indices
            let idx_x1 = get_idx(p1.x, &xs);
            let idx_y1 = get_idx(p1.y, &ys);
            let idx_x2 = get_idx(p2.x, &xs);
            let idx_y2 = get_idx(p2.y, &ys);

            let ix_min = std::cmp::min(idx_x1, idx_x2);
            let ix_max = std::cmp::max(idx_x1, idx_x2);
            let iy_min = std::cmp::min(idx_y1, idx_y2);
            let iy_max = std::cmp::max(idx_y1, idx_y2);

            // Expected number of cells
            let total_cells = ((ix_max - ix_min) * (iy_max - iy_min)) as u32;

            if total_cells > 0 {
                if count_valid(ix_min, iy_min, ix_max, iy_max) == total_cells {
                    let width = (p1.x as i64 - p2.x as i64).abs() as u64 + 1;
                    let height = (p1.y as i64 - p2.y as i64).abs() as u64 + 1;
                    max_area = std::cmp::max(max_area, width * height);
                }
            } else {
                // For degenerate rectangles (lines/points), a single point is always valid with area 1.
                max_area = std::cmp::max(max_area, 1);
            }
        }
    }

    Ok(max_area as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Part 1 Tests ---

    #[test]
    fn test_example() {
        let inputs = vec![
            Point2d { x: 7, y: 1 },
            Point2d { x: 11, y: 1 },
            Point2d { x: 11, y: 7 },
            Point2d { x: 9, y: 7 },
            Point2d { x: 9, y: 5 },
            Point2d { x: 2, y: 5 },
            Point2d { x: 2, y: 3 },
            Point2d { x: 7, y: 3 },
        ];
        assert_eq!(part1(&inputs), 50);
    }

    #[test]
    fn test_small_input() {
        let inputs = vec![Point2d { x: 0, y: 0 }, Point2d { x: 2, y: 2 }];
        assert_eq!(part1(&inputs), 9);
    }

    #[test]
    fn test_collinear_optimal() {
        let inputs = vec![
            Point2d { x: 0, y: 10 },
            Point2d { x: 10, y: 0 },
            Point2d { x: 5, y: 5 },
            Point2d { x: 20, y: 20 },
        ];
        assert_eq!(part1(&inputs), 256);
    }

    #[test]
    fn test_single_point() {
        let inputs = vec![Point2d { x: 5, y: 5 }];
        assert_eq!(part1(&inputs), 0);
    }

    #[test]
    fn test_negative_coordinates() {
        let inputs = vec![
            Point2d { x: -5, y: -5 },
            Point2d { x: -2, y: -2 },
            Point2d { x: 5, y: 5 },
        ];
        assert_eq!(part1(&inputs), 121);
    }

    #[test]
    fn test_vertical_line() {
        let inputs = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 0, y: 10 },
            Point2d { x: 0, y: 5 },
        ];
        assert_eq!(part1(&inputs), 11);
    }

    #[test]
    fn test_dense_grid() {
        let mut inputs = Vec::new();
        for x in 0..=2 {
            for y in 0..=2 {
                inputs.push(Point2d { x, y });
            }
        }
        assert_eq!(part1(&inputs), 9);
    }

    #[test]
    fn test_large_coordinates() {
        let inputs = vec![
            Point2d {
                x: -1_000_000_000,
                y: -1_000_000_000,
            },
            Point2d {
                x: 1_000_000_000,
                y: 1_000_000_000,
            },
        ];
        let dim = 2_000_000_001u64;
        let expected = dim * dim;
        assert_eq!(part1(&inputs) as u64, expected);
    }

    // --- Part 2 Tests ---

    #[test]
    fn test_part2_example() {
        let inputs = vec![
            Point2d { x: 7, y: 1 },
            Point2d { x: 11, y: 1 },
            Point2d { x: 11, y: 7 },
            Point2d { x: 9, y: 7 },
            Point2d { x: 9, y: 5 },
            Point2d { x: 2, y: 5 },
            Point2d { x: 2, y: 3 },
            Point2d { x: 7, y: 3 },
        ];
        assert_eq!(part2(&inputs).unwrap(), 24);
    }

    #[test]
    fn test_part2_simple_box() {
        let inputs = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 10, y: 0 },
            Point2d { x: 10, y: 10 },
            Point2d { x: 0, y: 10 },
        ];
        // Largest is (0,0) to (10,10) -> 11*11 = 121
        assert_eq!(part2(&inputs).unwrap(), 121);
    }

    #[test]
    fn test_part2_u_shape() {
        // U shape:
        // (0,3)--(1,3)   (2,3)--(3,3)
        //   |      |       |      |
        // (0,0)----(1,0)---(2,0)--(3,0)
        let inputs = vec![
            Point2d { x: 0, y: 3 },
            Point2d { x: 0, y: 0 },
            Point2d { x: 3, y: 0 },
            Point2d { x: 3, y: 3 },
            Point2d { x: 2, y: 3 },
            Point2d { x: 2, y: 1 },
            Point2d { x: 1, y: 1 },
            Point2d { x: 1, y: 3 },
        ];

        // Potential valid rectangles include the "legs" ([0,1]x[0,3] and [2,3]x[0,3])
        // with area 2*4=8. The full width [0,3]x[0,3] is invalid as it contains the hole.
        assert_eq!(part2(&inputs).unwrap(), 8);
    }

    #[test]
    fn test_part2_dumbbell() {
        // Two 3x3 boxes connected by a 1x1 pipe.
        // Vertices:
        // (0,0)->(3,0)->(3,1)->(6,1)->(6,0)->(9,0)->(9,3)->(6,3)->(6,2)->(3,2)->(3,3)->(0,3)->(0,0)
        let inputs = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 3, y: 0 },
            Point2d { x: 3, y: 1 },
            Point2d { x: 6, y: 1 },
            Point2d { x: 6, y: 0 },
            Point2d { x: 9, y: 0 },
            Point2d { x: 9, y: 3 },
            Point2d { x: 6, y: 3 },
            Point2d { x: 6, y: 2 },
            Point2d { x: 3, y: 2 },
            Point2d { x: 3, y: 3 },
            Point2d { x: 0, y: 3 },
        ];
        // Max area valid with *vertices*:
        // Left Box: [0,3]x[0,3] -> Vertices (0,0) and (3,3) exist. Area 4x4=16.
        // Right Box: [6,9]x[0,3] -> Vertices (6,3) and (9,0) exist. Area 4x4=16.
        // The long thin strip [0,9]x[1,2] (Area 20) is visually inside,
        // BUT requires corners like (0,1) or (9,2) which are NOT in the input list.
        assert_eq!(part2(&inputs).unwrap(), 16);
    }

    #[test]
    fn test_part2_c_shape() {
        // C-shape
        // (0,3)--(3,3)
        //   |      |
        // (0,0)--(3,0)
        //   |      |
        // (0,1)--(2,1)
        //   |      |
        // (0,2)--(2,2)
        // (0,0)->(3,0) H -> (3,1) V -> (1,1) H -> (1,2) V -> (3,2) H -> (3,3) V -> (0,3) H -> (0,0) V
        let inputs = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 3, y: 0 },
            Point2d { x: 3, y: 1 },
            Point2d { x: 1, y: 1 },
            Point2d { x: 1, y: 2 },
            Point2d { x: 3, y: 2 },
            Point2d { x: 3, y: 3 },
            Point2d { x: 0, y: 3 },
        ];

        // Bounding Box: [0,3]x[0,3] -> 4x4=16. Invalid (contains hole).
        // Left Bar: [0,1]x[0,3] -> 2x4=8.
        // Top Bar: [0,3]x[2,3] -> 4x2=8.
        // Bottom Bar: [0,3]x[0,1] -> 4x2=8.
        assert_eq!(part2(&inputs).unwrap(), 8);
    }

    #[test]
    fn test_part2_spiral() {
        // 5x5 "Snake" / C-facing-left with hole.
        let inputs = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 5, y: 0 },
            Point2d { x: 5, y: 1 },
            Point2d { x: 1, y: 1 },
            Point2d { x: 1, y: 2 },
            Point2d { x: 5, y: 2 },
            Point2d { x: 5, y: 3 },
            Point2d { x: 0, y: 3 },
        ];

        // Largest rectangles using *vertices*:
        // Top bar [0,5]x[2,3] -> Vertices (0,3) and (5,2) exist. Area 6x2 = 12.
        // Bottom bar [0,5]x[0,1] -> Vertices (0,0) and (5,1) exist. Area 6x2 = 12.
        //
        // The vertical block [1,5]x[0,3] (Area 20) is valid space,
        // but requires corners like (1,0) or (1,3) which are NOT vertices.
        // Available x=1 vertices are only (1,1) and (1,2).
        assert_eq!(part2(&inputs).unwrap(), 12);
    }
}
