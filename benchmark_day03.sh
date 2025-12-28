#!/bin/bash
set -e

# Build the binaries
echo "Building binaries..."
cargo build --release --bin antigravity_day03 --bin claude_day03 --bin cursor_day03 --bin gemini_cli_day03

# Run hyperfine
echo "Running benchmarks..."
if command -v hyperfine &> /dev/null; then
    hyperfine -N --warmup 3 --export-markdown day03_benchmark.md \
        'target/release/antigravity_day03' \
        'target/release/claude_day03' \
        'target/release/cursor_day03' \
        'target/release/gemini_cli_day03'
    
    echo "Benchmark results:"
    cat day03_benchmark.md
else
    echo "Error: hyperfine is not installed. Please install it with 'cargo install hyperfine' or your package manager."
    exit 1
fi
