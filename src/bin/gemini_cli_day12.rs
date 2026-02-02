use std::collections::{HashMap, HashSet};

fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_file_as_lines("12")?;
    match part1(&inputs) {
        Ok(result) => {
            println!("Part 1: {}", result);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn part1(input: &[String]) -> Result<u32, String> {
    let (shapes, regions) = parse_input(input)?;
    let mut solved_count = 0;

    for region in regions {
        if solve_region(&region, &shapes) {
            solved_count += 1;
        }
    }

    Ok(solved_count)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    r: i32,
    c: i32,
}

#[derive(Clone, Debug)]
struct Shape {
    variants: Vec<Variant>, // All unique rotations/flips
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Variant {
    cells: Vec<Point>,
    height: i32,
    width: i32,
}

#[derive(Clone, Debug)]
struct Region {
    width: usize,
    height: usize,
    required_shapes: Vec<usize>, // List of shape IDs to place
}

fn parse_input(input: &[String]) -> Result<(HashMap<usize, Shape>, Vec<Region>), String> {
    let mut shapes = HashMap::new();
    let mut regions = Vec::new();

    let mut i = 0;
    while i < input.len() {
        let line = input[i].trim();
        if line.is_empty() {
            i += 1;
            continue;
        }

        if let Some(colon_idx) = line.find(':') {
            let prefix = &line[..colon_idx];
            
            if prefix.contains('x') {
                regions.push(parse_region(line, prefix, colon_idx)?);
                i += 1;
            } else {
                let (id, shape, new_i) = parse_shape(input, i, prefix)?;
                shapes.insert(id, shape);
                i = new_i;
            }
        } else {
            // Should not happen based on description, but let's be robust
             return Err(format!("Line {} does not contain ':' separator", i + 1));
        }
    }
    Ok((shapes, regions))
}

fn parse_region(line: &str, prefix: &str, colon_idx: usize) -> Result<Region, String> {
    let dims_parts: Vec<&str> = prefix.split('x').collect();
    if dims_parts.len() != 2 {
         return Err(format!("Invalid region dimensions: {}", prefix));
    }
    let width: usize = dims_parts[0].parse().map_err(|_| format!("Invalid width: {}", dims_parts[0]))?;
    let height: usize = dims_parts[1].parse().map_err(|_| format!("Invalid height: {}", dims_parts[1]))?;
    
    let counts_str = &line[colon_idx+1..].trim();
    let counts: Vec<usize> = counts_str.split_whitespace()
        .map(|s| s.parse().map_err(|_| format!("Invalid shape count: {}", s)))
        .collect::<Result<_, _>>()?;
    
    let mut required_shapes = Vec::new();
    for (shape_id, &count) in counts.iter().enumerate() {
        for _ in 0..count {
            required_shapes.push(shape_id);
        }
    }
    required_shapes.sort(); 

    Ok(Region {
        width,
        height,
        required_shapes,
    })
}

fn parse_shape(input: &[String], start_idx: usize, prefix: &str) -> Result<(usize, Shape, usize), String> {
    let id: usize = prefix.parse().map_err(|_| format!("Invalid shape ID: {}", prefix))?;
    let mut i = start_idx + 1;
    let mut raw_cells = Vec::new();
    let mut r = 0;
    
    while i < input.len() {
        let shape_line = input[i].trim();
        if shape_line.is_empty() {
            break;
        }
        for (c, char) in shape_line.chars().enumerate() {
            if char == '#' {
                raw_cells.push(Point { r, c: c as i32 });
            }
        }
        r += 1;
        i += 1;
    }
    
    let variants = generate_variants(&raw_cells);
    Ok((id, Shape { variants }, i))
}

fn generate_variants(cells: &[Point]) -> Vec<Variant> {
    let mut unique_variants = HashSet::new();
    let mut variants = Vec::new();

    let mut current = cells.to_vec();
    
    // Try all 4 rotations
    for _ in 0..4 {
        add_variant(&mut unique_variants, &mut variants, &current);
        
        // Flip and add
        let flipped: Vec<Point> = current.iter().map(|p| Point { r: p.r, c: -p.c }).collect();
        add_variant(&mut unique_variants, &mut variants, &flipped);

        // Rotate 90 degrees clockwise: (r, c) -> (c, -r)
        current = current.iter().map(|p| Point { r: p.c, c: -p.r }).collect();
    }
    
    variants
}

fn add_variant(unique: &mut HashSet<Vec<Point>>, dest: &mut Vec<Variant>, cells: &[Point]) {
    if cells.is_empty() { return; }
    
    // Normalize: top-left bounding box at (0,0)
    let min_r = cells.iter().map(|p| p.r).min().unwrap();
    let min_c = cells.iter().map(|p| p.c).min().unwrap();
    
    let mut normalized: Vec<Point> = cells.iter().map(|p| Point {
        r: p.r - min_r,
        c: p.c - min_c
    }).collect();
    
    // Sort to canonicalize for hash set check
    normalized.sort_by(|a, b| a.r.cmp(&b.r).then(a.c.cmp(&b.c)));
    
    if unique.insert(normalized.clone()) {
        let max_r = normalized.iter().map(|p| p.r).max().unwrap();
        let max_c = normalized.iter().map(|p| p.c).max().unwrap();
        dest.push(Variant {
            cells: normalized,
            height: max_r + 1,
            width: max_c + 1,
        });
    }
}

fn solve_region(region: &Region, shapes: &HashMap<usize, Shape>) -> bool {
    let mut pieces_to_place = region.required_shapes.clone();
    
    // Sort by size of shape (descending) to fail fast
    pieces_to_place.sort_by(|&a, &b| {
        let shape_a = &shapes[&a];
        let shape_b = &shapes[&b];
        let size_a = shape_a.variants[0].cells.len();
        let size_b = shape_b.variants[0].cells.len();
        size_b.cmp(&size_a).then(a.cmp(&b))
    });

    // Pre-calculate remaining area needed at each step for pruning
    let mut remaining_areas = vec![0; pieces_to_place.len() + 1];
    let mut total_area = 0;
    for (i, &id) in pieces_to_place.iter().enumerate().rev() {
        let area = shapes[&id].variants[0].cells.len();
        total_area += area;
        remaining_areas[i] = total_area;
    }

    let total_grid_cells = region.width * region.height;
    if total_area > total_grid_cells {
        return false;
    }

    let mut grid = vec![false; total_grid_cells];
    
    solve_recursive(
        region, 
        shapes, 
        &pieces_to_place, 
        &remaining_areas,
        0, 
        &mut grid, 
        0,
        total_grid_cells
    )
}

// grid is flattened: index = r * width + c
#[allow(clippy::too_many_arguments)]
fn solve_recursive(
    region: &Region, 
    shapes: &HashMap<usize, Shape>, 
    pieces: &[usize],
    remaining_areas: &[usize],
    piece_idx: usize, 
    grid: &mut [bool],
    search_start_idx: usize, // For symmetry breaking of identical pieces
    free_cells: usize,
) -> bool {
    if piece_idx >= pieces.len() {
        return true;
    }

    // Pruning: Not enough space left for remaining pieces
    if free_cells < remaining_areas[piece_idx] {
        return false;
    }

    let shape_id = pieces[piece_idx];
    let shape = &shapes[&shape_id];
    let piece_area = shape.variants[0].cells.len(); // Invariant across variants
    
    // Symmetry breaking
    let start_idx = if piece_idx > 0 && pieces[piece_idx - 1] == shape_id {
        search_start_idx
    } else {
        0
    };

    for idx in start_idx..(region.width * region.height) {
        let r = (idx / region.width) as i32;
        let c = (idx % region.width) as i32;

        for variant in &shape.variants {
            // Check bounds
            if r + variant.height > region.height as i32 || c + variant.width > region.width as i32 {
                continue;
            }

            // Check overlap
            if can_place(grid, region.width, r, c, variant) {
                place(grid, region.width, r, c, variant, true);
                
                if solve_recursive(
                    region, 
                    shapes, 
                    pieces, 
                    remaining_areas,
                    piece_idx + 1, 
                    grid, 
                    idx, 
                    free_cells - piece_area
                ) {
                    return true;
                }
                
                // Backtrack
                place(grid, region.width, r, c, variant, false);
            }
        }
    }

    false
}

fn can_place(grid: &[bool], width: usize, r: i32, c: i32, variant: &Variant) -> bool {
    for p in &variant.cells {
        let grid_r = r + p.r;
        let grid_c = c + p.c;
        let idx = (grid_r as usize) * width + (grid_c as usize);
        if grid[idx] {
            return false;
        }
    }
    true
}

fn place(grid: &mut [bool], width: usize, r: i32, c: i32, variant: &Variant, val: bool) {
    for p in &variant.cells {
        let grid_r = r + p.r;
        let grid_c = c + p.c;
        let idx = (grid_r as usize) * width + (grid_c as usize);
        grid[idx] = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_solve_example() {
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
        
        assert_eq!(part1(&input), Ok(1));
    }

    #[test]
    fn test_larger_example() {
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
        
        // 4x4 (Shape 4 x2) -> Yes (1)
        // 12x5 (0x1, 2x1, 4x2, 5x2) -> Yes (2)
        // 12x5 (0x1, 2x1, 4x3, 5x2) -> No (3)
        // Total 2
        assert_eq!(part1(&input), Ok(2));
    }
    
    #[test]
    fn test_simple_fit() {
        // 1x1 box, 1x1 region
        let input = vec![
            "0:".to_string(),
            "#".to_string(),
            "".to_string(),
            "1x1: 1".to_string(),
        ];
        assert_eq!(part1(&input), Ok(1));
    }
    
    #[test]
    fn test_rotation_fit() {
        // 2x1 region, shape is 1x2 (#\n#)
        let input = vec![
            "0:".to_string(),
            "#".to_string(),
            "#".to_string(),
            "".to_string(),
            "2x1: 1".to_string(),
        ];
        assert_eq!(part1(&input), Ok(1));
    }

    #[test]
    fn test_fail_fit() {
        // 1x1 region, shape 2x1
        let input = vec![
            "0:".to_string(),
            "##".to_string(),
            "".to_string(),
            "1x1: 1".to_string(),
        ];
        assert_eq!(part1(&input), Ok(0));
    }

    #[test]
    fn test_shape_with_hole() {
        // Shape O with hole
        // ###
        // #.#
        // ###
        let input = vec![
            "0:".to_string(),
            "###".to_string(),
            "#.#".to_string(),
            "###".to_string(),
            "".to_string(),
            "3x3: 1".to_string(), // Fits exactly
            "2x2: 1".to_string(), // Too small
        ];
        // 3x3 -> 1
        // 2x2 -> 0
        // Total 1
        assert_eq!(part1(&input), Ok(1));
    }

    #[test]
    fn test_disconnected_shape() {
        // Shape: # . #
        let input = vec![
            "0:".to_string(),
            "#.#".to_string(),
            "".to_string(),
            "3x1: 1".to_string(),
            "2x1: 1".to_string(),
        ];
        // 3x1 -> Fits #.#
        // 2x1 -> Can't fit #.# (width 3)
        // Total 1
        assert_eq!(part1(&input), Ok(1));
    }

    #[test]
    fn test_invalid_input() {
        let input = vec![
            "invalid".to_string(),
        ];
        assert!(part1(&input).is_err());
    }
}
