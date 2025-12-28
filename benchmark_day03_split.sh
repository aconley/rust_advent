#!/bin/bash
set -e

# Run hyperfine for Part 1
echo "Running Part 1 benchmarks..."
hyperfine -N --warmup 3 --export-markdown day03_part1_benchmark.md \
    'target/release/antigravity_day03 part1' \
    'target/release/claude_day03 part1' \
    'target/release/cursor_day03 part1' \
    'target/release/gemini_cli_day03 part1'

echo "Part 1 Benchmark results:"
cat day03_part1_benchmark.md

# Run hyperfine for Part 2
echo "Running Part 2 benchmarks..."
hyperfine -N --warmup 3 --export-markdown day03_part2_benchmark.md \
    'target/release/antigravity_day03 part2' \
    'target/release/claude_day03 part2' \
    'target/release/cursor_day03 part2' \
    'target/release/gemini_cli_day03 part2'

echo "Part 2 Benchmark results:"
cat day03_part2_benchmark.md
