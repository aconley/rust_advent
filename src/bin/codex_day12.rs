fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_file_as_lines("12")?;
    println!("Part 1: {}", part1(&inputs));
    Ok(())
}

fn part1(input: &[String]) -> u32 {
    let (shapes, regions) = match parse_input(input) {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("{}", format_parse_error(&err));
            return 0;
        }
    };
    let mut count = 0u32;
    for region in regions {
        if can_fit_region(&region, &shapes) {
            count += 1;
        }
    }
    count
}

#[derive(Clone)]
struct Shape {
    area: usize,
    orientations: Vec<Orientation>,
}

#[derive(Clone)]
struct Orientation {
    width: usize,
    height: usize,
    row_masks: Vec<u64>,
    area: usize,
}

struct Region {
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

enum ParseError {
    InvalidShapeHeader(usize),
    MissingShapeHeader(usize),
    InvalidRegionHeader(usize),
    InvalidDimensions(usize),
    InvalidCount(usize),
}

#[derive(Clone)]
struct Placement {
    rows: Vec<(usize, u64)>,
    area: usize,
}

struct TypeData {
    area: usize,
    placements: Vec<Placement>,
    covers: Vec<Vec<usize>>,
}

fn parse_input(input: &[String]) -> Result<(Vec<Shape>, Vec<Region>), ParseError> {
    let mut index = 0usize;
    let mut shapes: Vec<Option<Shape>> = Vec::new();

    while index < input.len() {
        let line = input[index].trim();
        if line.is_empty() {
            index += 1;
            continue;
        }
        if is_region_line(line) {
            break;
        }
        let (id_str, _) = line
            .split_once(':')
            .ok_or(ParseError::MissingShapeHeader(index + 1))?;
        let id: usize = id_str
            .trim()
            .parse()
            .map_err(|_| ParseError::InvalidShapeHeader(index + 1))?;
        index += 1;

        let mut grid: Vec<&str> = Vec::new();
        while index < input.len() {
            let row = input[index].trim_end();
            if row.trim().is_empty() {
                index += 1;
                break;
            }
            if is_region_line(row.trim()) {
                break;
            }
            grid.push(row);
            index += 1;
        }

        let shape = build_shape(&grid);
        if shapes.len() <= id {
            shapes.resize_with(id + 1, || None);
        }
        shapes[id] = Some(shape);
    }

    let mut final_shapes = Vec::with_capacity(shapes.len());
    for opt in shapes {
        final_shapes.push(opt.unwrap_or_else(|| Shape {
            area: 0,
            orientations: Vec::new(),
        }));
    }

    let mut regions = Vec::new();
    while index < input.len() {
        let line = input[index].trim();
        index += 1;
        if line.is_empty() {
            continue;
        }
        if !is_region_line(line) {
            continue;
        }
        let (dims, rest) = line
            .split_once(':')
            .ok_or(ParseError::InvalidRegionHeader(index))?;
        let (w_str, h_str) = dims
            .split_once('x')
            .ok_or(ParseError::InvalidDimensions(index))?;
        let width: usize = w_str
            .trim()
            .parse()
            .map_err(|_| ParseError::InvalidDimensions(index))?;
        let height: usize = h_str
            .trim()
            .parse()
            .map_err(|_| ParseError::InvalidDimensions(index))?;
        let mut counts = Vec::new();
        for token in rest.split_whitespace() {
            let value = token
                .parse::<usize>()
                .map_err(|_| ParseError::InvalidCount(index))?;
            counts.push(value);
        }
        regions.push(Region {
            width,
            height,
            counts,
        });
    }

    Ok((final_shapes, regions))
}

fn format_parse_error(err: &ParseError) -> String {
    match *err {
        ParseError::InvalidShapeHeader(line) => {
            format!("Parse error on line {line}: invalid shape header")
        }
        ParseError::MissingShapeHeader(line) => {
            format!("Parse error on line {line}: missing shape header ':'")
        }
        ParseError::InvalidRegionHeader(line) => {
            format!("Parse error on line {line}: invalid region header")
        }
        ParseError::InvalidDimensions(line) => {
            format!("Parse error on line {line}: invalid region dimensions")
        }
        ParseError::InvalidCount(line) => {
            format!("Parse error on line {line}: invalid region count value")
        }
    }
}

fn is_region_line(line: &str) -> bool {
    let (left, _) = match line.split_once(':') {
        Some(parts) => parts,
        None => return false,
    };
    let (w, h) = match left.split_once('x') {
        Some(parts) => parts,
        None => return false,
    };
    !w.trim().is_empty()
        && !h.trim().is_empty()
        && w.trim().chars().all(|c| c.is_ascii_digit())
        && h.trim().chars().all(|c| c.is_ascii_digit())
}

fn build_shape(grid: &[&str]) -> Shape {
    let mut points: Vec<(i32, i32)> = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch == '#' {
                points.push((x as i32, y as i32));
            }
        }
    }

    if points.is_empty() {
        return Shape {
            area: 0,
            orientations: Vec::new(),
        };
    }

    let orientations = generate_orientations(&points);
    Shape {
        area: points.len(),
        orientations,
    }
}

