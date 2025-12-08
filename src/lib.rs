use std::collections::{HashMap, HashSet};
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

pub struct RulesAndUpdates {
    // Precedence of the rules.  before[x] is the set of pages that must be printed before x.
    pub before: HashMap<u16, HashSet<u16>>,
    // The pages.
    pub pages: Vec<Vec<u16>>,
}

pub fn read_rules_and_updates(day: &str) -> std::io::Result<RulesAndUpdates> {
    let input = std::fs::read_to_string(get_input_path(day))?;
    let (raw_rules, raw_pages) = input.split_once("\n\n").unwrap();
    let mut before = HashMap::<u16, HashSet<u16>>::new();
    for line in raw_rules.lines() {
        let (a, b) = line.split_once('|').expect("line did not contain |");
        before
            .entry(b.parse::<u16>().expect("Couldn't parse b as integer"))
            .or_default()
            .insert(a.parse::<u16>().expect("Couldn't parse a as integer"));
    }
    let pages = raw_pages
        .lines()
        .map(|line| {
            line.split(',')
                .map(|w| w.parse::<u16>().expect("Couldn't parse page as integer"))
                .collect::<Vec<_>>()
        })
        .collect();
    Ok(RulesAndUpdates { before, pages })
}
