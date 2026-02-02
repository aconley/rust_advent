use rust_advent::Point2d;
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Custom error type for puzzle parsing and solving
#[derive(Debug, Clone)]
enum PuzzleError {
    InvalidShape { line: usize, reason: String },
    InvalidRegion { line: String, reason: String },
    EmptyShape { id: usize },
    InvalidInput(String),
}

impl fmt::Display for PuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PuzzleError::InvalidShape { line, reason } => {
                write!(f, "Invalid shape at line {}: {}", line, reason)
            }
            PuzzleError::InvalidRegion { line, reason } => {
                write!(f, "Invalid region '{}': {}", line, reason)
            }
            PuzzleError::EmptyShape { id } => {
                write!(f, "Shape {} has no occupied cells", id)
            }
            PuzzleError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for PuzzleError {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = rust_advent::read_file_as_lines("12")?;
    let result = part1(&inputs)?;
    println!("Part 1: {}", result);
    Ok(())
}

/// Represents a 2D shape with normalized positions (min x,y at 0,0)
#[derive(Debug, Clone, PartialEq, Eq)]
struct Shape {
    id: usize,
    positions: Vec<Point2d>,
    width: i32,
    height: i32,
}

/// Represents a shape variant (rotation/flip)
#[derive(Debug, Clone, PartialEq, Eq)]
struct ShapeVariant {
    positions: Vec<Point2d>,
    width: i32,
    height: i32,
}

/// Represents a rectangular region with shape requirements
#[derive(Debug, Clone)]
struct Region {
    width: i32,
    height: i32,
    shape_counts: Vec<usize>,
}

/// Grid state for tracking placements
#[derive(Debug, Clone)]
struct Grid {
    width: i32,
    height: i32,
    cells: Vec<Vec<bool>>,
    empty_count: usize,
}

fn part1(input: &[String]) -> Result<u32, PuzzleError> {
    let (shapes, regions) = parse_input(input)?;

    if shapes.is_empty() {
        return Err(PuzzleError::InvalidInput(
            "No shapes found in input".to_string(),
        ));
    }

    let mut satisfied_count = 0;
    for region in regions {
        if can_fit_region(&region, &shapes) {
            satisfied_count += 1;
        }
    }

    Ok(satisfied_count)
}

/// Parse the entire input into shapes and regions
fn parse_input(lines: &[String]) -> Result<(Vec<Shape>, Vec<Region>), PuzzleError> {
    let mut shapes = Vec::new();
    let mut regions = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        // Check if this is a shape (format: "N:")
        if line.ends_with(':') && line.len() > 1 {
            if let Ok(id) = line[..line.len() - 1].parse::<usize>() {
                let start_line = i;
                let shape = parse_shape(lines, &mut i, id, start_line)?;
                shapes.push(shape);
                continue;
            }
        }

        // Check if this is a region (format: "WxH: ...")
        if line.contains('x') && line.contains(':') {
            let region = parse_region(line)?;
            regions.push(region);
        }

        i += 1;
    }

    Ok((shapes, regions))
}