fn generate_orientations(points: &[(i32, i32)]) -> Vec<Orientation> {
    use std::collections::HashSet;
    let mut seen: HashSet<String> = HashSet::new();
    let mut orientations = Vec::new();

    for rot in 0..4 {
        for flip in [false, true] {
            let mut transformed: Vec<(i32, i32)> = points
                .iter()
                .map(|&(x, y)| {
                    let (rx, ry) = match rot {
                        0 => (x, y),
                        1 => (y, -x),
                        2 => (-x, -y),
                        3 => (-y, x),
                        _ => unreachable!(),
                    };
                    let rx = if flip { -rx } else { rx };
                    (rx, ry)
                })
                .collect();

            let min_x = transformed.iter().map(|p| p.0).min().unwrap();
            let min_y = transformed.iter().map(|p| p.1).min().unwrap();
            for point in &mut transformed {
                point.0 -= min_x;
                point.1 -= min_y;
            }
            transformed.sort();

            let mut key = String::new();
            for (x, y) in &transformed {
                key.push_str(&format!("{x},{y};"));
            }
            if seen.insert(key) {
                orientations.push(orientation_from_points(&transformed));
            }
        }
    }

    orientations
}

fn orientation_from_points(points: &[(i32, i32)]) -> Orientation {
    let mut max_x = 0i32;
    let mut max_y = 0i32;
    for &(x, y) in points {
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
    }
    let width = (max_x + 1) as usize;
    let height = (max_y + 1) as usize;
    let mut row_masks = vec![0u64; height];
    for &(x, y) in points {
        row_masks[y as usize] |= 1u64 << (x as usize);
    }
    Orientation {
        width,
        height,
        row_masks,
        area: points.len(),
    }
}

fn can_fit_region(region: &Region, shapes: &[Shape]) -> bool {
    if region.width > 64 {
        return false;
    }

    if region.counts.len() > shapes.len()
        && region.counts[shapes.len()..].iter().any(|&count| count > 0)
    {
        return false;
    }

    let mut counts = vec![0usize; shapes.len()];
    for i in 0..counts.len() {
        if i < region.counts.len() {
            counts[i] = region.counts[i];
        }
    }

    let type_data = build_type_data(region, shapes);
    let total_needed: usize = counts
        .iter()
        .enumerate()
        .map(|(i, &count)| count * type_data[i].area)
        .sum();
    if total_needed > region.width * region.height {
        return false;
    }
    for (i, &count) in counts.iter().enumerate() {
        if count > 0 && type_data[i].placements.is_empty() {
            return false;
        }
    }

    let mut occupied = vec![0u64; region.height];
    let free = region.width * region.height;
    let mask_all = if region.width == 64 {
        u64::MAX
    } else {
        (1u64 << region.width) - 1
    };
    dfs(
        &mut occupied,
        &mut counts,
        free,
        &type_data,
        region.width,
        mask_all,
    )
}

fn build_type_data(region: &Region, shapes: &[Shape]) -> Vec<TypeData> {
    let mut data = Vec::with_capacity(shapes.len());
    for shape in shapes {
        let mut placements = Vec::new();
        for orientation in &shape.orientations {
            if orientation.width > region.width || orientation.height > region.height {
                continue;
            }
            for y in 0..=region.height - orientation.height {
                for x in 0..=region.width - orientation.width {
                    let mut rows = Vec::with_capacity(orientation.height);
                    for (dy, rowmask) in orientation.row_masks.iter().enumerate() {
                        let mask = rowmask << x;
                        rows.push((y + dy, mask));
                    }
                    placements.push(Placement {
                        rows,
                        area: orientation.area,
                    });
                }
            }
        }
        let mut covers = vec![Vec::new(); region.width * region.height];
        for (idx, placement) in placements.iter().enumerate() {
            for (row, mask) in &placement.rows {
                let mut bits = *mask;
                while bits != 0 {
                    let b = bits.trailing_zeros() as usize;
                    bits &= bits - 1;
                    covers[row * region.width + b].push(idx);
                }
            }
        }
        data.push(TypeData {
            area: shape.area,
            placements,
            covers,
        });
    }
    data
}

