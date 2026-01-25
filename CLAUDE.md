# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an exact Rust port of https://github.com/RuneBjonness/advent-of-code (TypeScript). The CLI arguments and output format must match exactly.

## Build Commands

```bash
cargo build                    # Build debug
cargo build --release          # Build with optimizations
cargo run -- [options]         # Run with options
cargo test                     # Run all tests
cargo test day01               # Run tests for specific day
cargo fmt                      # Format code
cargo clippy                   # Lint
```

## CLI Interface (must match TypeScript version exactly)

```
aoc [options]

Options:
  -y, --year <year>    Specify the year (e.g., 2025)
  -d, --day <day>      Specify the day (1-25)
      --silver         Run part 1 individually
      --gold           Run part 2 individually
      --both           Run both parts combined
  -p, --path <path>    Custom input file path (requires --year and --day)
      --dryrun         Run without computation (measure overhead)
  -s, --summary        Print total duration summary
```

## Output Format (must match exactly)

```
YEAR |  D | PART    |   DURATION | RESULT
2015 |  1 | silver  |     0.1 ms | 232
2015 |  1 | gold    |     0.2 ms | 1783
2015 |  1 | both    |     0.3 ms | 232
                                   1783
```

- Day is right-aligned, 2 chars
- Part is left-aligned, 7 chars
- Duration is right-aligned, 10 chars with " ms" suffix
- "both" result shows silver on first line, gold indented 35 chars on next line
- Summary line: `Total duration: X.X ms`

## Project Structure

```
src/
  main.rs                      # CLI entry point
  aoc_puzzle.rs                # Puzzle trait/struct
  puzzle_collection.rs         # Collects all puzzles
  2015/
    mod.rs                     # exports puzzles2015 vec
    day_01/
      mod.rs                   # solution with silver(), gold(), optional both()
      solution_test.rs         # tests
  ...
input/
  2015_01.txt                  # Input file naming: YYYY_DD.txt
```

## Solution Interface

Each day exports an `AocPuzzle` with:
- `silver(input: &str) -> impl Display` - Part 1 solution
- `gold(input: &str) -> impl Display` - Part 2 solution
- `both(input: &str) -> (impl Display, impl Display)` - Optional optimized combined solution
- `skip(part, reason)` - Mark slow/unsolved parts to skip with a message

Default input path: `./input/{year}_{day:02}.txt`

## Testing Pattern

Tests should verify both example inputs and actual puzzle inputs:
```rust
#[test]
fn silver_test_input() {
    assert_eq!(silver(")())())"), -3);
}

#[test]
fn silver_actual_input() {
    let input = read_input(2015, 1);
    assert_eq!(silver(&input), 232);
}
```