/// Parse a single shape definition
fn parse_shape(
    lines: &[String],
    start: &mut usize,
    id: usize,
    start_line: usize,
) -> Result<Shape, PuzzleError> {
    *start += 1; // Move past the "N:" line

    let mut positions = Vec::new();
    let mut pattern_lines = Vec::new();

    while *start < lines.len() {
        let line = &lines[*start];
        if line.trim().is_empty() {
            break;
        }

        // Check if this is the start of a new section
        if line.contains(':') && line.len() > 1 {
            // Don't consume this line, it's the next section
            break;
        }

        // Validate that line only contains valid characters
        for ch in line.chars() {
            if ch != '#' && ch != '.' && !ch.is_whitespace() {
                return Err(PuzzleError::InvalidShape {
                    line: *start + 1,
                    reason: format!("Invalid character '{}' in shape pattern", ch),
                });
            }
        }

        pattern_lines.push(line.as_str());
        *start += 1;
    }

    if pattern_lines.is_empty() {
        return Err(PuzzleError::InvalidShape {
            line: start_line + 1,
            reason: "Shape has no pattern lines".to_string(),
        });
    }

    // Parse the pattern to extract positions
    for (y, line) in pattern_lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                positions.push(Point2d {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    if positions.is_empty() {
        return Err(PuzzleError::EmptyShape { id });
    }

    let (normalized_positions, width, height) = normalize_positions(&positions);

    Ok(Shape {
        id,
        positions: normalized_positions,
        width,
        height,
    })
}

/// Parse a single region specification
fn parse_region(line: &str) -> Result<Region, PuzzleError> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        return Err(PuzzleError::InvalidRegion {
            line: line.to_string(),
            reason: "Expected format 'WxH: count0 count1 ...'".to_string(),
        });
    }

    // Parse dimensions "WxH"
    let dims: Vec<&str> = parts[0].trim().split('x').collect();
    if dims.len() != 2 {
        return Err(PuzzleError::InvalidRegion {
            line: line.to_string(),
            reason: format!("Invalid dimensions '{}', expected 'WxH'", parts[0]),
        });
    }

    let width = dims[0].parse::<i32>().map_err(|_| PuzzleError::InvalidRegion {
        line: line.to_string(),
        reason: format!("Invalid width '{}'", dims[0]),
    })?;

    let height = dims[1].parse::<i32>().map_err(|_| PuzzleError::InvalidRegion {
        line: line.to_string(),
        reason: format!("Invalid height '{}'", dims[1]),
    })?;

    if width <= 0 || height <= 0 {
        return Err(PuzzleError::InvalidRegion {
            line: line.to_string(),
            reason: format!("Dimensions must be positive, got {}x{}", width, height),
        });
    }

    // Parse shape counts
    let counts: Vec<usize> = parts[1]
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    if counts.is_empty() {
        return Err(PuzzleError::InvalidRegion {
            line: line.to_string(),
            reason: "No shape counts specified".to_string(),
        });
    }

    Ok(Region {
        width,
        height,
        shape_counts: counts,
    })
}

/// Normalize shape positions to have min x,y at (0,0) - single pass optimization
fn normalize_positions(positions: &[Point2d]) -> (Vec<Point2d>, i32, i32) {
    if positions.is_empty() {
        return (Vec::new(), 0, 0);
    }

    // Single pass to find bounds
    let (min_x, min_y, max_x, max_y) = positions.iter().fold(
        (i32::MAX, i32::MAX, i32::MIN, i32::MIN),
        |(min_x, min_y, max_x, max_y), p| {
            (
                min_x.min(p.x),
                min_y.min(p.y),
                max_x.max(p.x),
                max_y.max(p.y),
            )
        },
    );

    let normalized: Vec<Point2d> = positions
        .iter()
        .map(|p| Point2d {
            x: p.x - min_x,
            y: p.y - min_y,
        })
        .collect();

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    (normalized, width, height)
}

/// Rotate positions 90 degrees clockwise
fn rotate_90(positions: &[Point2d], _width: i32, height: i32) -> Vec<Point2d> {
    positions
        .iter()
        .map(|p| Point2d {
            x: height - 1 - p.y,
            y: p.x,
        })
        .collect()
}

/// Flip positions horizontally
fn flip_horizontal(positions: &[Point2d], width: i32) -> Vec<Point2d> {
    positions
        .iter()
        .map(|p| Point2d {
            x: width - 1 - p.x,
            y: p.y,
        })
        .collect()
}

/// Generate all unique transformations of a shape
fn generate_all_variants(shape: &Shape) -> Vec<ShapeVariant> {
    let mut variants = Vec::new();
    let mut current_positions = shape.positions.clone();
    let mut current_width = shape.width;
    let mut current_height = shape.height;

    // Generate 4 rotations
    for _ in 0..4 {
        // Add current rotation
        variants.push(ShapeVariant {
            positions: current_positions.clone(),
            width: current_width,
            height: current_height,
        });

        // Add flipped version
        let flipped = flip_horizontal(&current_positions, current_width);
        variants.push(ShapeVariant {
            positions: flipped,
            width: current_width,
            height: current_height,
        });

        // Rotate for next iteration
        current_positions = rotate_90(&current_positions, current_width, current_height);
        std::mem::swap(&mut current_width, &mut current_height);
    }

    deduplicate_variants(variants)
}