fn dfs(
    occupied: &mut [u64],
    remaining: &mut [usize],
    free: usize,
    type_data: &[TypeData],
    width: usize,
    mask_all: u64,
) -> bool {
    let mut remaining_area = 0usize;
    let mut any_remaining = false;
    for (i, &count) in remaining.iter().enumerate() {
        if count > 0 {
            any_remaining = true;
        }
        remaining_area += count * type_data[i].area;
    }
    if !any_remaining {
        return true;
    }
    if free < remaining_area {
        return false;
    }

    if remaining_area == free {
        let (target_row, target_col) = match find_first_empty(occupied, mask_all) {
            Some(pos) => pos,
            None => return false,
        };
        let target_idx = target_row * width + target_col;

        let mut best_type = None;
        let mut best_count = usize::MAX;
        for i in 0..remaining.len() {
            if remaining[i] == 0 {
                continue;
            }
            let list = &type_data[i].covers[target_idx];
            if list.is_empty() {
                continue;
            }
            if list.len() < best_count {
                best_count = list.len();
                best_type = Some(i);
            }
        }

        let idx = match best_type {
            Some(i) => i,
            None => return false,
        };

        for &pidx in &type_data[idx].covers[target_idx] {
            let placement = &type_data[idx].placements[pidx];
            if !can_place(occupied, placement) {
                continue;
            }
            apply_place(occupied, placement);
            remaining[idx] -= 1;
            let new_free = free - placement.area;
            if dfs(occupied, remaining, new_free, type_data, width, mask_all) {
                return true;
            }
            remaining[idx] += 1;
            remove_place(occupied, placement);
        }
        return false;
    }

    let mut best_type = None;
    let mut best_count = usize::MAX;
    for i in 0..remaining.len() {
        if remaining[i] == 0 {
            continue;
        }
        let mut count = 0usize;
        for placement in &type_data[i].placements {
            if can_place(occupied, placement) {
                count += 1;
                if count >= best_count {
                    break;
                }
            }
        }
        if count == 0 {
            return false;
        }
        if count < best_count {
            best_count = count;
            best_type = Some(i);
        }
    }

    let idx = best_type.expect("remaining pieces");
    for placement in &type_data[idx].placements {
        if !can_place(occupied, placement) {
            continue;
        }
        apply_place(occupied, placement);
        remaining[idx] -= 1;
        let new_free = free - placement.area;
        if dfs(occupied, remaining, new_free, type_data, width, mask_all) {
            return true;
        }
        remaining[idx] += 1;
        remove_place(occupied, placement);
    }
    false
}

fn can_place(occupied: &[u64], placement: &Placement) -> bool {
    for (row, mask) in &placement.rows {
        if (occupied[*row] & *mask) != 0 {
            return false;
        }
    }
    true
}

fn apply_place(occupied: &mut [u64], placement: &Placement) {
    for (row, mask) in &placement.rows {
        occupied[*row] |= *mask;
    }
}

fn remove_place(occupied: &mut [u64], placement: &Placement) {
    for (row, mask) in &placement.rows {
        occupied[*row] &= !*mask;
    }
}

fn find_first_empty(occupied: &[u64], mask_all: u64) -> Option<(usize, usize)> {
    for (row, &mask) in occupied.iter().enumerate() {
        let free = !mask & mask_all;
        if free != 0 {
            let col = free.trailing_zeros() as usize;
            return Some((row, col));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(input: &str) -> u32 {
        let lines = input.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        part1(&lines)
    }

    #[test]
    fn example_from_prompt() {
        let input = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";
        assert_eq!(run(input), 2);
    }

    #[test]
    fn single_cell_shapes() {
        let input = "\
0:
#

2x2: 3
2x2: 5
";
        assert_eq!(run(input), 1);
    }

    #[test]
    fn rotation_allows_fit() {
        let input = "\
0:
##

1x2: 1
2x1: 1
";
        assert_eq!(run(input), 2);
    }

    #[test]
    fn missing_shape_type_counts() {
        let input = "\
0:
#

2x2: 0 1
2x2: 0 0
";
        assert_eq!(run(input), 1);
    }

    #[test]
    fn shape_too_large() {
        let input = "\
0:
##
##

1x3: 1
";
        assert_eq!(run(input), 0);
    }
}
