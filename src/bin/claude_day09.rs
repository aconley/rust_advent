use rust_advent::Point2d;

fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_points2d("09")?;
    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
    Ok(())
}

/// Andrew's monotone chain convex hull algorithm.
/// Returns the convex hull points in counter-clockwise order.
/// Time complexity: O(n log n)
fn convex_hull(points: &[Point2d]) -> Vec<Point2d> {
    if points.len() < 3 {
        return points.to_vec();
    }

    // Sort points lexicographically (first by x, then by y)
    let mut sorted = points.to_vec();
    sorted.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

    // Remove duplicates
    sorted.dedup();

    if sorted.len() < 3 {
        return sorted;
    }

    // Cross product to determine turn direction
    // Positive = counter-clockwise, Negative = clockwise, Zero = collinear
    let cross = |o: &Point2d, a: &Point2d, b: &Point2d| -> i64 {
        (a.x as i64 - o.x as i64) * (b.y as i64 - o.y as i64)
            - (a.y as i64 - o.y as i64) * (b.x as i64 - o.x as i64)
    };

    // Build lower hull
    let mut lower = Vec::new();
    for p in &sorted {
        while lower.len() >= 2 && cross(&lower[lower.len() - 2], &lower[lower.len() - 1], p) <= 0 {
            lower.pop();
        }
        lower.push(*p);
    }

    // Build upper hull
    let mut upper = Vec::new();
    for p in sorted.iter().rev() {
        while upper.len() >= 2 && cross(&upper[upper.len() - 2], &upper[upper.len() - 1], p) <= 0 {
            upper.pop();
        }
        upper.push(*p);
    }

    // Remove last point of each half because it's repeated
    lower.pop();
    upper.pop();

    // Concatenate lower and upper hull
    lower.extend(upper);
    lower
}

/// Finds the maximum area of an axis-aligned rectangle formed by any two points.
/// Uses inclusive grid counting: area = (|x2 - x1| + 1) * (|y2 - y1| + 1)
///
/// Optimization: Only checks pairs of points on the convex hull, since the
/// optimal rectangle must have both corners on the hull.
/// Time complexity: O(n log n + h²) where h is the hull size
fn part1(inputs: &[Point2d]) -> usize {
    if inputs.len() < 2 {
        return 0;
    }

    // Compute convex hull: O(n log n)
    let hull = convex_hull(inputs);

    if hull.len() < 2 {
        return 0;
    }

    // Check all pairs on hull: O(h²) where h << n typically
    let mut max_area: i64 = 0;

    for i in 0..hull.len() {
        for j in (i + 1)..hull.len() {
            let width = (hull[i].x - hull[j].x).abs() as i64 + 1;
            let height = (hull[i].y - hull[j].y).abs() as i64 + 1;
            let area = width * height;
            max_area = max_area.max(area);
        }
    }

    max_area as usize
}

/// Checks if a point is on a line segment (for rectilinear edges only).
fn is_on_segment(point: Point2d, p1: Point2d, p2: Point2d) -> bool {
    if p1.x == p2.x {
        // Vertical segment
        point.x == p1.x && point.y >= p1.y.min(p2.y) && point.y <= p1.y.max(p2.y)
    } else if p1.y == p2.y {
        // Horizontal segment
        point.y == p1.y && point.x >= p1.x.min(p2.x) && point.x <= p1.x.max(p2.x)
    } else {
        false // Invalid for rectilinear polygon
    }
}

/// Checks if a point is on the boundary of the polygon.
fn point_on_boundary(point: Point2d, polygon: &[Point2d]) -> bool {
    let n = polygon.len();
    for i in 0..n {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % n];
        if is_on_segment(point, p1, p2) {
            return true;
        }
    }
    false
}

/// Ray casting algorithm to determine if a point is inside a polygon.
/// Casts a horizontal ray to the right and counts edge crossings.
fn point_in_polygon(point: Point2d, polygon: &[Point2d]) -> bool {
    let mut inside = false;
    let n = polygon.len();

    let mut j = n - 1;
    for i in 0..n {
        let pi = polygon[i];
        let pj = polygon[j];

        // Check if ray crosses this edge
        if ((pi.y > point.y) != (pj.y > point.y))
            && (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x)
        {
            inside = !inside;
        }
        j = i;
    }

    inside
}

/// Checks if a point is inside or on the polygon boundary.
fn point_in_or_on_polygon(point: Point2d, polygon: &[Point2d]) -> bool {
    point_in_polygon(point, polygon) || point_on_boundary(point, polygon)
}