/// Deduplicate shape variants (remove symmetric duplicates)
fn deduplicate_variants(variants: Vec<ShapeVariant>) -> Vec<ShapeVariant> {
    let mut seen = HashSet::new();
    let mut unique = Vec::new();

    for variant in variants {
        // Create a normalized representation for comparison using tuples
        let mut sorted_positions: Vec<(i32, i32)> = variant
            .positions
            .iter()
            .map(|p| (p.x, p.y))
            .collect();
        sorted_positions.sort();

        let key = (sorted_positions, variant.width, variant.height);
        if seen.insert(key) {
            unique.push(variant);
        }
    }

    unique
}

/// Create a new empty grid
fn create_grid(width: i32, height: i32) -> Grid {
    let empty_count = (width * height) as usize;
    Grid {
        width,
        height,
        cells: vec![vec![false; width as usize]; height as usize],
        empty_count,
    }
}

/// Check if a shape variant can be placed at the given origin
fn can_place(grid: &Grid, variant: &ShapeVariant, origin: Point2d) -> bool {
    for pos in &variant.positions {
        let x = origin.x + pos.x;
        let y = origin.y + pos.y;

        // Check bounds
        if x < 0 || y < 0 || x >= grid.width || y >= grid.height {
            return false;
        }

        // Check if cell is already occupied
        if grid.cells[y as usize][x as usize] {
            return false;
        }
    }

    true
}

/// Place a piece on the grid
fn place_piece(grid: &mut Grid, variant: &ShapeVariant, origin: Point2d) {
    for pos in &variant.positions {
        let x = (origin.x + pos.x) as usize;
        let y = (origin.y + pos.y) as usize;
        grid.cells[y][x] = true;
    }
    grid.empty_count -= variant.positions.len();
}

/// Remove a piece from the grid (for backtracking)
fn remove_piece(grid: &mut Grid, variant: &ShapeVariant, origin: Point2d) {
    for pos in &variant.positions {
        let x = (origin.x + pos.x) as usize;
        let y = (origin.y + pos.y) as usize;
        grid.cells[y][x] = false;
    }
    grid.empty_count += variant.positions.len();
}

/// Get the count of remaining empty cells in the grid (O(1))
fn count_empty_cells(grid: &Grid) -> usize {
    grid.empty_count
}

/// Try to fit all required pieces into the region
fn can_fit_region(region: &Region, shapes: &[Shape]) -> bool {
    // Build list of pieces to place
    let mut pieces = build_piece_list(region);

    if pieces.is_empty() {
        return true; // No pieces to place
    }

    // Generate all variants for required shapes (using entry API to avoid double lookup)
    let mut all_variants = HashMap::new();
    for (shape_id, _) in &pieces {
        all_variants.entry(*shape_id).or_insert_with(|| {
            if *shape_id < shapes.len() {
                generate_all_variants(&shapes[*shape_id])
            } else {
                Vec::new()
            }
        });
    }

    // Sort pieces by constraint (most constrained first)
    // This dramatically improves backtracking performance
    pieces.sort_by_key(|(shape_id, _)| {
        let shape_size = shapes
            .get(*shape_id)
            .map(|s| s.positions.len())
            .unwrap_or(0);
        let variant_count = all_variants.get(shape_id).map(|v| v.len()).unwrap_or(1);

        // Sort by: larger pieces first, then fewer variants first
        // Using Reverse to get descending order for size, ascending for variant count
        (std::cmp::Reverse(shape_size), variant_count)
    });

    // Create grid
    let mut grid = create_grid(region.width, region.height);

    // Try to place all pieces
    try_place_pieces(&mut grid, &pieces, 0, &all_variants, shapes)
}

/// Expand region requirements into a list of individual pieces
fn build_piece_list(region: &Region) -> Vec<(usize, usize)> {
    let mut pieces = Vec::new();
    for (shape_id, &count) in region.shape_counts.iter().enumerate() {
        for piece_index in 0..count {
            pieces.push((shape_id, piece_index));
        }
    }
    pieces
}

