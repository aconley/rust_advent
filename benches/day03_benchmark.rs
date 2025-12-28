use criterion::{criterion_group, criterion_main, Criterion};

// Include the binary files as modules
#[allow(dead_code)]
#[path = "../src/bin/antigravity_day03.rs"]
mod antigravity;

#[allow(dead_code)]
#[path = "../src/bin/claude_day03.rs"]
mod claude;

#[allow(dead_code)]
#[path = "../src/bin/cursor_day03.rs"]
mod cursor;

#[allow(dead_code)]
#[path = "../src/bin/gemini_cli_day03.rs"]
mod gemini_cli;

fn benchmark_part1(c: &mut Criterion) {
    let inputs = rust_advent::read_number_grid("03").expect("Failed to read input");
    
    let mut group = c.benchmark_group("Day 3 Part 1");
    
    group.bench_function("antigravity", |b| b.iter(|| antigravity::part1(&inputs)));
    group.bench_function("claude", |b| b.iter(|| claude::part1_parallel(&inputs)));
    group.bench_function("cursor", |b| b.iter(|| cursor::part1(&inputs)));
    group.bench_function("gemini_cli", |b| b.iter(|| gemini_cli::part1(&inputs)));
    
    group.finish();
}

fn benchmark_part2(c: &mut Criterion) {
    let inputs = rust_advent::read_number_grid("03").expect("Failed to read input");
    
    let mut group = c.benchmark_group("Day 3 Part 2");
    
    group.bench_function("antigravity", |b| b.iter(|| antigravity::part2(&inputs)));
    group.bench_function("claude", |b| b.iter(|| claude::part2_parallel(&inputs)));
    group.bench_function("cursor", |b| b.iter(|| cursor::part2(&inputs)));
    group.bench_function("gemini_cli", |b| b.iter(|| gemini_cli::part2(&inputs)));
    
    group.finish();
}

criterion_group!(benches, benchmark_part1, benchmark_part2);
criterion_main!(benches);
