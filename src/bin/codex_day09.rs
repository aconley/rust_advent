use rust_advent::Point2d;

fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_points2d("09")?;
    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
    Ok(())
}

fn part1(inputs: &[Point2d]) -> usize {
    if inputs.len() < 2 {
        return 0;
    }

    let mut max_area: i64 = 0;
    for i in 0..inputs.len() {
        let a = &inputs[i];
        for b in &inputs[(i + 1)..] {
            let dx = (a.x as i64 - b.x as i64).abs() + 1;
            let dy = (a.y as i64 - b.y as i64).abs() + 1;
            let area = dx * dy;
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area as usize
}

fn part2(inputs: &[Point2d]) -> usize {
    if inputs.len() < 2 {
        return 0;
    }

    let edges = build_edges(inputs);

    let mut max_area: i64 = 0;
    for i in 0..inputs.len() {
        let a = &inputs[i];
        for b in &inputs[(i + 1)..] {
            let min_x = (a.x.min(b.x)) as i64;
            let max_x = (a.x.max(b.x)) as i64;
            let min_y = (a.y.min(b.y)) as i64;
            let max_y = (a.y.max(b.y)) as i64;
            let area = (max_x - min_x + 1) * (max_y - min_y + 1);
            if area <= max_area {
                continue;
            }
            if rectangle_inside(min_x, max_x, min_y, max_y, &edges) {
                max_area = area;
            }
        }
    }

    max_area as usize
}

#[derive(Copy, Clone)]
struct Edge {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
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
        let a = &points[i];
        let b = &points[(i + 1) % points.len()];
        if a.x != b.x && a.y != b.y {
            panic!(
                "Non-axis-aligned edge between ({},{}) and ({},{})",
                a.x, a.y, b.x, b.y
            );
        }
        edges.push(Edge {
            x1: a.x as i64,
            y1: a.y as i64,
            x2: b.x as i64,
            y2: b.y as i64,
        });
    }
    edges
}

fn rectangle_inside(min_x: i64, max_x: i64, min_y: i64, max_y: i64, edges: &[Edge]) -> bool {
    let corners = [
        (min_x, min_y),
        (min_x, max_y),
        (max_x, min_y),
        (max_x, max_y),
    ];
    for (x, y) in corners {
        if !point_in_polygon(x, y, edges) {
            return false;
        }
    }

    if min_x == max_x && min_y == max_y {
        return true;
    }

    if min_x == max_x {
        return !segment_crosses_boundary_vertical(min_x, min_y, max_y, edges);
    }
    if min_y == max_y {
        return !segment_crosses_boundary_horizontal(min_y, min_x, max_x, edges);
    }

    let center_x = (min_x + max_x) as f64 / 2.0;
    let center_y = (min_y + max_y) as f64 / 2.0;
    if !point_in_polygon_f64(center_x, center_y, edges) {
        return false;
    }

    for edge in edges {
        if edge.is_vertical() {
            let x = edge.x1;
            if x > min_x && x < max_x {
                let (y_low, y_high) = ordered(edge.y1, edge.y2);
                if y_high > min_y && y_low < max_y {
                    return false;
                }
            }
        } else if edge.is_horizontal() {
            let y = edge.y1;
            if y > min_y && y < max_y {
                let (x_low, x_high) = ordered(edge.x1, edge.x2);
                if x_high > min_x && x_low < max_x {
                    return false;
                }
            }
        }
    }
    true
}

fn segment_crosses_boundary_vertical(x: i64, y1: i64, y2: i64, edges: &[Edge]) -> bool {
    let (y_low, y_high) = ordered(y1, y2);
    for edge in edges {
        if edge.is_horizontal() {
            let y = edge.y1;
            if y > y_low && y < y_high {
                let (x_low, x_high) = ordered(edge.x1, edge.x2);
                if x > x_low && x < x_high {
                    return true;
                }
            }
        }
    }
    false
}

fn segment_crosses_boundary_horizontal(y: i64, x1: i64, x2: i64, edges: &[Edge]) -> bool {
    let (x_low, x_high) = ordered(x1, x2);
    for edge in edges {
        if edge.is_vertical() {
            let x = edge.x1;
            if x > x_low && x < x_high {
                let (y_low, y_high) = ordered(edge.y1, edge.y2);
                if y > y_low && y < y_high {
                    return true;
                }
            }
        }
    }
    false
}

fn point_in_polygon(x: i64, y: i64, edges: &[Edge]) -> bool {
    for edge in edges {
        if point_on_edge(x, y, edge) {
            return true;
        }
    }

    let px = x as f64;
    let py = y as f64;
    point_in_polygon_f64(px, py, edges)
}

fn point_in_polygon_f64(px: f64, py: f64, edges: &[Edge]) -> bool {
    let mut inside = false;
    for edge in edges {
        if edge.is_vertical() {
            let x_edge = edge.x1 as f64;
            let (y_low, y_high) = ordered(edge.y1, edge.y2);
            let y_low = y_low as f64;
            let y_high = y_high as f64;
            if py >= y_low && py < y_high && x_edge > px {
                inside = !inside;
            }
        }
    }
    inside
}

fn point_on_edge(x: i64, y: i64, edge: &Edge) -> bool {
    if edge.is_vertical() {
        if x != edge.x1 {
            return false;
        }
        let (y_low, y_high) = ordered(edge.y1, edge.y2);
        y >= y_low && y <= y_high
    } else {
        if y != edge.y1 {
            return false;
        }
        let (x_low, x_high) = ordered(edge.x1, edge.x2);
        x >= x_low && x <= x_high
    }
}

fn ordered(a: i64, b: i64) -> (i64, i64) {
    if a <= b { (a, b) } else { (b, a) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pt(x: i32, y: i32) -> Point2d {
        Point2d { x, y }
    }

    #[test]
    fn example_from_prompt() {
        let inputs = vec![
            pt(7, 1),
            pt(11, 1),
            pt(11, 7),
            pt(9, 7),
            pt(9, 5),
            pt(2, 5),
            pt(2, 3),
            pt(7, 3),
        ];
        assert_eq!(part1(&inputs), 50);
    }

    #[test]
    fn empty_and_single_point() {
        assert_eq!(part1(&[]), 0);
        assert_eq!(part1(&[pt(3, 4)]), 0);
    }

    #[test]
    fn duplicate_points_have_unit_area() {
        let inputs = vec![pt(5, -2), pt(5, -2)];
        assert_eq!(part1(&inputs), 1);
    }

    #[test]
    fn inclusive_area_on_axis_aligned_line() {
        let inputs = vec![pt(2, 2), pt(2, 5)];
        assert_eq!(part1(&inputs), 4);
    }

    #[test]
    fn handles_negative_coordinates() {
        let inputs = vec![pt(-3, -2), pt(2, 4), pt(-10, 0)];
        assert_eq!(part1(&inputs), 65);
    }

    #[test]
    fn large_coordinate_span() {
        let inputs = vec![
            pt(-1_000_000_000, 1_000_000_000),
            pt(1_000_000_000, -1_000_000_000),
        ];
        assert_eq!(part1(&inputs), 4_000_000_004_000_000_001usize);
    }

    #[test]
    fn max_area_from_non_adjacent_points() {
        let inputs = vec![pt(0, 0), pt(10, 0), pt(0, 10), pt(6, 6)];
        assert_eq!(part1(&inputs), 121);
    }

    #[test]
    fn part2_example_from_prompt() {
        let inputs = vec![
            pt(7, 1),
            pt(11, 1),
            pt(11, 7),
            pt(9, 7),
            pt(9, 5),
            pt(2, 5),
            pt(2, 3),
            pt(7, 3),
        ];
        assert_eq!(part2(&inputs), 24);
    }

    #[test]
    fn part2_rectangle_loop() {
        let inputs = vec![pt(0, 0), pt(4, 0), pt(4, 3), pt(0, 3)];
        assert_eq!(part2(&inputs), 20);
    }

    #[test]
    fn part2_concave_l_shape() {
        let inputs = vec![pt(0, 0), pt(4, 0), pt(4, 1), pt(1, 1), pt(1, 4), pt(0, 4)];
        assert_eq!(part2(&inputs), 10);
    }

    #[test]
    fn part2_u_shape() {
        let inputs = vec![
            pt(0, 0),
            pt(4, 0),
            pt(4, 4),
            pt(3, 4),
            pt(3, 1),
            pt(1, 1),
            pt(1, 4),
            pt(0, 4),
        ];
        assert_eq!(part2(&inputs), 10);
    }

    #[test]
    fn part2_empty_and_single_point() {
        let empty: Vec<Point2d> = Vec::new();
        assert_eq!(part2(&empty), 0);

        let inputs = vec![pt(2, 3)];
        assert_eq!(part2(&inputs), 0);
    }

    #[test]
    fn part2_rectangle_with_duplicate_points() {
        let inputs = vec![pt(0, 0), pt(4, 0), pt(4, 3), pt(4, 3), pt(0, 3)];
        assert_eq!(part2(&inputs), 20);
    }

    #[test]
    fn part2_wide_u_shape_prefers_bottom_bar() {
        let inputs = vec![
            pt(0, 0),
            pt(6, 0),
            pt(6, 6),
            pt(4, 6),
            pt(4, 2),
            pt(2, 2),
            pt(2, 6),
            pt(0, 6),
        ];
        assert_eq!(part2(&inputs), 21);
    }

    #[test]
    fn part2_notched_top_prefers_side_bar() {
        let inputs = vec![
            pt(0, 0),
            pt(6, 0),
            pt(6, 6),
            pt(4, 6),
            pt(4, 3),
            pt(2, 3),
            pt(2, 6),
            pt(0, 6),
        ];
        assert_eq!(part2(&inputs), 21);
    }
}