/// Main backtracking function to place all pieces
fn try_place_pieces(
    grid: &mut Grid,
    pieces: &[(usize, usize)],
    current_idx: usize,
    all_variants: &HashMap<usize, Vec<ShapeVariant>>,
    shapes: &[Shape],
) -> bool {
    // Base case: all pieces placed
    if current_idx >= pieces.len() {
        return true;
    }

    // Early pruning: check if remaining pieces can possibly fit
    let remaining_cells_needed: usize = pieces[current_idx..]
        .iter()
        .filter_map(|(sid, _)| shapes.get(*sid))
        .map(|s| s.positions.len())
        .sum();

    let empty_cells = count_empty_cells(grid);
    if remaining_cells_needed > empty_cells {
        return false;
    }

    let (shape_id, _piece_index) = pieces[current_idx];

    // Get variants for this shape
    let variants = match all_variants.get(&shape_id) {
        Some(v) => v,
        None => return false,
    };

    // Try all variants
    for variant in variants {
        // Try all possible positions
        // Note: Could optimize further by only trying positions near first empty cell,
        // but that requires more sophisticated logic to maintain correctness
        for y in 0..=grid.height - variant.height {
            for x in 0..=grid.width - variant.width {
                let origin = Point2d { x, y };

                if can_place(grid, variant, origin) {
                    // Place the piece
                    place_piece(grid, variant, origin);

                    // Recurse
                    if try_place_pieces(grid, pieces, current_idx + 1, all_variants, shapes) {
                        return true;
                    }

                    // Backtrack
                    remove_piece(grid, variant, origin);
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(x: i32, y: i32) -> Point2d {
        Point2d { x, y }
    }

    #[test]
    fn test_normalize_positions() {
        let positions = vec![point(2, 3), point(3, 3), point(2, 4)];
        let (normalized, width, height) = normalize_positions(&positions);

        assert_eq!(normalized, vec![point(0, 0), point(1, 0), point(0, 1)]);
        assert_eq!(width, 2);
        assert_eq!(height, 2);
    }

    #[test]
    fn test_rotate_90() {
        // L-shape: ##
        //          #.
        let positions = vec![point(0, 0), point(1, 0), point(0, 1)];
        let rotated = rotate_90(&positions, 2, 2);

        // After 90° rotation: #.
        //                     ##
        assert_eq!(rotated, vec![point(1, 0), point(1, 1), point(0, 0)]);
    }

    #[test]
    fn test_flip_horizontal() {
        // L-shape: ##
        //          #.
        let positions = vec![point(0, 0), point(1, 0), point(0, 1)];
        let flipped = flip_horizontal(&positions, 2);

        // After flip: ##
        //             .#
        assert_eq!(flipped, vec![point(1, 0), point(0, 0), point(1, 1)]);
    }

    #[test]
    fn test_parse_shape_basic() {
        let lines = vec![
            "0:".to_string(),
            "##".to_string(),
            "#.".to_string(),
            "".to_string(),
        ];

        let mut start = 0;
        let shape = parse_shape(&lines, &mut start, 0, 0).unwrap();

        assert_eq!(shape.id, 0);
        assert_eq!(shape.positions.len(), 3);
        assert_eq!(shape.width, 2);
        assert_eq!(shape.height, 2);
    }

    #[test]
    fn test_parse_region() {
        let line = "4x4: 0 0 0 0 2 0";
        let region = parse_region(line).unwrap();

        assert_eq!(region.width, 4);
        assert_eq!(region.height, 4);
        assert_eq!(region.shape_counts, vec![0, 0, 0, 0, 2, 0]);
    }

    #[test]
    fn test_create_grid() {
        let grid = create_grid(3, 2);

        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 2);
        assert_eq!(grid.cells.len(), 2);
        assert_eq!(grid.cells[0].len(), 3);
        assert!(!grid.cells[0][0]);
        assert_eq!(grid.empty_count, 6);
    }

    #[test]
    fn test_can_place_valid() {
        let grid = create_grid(4, 4);
        let variant = ShapeVariant {
            positions: vec![point(0, 0), point(1, 0)],
            width: 2,
            height: 1,
        };

        assert!(can_place(&grid, &variant, point(0, 0)));
        assert!(can_place(&grid, &variant, point(2, 3)));
    }

    #[test]
    fn test_can_place_out_of_bounds() {
        let grid = create_grid(4, 4);
        let variant = ShapeVariant {
            positions: vec![point(0, 0), point(1, 0)],
            width: 2,
            height: 1,
        };

        assert!(!can_place(&grid, &variant, point(3, 0))); // Would go to x=4
        assert!(!can_place(&grid, &variant, point(0, 4))); // y out of bounds
    }

    #[test]
    fn test_place_and_remove_piece() {
        let mut grid = create_grid(4, 4);
        let variant = ShapeVariant {
            positions: vec![point(0, 0), point(1, 0)],
            width: 2,
            height: 1,
        };

        assert_eq!(grid.empty_count, 16);
        place_piece(&mut grid, &variant, point(1, 1));
        assert!(grid.cells[1][1]);
        assert!(grid.cells[1][2]);
        assert_eq!(grid.empty_count, 14);

        remove_piece(&mut grid, &variant, point(1, 1));
        assert!(!grid.cells[1][1]);
        assert!(!grid.cells[1][2]);
        assert_eq!(grid.empty_count, 16);
    }

    #[test]
    fn test_can_place_overlapping() {
        let mut grid = create_grid(4, 4);
        let variant = ShapeVariant {
            positions: vec![point(0, 0), point(1, 0)],
            width: 2,
            height: 1,
        };

        place_piece(&mut grid, &variant, point(0, 0));
        assert!(!can_place(&grid, &variant, point(0, 0)));
        assert!(!can_place(&grid, &variant, point(1, 0))); // Overlaps at x=1
    }

    #[test]
    fn test_generate_variants_square() {
        let shape = Shape {
            id: 0,
            positions: vec![point(0, 0), point(1, 0), point(0, 1), point(1, 1)],
            width: 2,
            height: 2,
        };

        let variants = generate_all_variants(&shape);
        // A square should have only 1 unique variant (all rotations/flips are the same)
        assert_eq!(variants.len(), 1);
    }

    #[test]
    fn test_generate_variants_line() {
        let shape = Shape {
            id: 0,
            positions: vec![point(0, 0), point(1, 0)],
            width: 2,
            height: 1,
        };

        let variants = generate_all_variants(&shape);
        // A horizontal line should have 2 unique variants (horizontal and vertical)
        assert_eq!(variants.len(), 2);
    }

    #[test]
    fn test_single_shape_exact_fit() {
        let shapes = vec![Shape {
            id: 0,
            positions: vec![point(0, 0), point(1, 0), point(0, 1), point(1, 1)],
            width: 2,
            height: 2,
        }];

        let region = Region {
            width: 2,
            height: 2,
            shape_counts: vec![1],
        };

        assert!(can_fit_region(&region, &shapes));
    }

    #[test]
    fn test_impossible_fit() {
        let shapes = vec![Shape {
            id: 0,
            positions: vec![point(0, 0), point(1, 0), point(0, 1), point(1, 1)],
            width: 2,
            height: 2,
        }];

        // Try to fit a 2x2 piece into a 1x1 grid
        let region = Region {
            width: 1,
            height: 1,
            shape_counts: vec![1],
        };

        assert!(!can_fit_region(&region, &shapes));
    }

    #[test]
    fn test_two_identical_shapes() {
        // Two 2x1 pieces
        let shapes = vec![Shape {
            id: 0,
            positions: vec![point(0, 0), point(1, 0)],
            width: 2,
            height: 1,
        }];

        // Should fit in a 4x1 or 2x2 grid
        let region = Region {
            width: 4,
            height: 1,
            shape_counts: vec![2],
        };

        assert!(can_fit_region(&region, &shapes));
    }

    #[test]
    fn test_rotation_required() {
        // 3x1 horizontal piece
        let shapes = vec![Shape {
            id: 0,
            positions: vec![point(0, 0), point(1, 0), point(2, 0)],
            width: 3,
            height: 1,
        }];

        // Must be placed vertically in a 1x3 grid
        let region = Region {
            width: 1,
            height: 3,
            shape_counts: vec![1],
        };

        assert!(can_fit_region(&region, &shapes));
    }

    #[test]
    fn test_empty_region() {
        let shapes = vec![Shape {
            id: 0,
            positions: vec![point(0, 0)],
            width: 1,
            height: 1,
        }];

        // No shapes required
        let region = Region {
            width: 5,
            height: 5,
            shape_counts: vec![0],
        };

        assert!(can_fit_region(&region, &shapes));
    }

    #[test]
    fn test_problem_example() {
        let input = vec![
            "0:".to_string(),
            "###".to_string(),
            "##.".to_string(),
            "##.".to_string(),
            "".to_string(),
            "1:".to_string(),
            "###".to_string(),
            "##.".to_string(),
            ".##".to_string(),
            "".to_string(),
            "2:".to_string(),
            ".##".to_string(),
            "###".to_string(),
            "##.".to_string(),
            "".to_string(),
            "3:".to_string(),
            "##.".to_string(),
            "###".to_string(),
            "##.".to_string(),
            "".to_string(),
            "4:".to_string(),
            "###".to_string(),
            "#..".to_string(),
            "###".to_string(),
            "".to_string(),
            "5:".to_string(),
            "###".to_string(),
            ".#.".to_string(),
            "###".to_string(),
            "".to_string(),
            "4x4: 0 0 0 0 2 0".to_string(),
            "12x5: 1 0 1 0 2 2".to_string(),
            "12x5: 1 0 1 0 3 2".to_string(),
        ];

        let result = part1(&input).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_count_empty_cells() {
        let mut grid = create_grid(3, 3);
        assert_eq!(count_empty_cells(&grid), 9);

        // Manually mark cells as occupied and update count
        grid.cells[0][0] = true;
        grid.empty_count -= 1;
        assert_eq!(count_empty_cells(&grid), 8);

        grid.cells[1][1] = true;
        grid.empty_count -= 1;
        assert_eq!(count_empty_cells(&grid), 7);
    }

    #[test]
    fn test_single_cell_shape() {
        let shapes = vec![Shape {
            id: 0,
            positions: vec![point(0, 0)],
            width: 1,
            height: 1,
        }];

        let region = Region {
            width: 2,
            height: 2,
            shape_counts: vec![3],
        };

        assert!(can_fit_region(&region, &shapes));
    }

    #[test]
    fn test_parse_full_example() {
        let input = vec![
            "0:".to_string(),
            "###".to_string(),
            "##.".to_string(),
            "##.".to_string(),
            "".to_string(),
            "4:".to_string(),
            "###".to_string(),
            "#..".to_string(),
            "###".to_string(),
            "".to_string(),
            "4x4: 0 0 0 0 2 0".to_string(),
        ];

        let (shapes, regions) = parse_input(&input).unwrap();

        assert_eq!(shapes.len(), 2);
        assert_eq!(shapes[0].id, 0);
        assert_eq!(shapes[1].id, 4);

        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].width, 4);
        assert_eq!(regions[0].height, 4);
        assert_eq!(regions[0].shape_counts, vec![0, 0, 0, 0, 2, 0]);
    }

    #[test]
    fn test_parse_error_empty_shape() {
        let input = vec![
            "0:".to_string(),
            "...".to_string(),
            "...".to_string(),
            "".to_string(),
        ];

        let result = parse_input(&input);
        assert!(result.is_err());
        match result {
            Err(PuzzleError::EmptyShape { id }) => assert_eq!(id, 0),
            _ => panic!("Expected EmptyShape error"),
        }
    }

    #[test]
    fn test_parse_error_invalid_region() {
        let input = vec!["invalid".to_string()];

        let result = parse_region(&input[0]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_negative_dimensions() {
        let result = parse_region("-5x10: 1 2 3");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_no_shape_counts() {
        let result = parse_region("5x10:");
        assert!(result.is_err());
        match result {
            Err(PuzzleError::InvalidRegion { reason, .. }) => {
                assert!(reason.contains("No shape counts"));
            }
            _ => panic!("Expected InvalidRegion error"),
        }
    }
}
