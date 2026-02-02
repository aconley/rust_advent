fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_file_as_lines("11")?;
    println!("Part 1: {}", part1("you", "out", &inputs));
    println!("Part 2: {}", part2("svr", "out", &["dac", "fft"], &inputs));
    Ok(())
}

/// Part 1: Beam splitter
fn part1(_start_vertex: &str, _target_vertex: &str, _input: &[String]) -> u64 {
    todo!("Implement");
}

fn part2<R: AsRef<str>>(
    _start_vertex: &str,
    _target_vertex: &str,
    _required_vertices: &[R],
    _input: &[String],
) -> u64 {
    todo!("Implement");
}