#[derive(Copy, Clone)]
struct Edge {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Edge {
    fn is_vertical(&self) -> bool {
        self.x1 == self.x2
    }

    fn is_horizontal(&self) -> bool {
        self.y1 == self.y2
    }
}

fn build_edges(points: &[Point2d]) -> Vec<Edge> {
    let mut edges = Vec::with_capacity(points.len());
    for i in 0..points.len() {
        let p1 = &points[i];
        let p2 = &points[(i + 1) % points.len()];

        // Skip non-axis-aligned edges (only handle rectilinear polygons)
        if p1.x != p2.x && p1.y != p2.y {
            continue;
        }

        edges.push(Edge {
            x1: p1.x,
            y1: p1.y,
            x2: p2.x,
            y2: p2.y,
        });
    }
    edges
}

/// Checks if an entire rectangle is inside the polygon.
/// Uses explicit edge-crossing detection to avoid blind spots from sampling.
fn rectangle_in_polygon(p1: Point2d, p2: Point2d, polygon: &[Point2d]) -> bool {
    let min_x = p1.x.min(p2.x);
    let max_x = p1.x.max(p2.x);
    let min_y = p1.y.min(p2.y);
    let max_y = p1.y.max(p2.y);

    // Check all four corners
    let corners = [
        Point2d { x: min_x, y: min_y },
        Point2d { x: min_x, y: max_y },
        Point2d { x: max_x, y: min_y },
        Point2d { x: max_x, y: max_y },
    ];

    for corner in &corners {
        if !point_in_or_on_polygon(*corner, polygon) {
            return false;
        }
    }

    // Check center point for concave polygons
    let center_x = (min_x + max_x) / 2;
    let center_y = (min_y + max_y) / 2;
    if !point_in_or_on_polygon(Point2d { x: center_x, y: center_y }, polygon) {
        return false;
    }

    // Build edges and check for edge-crossing
    let edges = build_edges(polygon);

    for edge in &edges {
        if edge.is_vertical() {
            let x = edge.x1;
            // Check if edge is strictly inside rectangle's x-range
            if x > min_x && x < max_x {
                let (y_low, y_high) = if edge.y1 <= edge.y2 {
                    (edge.y1, edge.y2)
                } else {
                    (edge.y2, edge.y1)
                };
                // Check if edge's y-range overlaps rectangle's y-range
                if y_high > min_y && y_low < max_y {
                    return false;
                }
            }
        } else if edge.is_horizontal() {
            let y = edge.y1;
            // Check if edge is strictly inside rectangle's y-range
            if y > min_y && y < max_y {
                let (x_low, x_high) = if edge.x1 <= edge.x2 {
                    (edge.x1, edge.x2)
                } else {
                    (edge.x2, edge.x1)
                };
                // Check if edge's x-range overlaps rectangle's x-range
                if x_high > min_x && x_low < max_x {
                    return false;
                }
            }
        }
    }

    true
}

/// Finds the maximum area rectangle that fits entirely within a rectilinear polygon.
/// The polygon is formed by connecting consecutive points with horizontal/vertical lines.
fn part2(inputs: &[Point2d]) -> usize {
    if inputs.len() < 3 {
        return 0;
    }

    let mut max_area: i64 = 0;

    // Try all pairs of input points as opposite corners
    for i in 0..inputs.len() {
        for j in (i + 1)..inputs.len() {
            let p1 = inputs[i];
            let p2 = inputs[j];

            // Check if rectangle is entirely within polygon
            if rectangle_in_polygon(p1, p2, inputs) {
                let width = (p1.x - p2.x).abs() as i64 + 1;
                let height = (p1.y - p2.y).abs() as i64 + 1;
                let area = width * height;
                max_area = max_area.max(area);
            }
        }
    }

    max_area as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convex_hull_triangle() {
        let points = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 4, y: 9 },
            Point2d { x: 2, y: 3 },
        ];
        let hull = convex_hull(&points);
        // All three points should be on the hull
        assert_eq!(hull.len(), 3);
    }

    #[test]
    fn test_convex_hull_with_interior_point() {
        let points = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 10, y: 0 },
            Point2d { x: 10, y: 10 },
            Point2d { x: 0, y: 10 },
            Point2d { x: 5, y: 5 }, // Interior point
        ];
        let hull = convex_hull(&points);
        // Only the 4 corners should be on the hull
        assert_eq!(hull.len(), 4);
        // Interior point should not be in hull
        assert!(!hull.contains(&Point2d { x: 5, y: 5 }));
    }

    #[test]
    fn test_convex_hull_collinear() {
        let points = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 1, y: 1 },
            Point2d { x: 2, y: 2 },
            Point2d { x: 3, y: 3 },
        ];
        let hull = convex_hull(&points);
        // Only endpoints of collinear points
        assert_eq!(hull.len(), 2);
    }

    #[test]
    fn test_convex_hull_duplicates() {
        let points = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 0, y: 0 },
            Point2d { x: 1, y: 1 },
            Point2d { x: 1, y: 1 },
        ];
        let hull = convex_hull(&points);
        // Should handle duplicates
        assert_eq!(hull.len(), 2);
    }

    #[test]
    fn test_example_from_problem() {
        // Example from the prompt
        let points = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 4, y: 9 },
            Point2d { x: 2, y: 3 },
        ];
        assert_eq!(part1(&points), 50);
    }

    #[test]
    fn test_empty_input() {
        let points = vec![];
        assert_eq!(part1(&points), 0);
    }

    #[test]
    fn test_single_point() {
        let points = vec![Point2d { x: 5, y: 10 }];
        assert_eq!(part1(&points), 0);
    }

    #[test]
    fn test_two_identical_points() {
        let points = vec![Point2d { x: 3, y: 3 }, Point2d { x: 3, y: 3 }];
        assert_eq!(part1(&points), 1);
    }

    #[test]
    fn test_unit_square() {
        let points = vec![Point2d { x: 0, y: 0 }, Point2d { x: 0, y: 0 }];
        assert_eq!(part1(&points), 1);
    }

    #[test]
    fn test_horizontal_line() {
        let points = vec![Point2d { x: 0, y: 5 }, Point2d { x: 3, y: 5 }];
        // width = |3 - 0| + 1 = 4, height = |5 - 5| + 1 = 1
        assert_eq!(part1(&points), 4);
    }

    #[test]
    fn test_vertical_line() {
        let points = vec![Point2d { x: 7, y: 2 }, Point2d { x: 7, y: 5 }];
        // width = |7 - 7| + 1 = 1, height = |5 - 2| + 1 = 4
        assert_eq!(part1(&points), 4);
    }

    #[test]
    fn test_negative_coordinates() {
        let points = vec![Point2d { x: -5, y: -3 }, Point2d { x: -2, y: -1 }];
        // width = |-2 - (-5)| + 1 = 4, height = |-1 - (-3)| + 1 = 3
        assert_eq!(part1(&points), 12);
    }

    #[test]
    fn test_mixed_positive_negative() {
        let points = vec![Point2d { x: -2, y: -1 }, Point2d { x: 2, y: 3 }];
        // width = |2 - (-2)| + 1 = 5, height = |3 - (-1)| + 1 = 5
        assert_eq!(part1(&points), 25);
    }

    #[test]
    fn test_large_values() {
        let points = vec![
            Point2d { x: 0, y: 0 },
            Point2d {
                x: 10000,
                y: 10000,
            },
        ];
        // width = 10001, height = 10001, area = 100020001
        assert_eq!(part1(&points), 100020001);
    }

    #[test]
    fn test_multiple_points_finds_maximum() {
        let points = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 1, y: 1 }, // small rectangle with point 0
            Point2d { x: 5, y: 5 }, // larger rectangle with point 0
            Point2d { x: 2, y: 2 }, // medium rectangle with point 0
        ];
        // Maximum should be between point 0 (0,0) and point 2 (5,5)
        // width = 6, height = 6, area = 36
        assert_eq!(part1(&points), 36);
    }

    #[test]
    fn test_order_independence() {
        let points1 = vec![Point2d { x: 1, y: 2 }, Point2d { x: 4, y: 6 }];
        let points2 = vec![Point2d { x: 4, y: 6 }, Point2d { x: 1, y: 2 }];
        assert_eq!(part1(&points1), part1(&points2));
    }

    // Part 2 tests
    #[test]
    fn test_part2_example() {
        // Example from the problem
        let points = vec![
            Point2d { x: 7, y: 1 },
            Point2d { x: 11, y: 1 },
            Point2d { x: 11, y: 7 },
            Point2d { x: 9, y: 7 },
            Point2d { x: 9, y: 5 },
            Point2d { x: 2, y: 5 },
            Point2d { x: 2, y: 3 },
            Point2d { x: 7, y: 3 },
        ];
        // Largest rectangle is from (2,3) to (9,5) with area 8 * 3 = 24
        assert_eq!(part2(&points), 24);
    }

    #[test]
    fn test_part2_simple_square() {
        // Simple square polygon
        let points = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 10, y: 0 },
            Point2d { x: 10, y: 10 },
            Point2d { x: 0, y: 10 },
        ];
        // Largest rectangle is the entire square: 11 * 11 = 121
        assert_eq!(part2(&points), 121);
    }

    #[test]
    fn test_part2_l_shape() {
        // L-shaped polygon
        let points = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 10, y: 0 },
            Point2d { x: 10, y: 5 },
            Point2d { x: 5, y: 5 },
            Point2d { x: 5, y: 10 },
            Point2d { x: 0, y: 10 },
        ];
        // Largest should be from (0,0) to (10,5) or (0,5) to (5,10)
        // Both give area 11 * 6 = 66 or 6 * 6 = 36
        let result = part2(&points);
        assert_eq!(result, 66);
    }

    #[test]
    fn test_part2_point_on_boundary() {
        let point = Point2d { x: 5, y: 0 };
        let polygon = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 10, y: 0 },
            Point2d { x: 10, y: 10 },
            Point2d { x: 0, y: 10 },
        ];
        assert!(point_on_boundary(point, &polygon));
    }

    #[test]
    fn test_part2_point_inside() {
        let point = Point2d { x: 5, y: 5 };
        let polygon = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 10, y: 0 },
            Point2d { x: 10, y: 10 },
            Point2d { x: 0, y: 10 },
        ];
        assert!(point_in_polygon(point, &polygon));
    }

    #[test]
    fn test_part2_point_outside() {
        let point = Point2d { x: 15, y: 5 };
        let polygon = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 10, y: 0 },
            Point2d { x: 10, y: 10 },
            Point2d { x: 0, y: 10 },
        ];
        assert!(!point_in_polygon(point, &polygon));
        assert!(!point_on_boundary(point, &polygon));
    }

    #[test]
    fn test_part2_concave_polygon() {
        // U-shaped polygon (concave)
        let points = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 10, y: 0 },
            Point2d { x: 10, y: 10 },
            Point2d { x: 7, y: 10 },
            Point2d { x: 7, y: 3 },
            Point2d { x: 3, y: 3 },
            Point2d { x: 3, y: 10 },
            Point2d { x: 0, y: 10 },
        ];
        // Point (5, 5) should be outside this U-shape
        let inside_point = Point2d { x: 5, y: 5 };
        assert!(!point_in_polygon(inside_point, &points));

        // The result should not include rectangles that span across the U
        let result = part2(&points);
        assert!(result > 0);
        // Largest rectangle should be in one of the sides of the U
        // Left side: (0,0) to (3,10) = 4 * 11 = 44
        // Right side: (7,0) to (10,10) = 4 * 11 = 44
        // Bottom: (0,0) to (10,3) = 11 * 4 = 44
        assert_eq!(result, 44);
    }

    #[test]
    fn test_part2_thin_rectangle() {
        // Very thin rectangular polygon
        let points = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 20, y: 0 },
            Point2d { x: 20, y: 2 },
            Point2d { x: 0, y: 2 },
        ];
        assert_eq!(part2(&points), 63); // 21 * 3
    }

    #[test]
    fn test_part2_minimal_polygon() {
        // Triangle (minimum for a polygon)
        let points = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 5, y: 0 },
            Point2d { x: 0, y: 5 },
        ];
        // Should find some valid rectangle
        let result = part2(&points);
        assert!(result > 0);
    }

    #[test]
    fn test_part2_negative_coordinates() {
        // Test with negative coordinates
        let points = vec![
            Point2d { x: -5, y: -5 },
            Point2d { x: 5, y: -5 },
            Point2d { x: 5, y: 5 },
            Point2d { x: -5, y: 5 },
        ];
        assert_eq!(part2(&points), 121); // 11 * 11
    }

    #[test]
    fn test_part2_large_coordinates_no_overflow() {
        // Test that we handle large coordinate ranges without overflow
        // This would overflow if we multiply i32 * i32 without casting to i64
        let points = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 50000, y: 0 },
            Point2d { x: 50000, y: 50000 },
            Point2d { x: 0, y: 50000 },
        ];
        // Should not panic with overflow
        let result = part2(&points);
        // Largest rectangle is the full square
        assert!(result > 0);
    }

    #[test]
    fn test_part2_rectangle_corners_outside() {
        // Create a cross-shaped polygon where some rectangle corners would be outside
        let points = vec![
            Point2d { x: 5, y: 0 },
            Point2d { x: 10, y: 0 },
            Point2d { x: 10, y: 5 },
            Point2d { x: 15, y: 5 },
            Point2d { x: 15, y: 10 },
            Point2d { x: 10, y: 10 },
            Point2d { x: 10, y: 15 },
            Point2d { x: 5, y: 15 },
            Point2d { x: 5, y: 10 },
            Point2d { x: 0, y: 10 },
            Point2d { x: 0, y: 5 },
            Point2d { x: 5, y: 5 },
        ];
        // Rectangle from (0,5) to (15,10) would have corners outside
        let result = part2(&points);
        assert!(result > 0);
        // Should be less than the full cross dimensions
        assert!(result <= 216); // 16 * 16 would be if it were a full square
    }

    #[test]
    fn test_large_dataset() {
        // Create a circle of points with many interior points
        let mut points = Vec::new();

        // Add points on a circle (these should be on the hull)
        for i in 0..20 {
            let angle = (i as f64) * 2.0 * std::f64::consts::PI / 20.0;
            points.push(Point2d {
                x: (100.0 * angle.cos()) as i32,
                y: (100.0 * angle.sin()) as i32,
            });
        }

        // Add interior points (these should NOT affect the result)
        for x in -50..50 {
            for y in -50..50 {
                if x * x + y * y < 2500 { // Inside circle of radius 50
                    points.push(Point2d { x, y });
                }
            }
        }

        // Verify the convex hull optimization works on large datasets
        let result = part1(&points);
        assert!(result > 0);

        let hull_size = convex_hull(&points).len();
        let n = points.len();
        let naive_comparisons = n * (n - 1) / 2;
        let optimized_comparisons = hull_size * (hull_size - 1) / 2;

        println!("\nConvex hull optimization stats:");
        println!("  Total points: {}", n);
        println!("  Hull size: {}", hull_size);
        println!("  Comparisons without optimization: {}", naive_comparisons);
        println!("  Comparisons with optimization: {}", optimized_comparisons);
        println!("  Reduction: {:.1}x fewer comparisons\n",
                 naive_comparisons as f64 / optimized_comparisons as f64);
    }

    #[test]
    fn test_part2_thin_intrusion_at_sample_boundary() {
        // U-shaped polygon with notch cut from middle (x=2 to x=18, y=2 to y=10)
        // This tests that vertical edges at x=2 and x=18 correctly reject rectangles
        let points = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 20, y: 0 },
            Point2d { x: 20, y: 10 },
            Point2d { x: 18, y: 10 },
            Point2d { x: 18, y: 2 },
            Point2d { x: 2, y: 2 },
            Point2d { x: 2, y: 10 },
            Point2d { x: 0, y: 10 },
        ];

        // The vertical edge at x=2 (from y=2 to y=10) would cut through any rectangle
        // that has min_x < 2 and max_x > 2 and overlaps y-range [2,10]
        // The vertical edge at x=18 similarly blocks rectangles with x=18 in interior

        // Maximum valid rectangle using input points as corners:
        // (0,0) to (18,2) = 19 * 3 = 57 (bottom strip below the notch)
        // The edge at x=2 doesn't block this because y-range [2,10] doesn't overlap [0,2]
        // The edge at x=18 is on the boundary, not in interior
        assert_eq!(part2(&points), 57);
    }

    #[test]
    fn test_part2_horizontal_intrusion() {
        // Sideways U-shaped polygon with notch cut from top (x=2 to x=18, y=18 to y=20)
        // This tests that horizontal edges at y=18 correctly reject rectangles
        let points = vec![
            Point2d { x: 0, y: 0 },
            Point2d { x: 20, y: 0 },
            Point2d { x: 20, y: 20 },
            Point2d { x: 18, y: 20 },
            Point2d { x: 18, y: 18 },
            Point2d { x: 2, y: 18 },
            Point2d { x: 2, y: 20 },
            Point2d { x: 0, y: 20 },
        ];

        // The horizontal edge at y=18 (from x=18 to x=2) would cut through any rectangle
        // that has min_y < 18 and max_y > 18 and overlaps x-range [2,18]

        // Maximum valid rectangle using input points as corners:
        // (0,0) to (18,18) = 19 * 19 = 361 (main area below the notch)
        // The edge at y=18 is on the boundary, not in interior
        let result = part2(&points);
        assert_eq!(result, 361);
    }
}
