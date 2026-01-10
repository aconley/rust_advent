use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

const INPUT_BASE_PATH: &str = "/Users/alexconley/Programming/Advent Of Code/2025/input";

/// Returns the path to the input file for the given day.
fn get_input_path(day: &str) -> PathBuf {
    let mut path = Path::new(INPUT_BASE_PATH).join(day);
    path.set_extension("txt");
    path
}

/// Reads the input file for the given day as a single string.
pub fn read_file_as_string(day: &str) -> std::io::Result<String> {
    std::fs::read_to_string(get_input_path(day))
}

/// Reads the input file for the given day as a vector of strings, one for each line.
pub fn read_file_as_lines(day: &str) -> std::io::Result<Vec<String>> {
    BufReader::new(File::open(get_input_path(day))?)
        .lines()
        .collect()
}

pub fn read_int_pairs(day: &str) -> std::io::Result<(Vec<i32>, Vec<i32>)> {
    let reader = BufReader::new(File::open(get_input_path(day))?);
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        v1.push(
            parts
                .next()
                .expect("No first number")
                .parse()
                .expect("First number is not an integer"),
        );
        v2.push(
            parts
                .next()
                .expect("No second number")
                .parse()
                .expect("Second number is not an integer"),
        );
    }
    Ok((v1, v2))
}

pub fn read_numbers_with_whitespace(day: &str) -> std::io::Result<Vec<u64>> {
    Ok(read_file_as_string(day)?
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("Value is not an u64"))
        .collect())
}

pub fn read_number_grid_with_whitespace(day: &str) -> std::io::Result<Vec<Vec<i32>>> {
    BufReader::new(File::open(get_input_path(day))?)
        .lines()
        .map(|line| {
            Ok(line?
                .split_whitespace()
                .map(|s| s.parse::<i32>().expect("Value is not an i32"))
                .collect::<Vec<i32>>())
        })
        .collect()
}

pub fn read_ascii_grid(day: &str) -> std::io::Result<Vec<Vec<u8>>> {
    BufReader::new(File::open(get_input_path(day))?)
        .lines()
        .map(|line| Ok(line?.as_bytes().to_vec()))
        .collect()
}

pub fn parse_to_number_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                .collect()
        })
        .filter(|line: &Vec<u8>| !line.is_empty())
        .collect()
}

pub fn read_number_grid(day: &str) -> std::io::Result<Vec<Vec<u8>>> {
    Ok(parse_to_number_grid(&read_file_as_string(day)?))
}

#[derive(Debug, PartialEq)]
pub struct RangeData {
    pub ranges: Vec<(isize, isize)>,
    pub values: Vec<isize>,
}

fn parse_range_data(input: &str) -> Result<RangeData, String> {
    let parts: Vec<&str> = input
        .split("\n\n")
        .filter(|s| !s.trim().is_empty())
        .collect();
    if parts.len() != 2 {
        return Err("Input must have two sections separated by empty lines".to_string());
    }

    let ranges_str = parts[0].trim();
    let values_str = parts[1].trim();

    let ranges: Result<Vec<(isize, isize)>, String> = ranges_str
        .lines()
        .map(|line| {
            let mut split = line.split('-');
            let start: isize = split
                .next()
                .ok_or_else(|| "Missing start of range".to_string())?
                .parse()
                .map_err(|_| format!("Invalid range start in {}", line))?;
            let end: isize = split
                .next()
                .ok_or_else(|| "Missing end of range".to_string())?
                .parse()
                .map_err(|_| format!("Invalid range end in {}", line))?;
            if start > end {
                return Err(format!("Invalid range: start > end ({}- {})", start, end));
            }
            Ok((start, end))
        })
        .collect();
    let ranges = ranges?;

    let values: Result<Vec<isize>, String> = values_str
        .lines()
        .map(|line| {
            line.trim()
                .parse()
                .map_err(|_| format!("Invalid value {}", line))
        })
        .collect();
    let values = values?;

    Ok(RangeData { ranges, values })
}

pub fn read_range_data(day: &str) -> std::io::Result<RangeData> {
    let content = read_file_as_string(day)?;
    parse_range_data(&content).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_range_data() {
        let input = "1-4\n7-11\n\n2\n9";
        let expected = RangeData {
            ranges: vec![(1, 4), (7, 11)],
            values: vec![2, 9],
        };
        assert_eq!(parse_range_data(input).unwrap(), expected);
    }

    #[test]
    fn test_parse_range_data_with_extra_newline() {
        let input = "1-4\n7-11\n\n\n2\n9";
        let expected = RangeData {
            ranges: vec![(1, 4), (7, 11)],
            values: vec![2, 9],
        };
        assert_eq!(parse_range_data(input).unwrap(), expected);
    }

    #[test]
    fn test_parse_invalid_range() {
        let input = "5-4\n\n1";
        assert!(parse_range_data(input).is_err());
    }
}
