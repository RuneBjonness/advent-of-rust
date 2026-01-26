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
cargo test y2015::day_01       # Run tests for specific year and day
cargo fmt                      # Format code
cargo clippy                   # Lint
```

## GitHub Actions

The project uses GitHub Actions for CI/CD:

- **Build workflow** (`.github/workflows/build.yml`): Builds a Linux release binary on pushes to main branch
  - Runs on `ubuntu-latest`
  - Builds with `cargo build --release`
  - Uploads the release binary as an artifact

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
  aoc_puzzle.rs                # AocPuzzle struct and runner
  puzzle_collection.rs         # Collects all puzzles
  y2015/
    mod.rs                     # exports puzzles() returning &'static Vec<AocPuzzle>
    day_01/
      mod.rs                   # solution with silver(), gold(), puzzle(), and tests
  y2016/                       # (when added)
    ...
input/
  2015_01.txt                  # Input file naming: YYYY_DD.txt
```

## Solution Interface

Each day module exports:
- `silver(input: &str) -> Box<dyn Display>` - Part 1 solution
- `gold(input: &str) -> Box<dyn Display>` - Part 2 solution
- `puzzle() -> AocPuzzle` - Returns configured puzzle

Optional features:
- `.with_both(both_fn)` - Add optimized combined solution
- `.skip(PuzzlePart::Gold, "reason")` - Mark parts to skip

Default input path: `./input/{year}_{day:02}.txt`

## Testing Pattern

Tests are included in the same file as the solution using `#[cfg(test)]`:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn read_input(year: u16, day: u8) -> String {
        fs::read_to_string(format!("./input/{}_{:02}.txt", year, day))
            .unwrap()
            .trim_end()
            .to_string()
    }

    #[test]
    fn silver_test_input() {
        assert_eq!(silver(")())())").to_string(), "-3");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2015, 1);
        assert_eq!(silver(&input).to_string(), "232");
    }
}
```

---

## Quick Start: Port a Puzzle

Use the custom command to port a puzzle automatically:

```
/port-puzzle 2015 2
```

This will fetch the TypeScript source, create the Rust module, port the solution, and run tests.

---

## Manual Step-by-Step Porting Guide

Follow these steps to port a puzzle from the TypeScript version.

### Step 1: Locate the TypeScript Source

Find the original solution at: `https://github.com/RuneBjonness/advent-of-code`

Path pattern: `src/{year}/day-{DD}/solution.ts`

Example: `src/2015/day-02/solution.ts`

### Step 2: Copy the Input File

1. Copy the input file from the TypeScript repo: `input/{year}_{DD}.txt`
2. Place it in: `./input/{year}_{DD}.txt`

Example: `input/2015_02.txt`

### Step 3: Create the Day Module

Create directory and file:
```
src/y{year}/day_{DD}/mod.rs
```

Use this template:
```rust
use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    // TODO: Port silver solution from TypeScript
    Box::new(0)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    // TODO: Port gold solution from TypeScript
    Box::new(0)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new({YEAR}, {DAY}, silver, gold)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn read_input(year: u16, day: u8) -> String {
        fs::read_to_string(format!("./input/{}_{:02}.txt", year, day))
            .unwrap()
            .trim_end()
            .to_string()
    }

    #[test]
    fn silver_test_input() {
        // TODO: Add test with example input from puzzle description
        assert_eq!(silver("example").to_string(), "expected");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input({YEAR}, {DAY});
        assert_eq!(silver(&input).to_string(), "{EXPECTED_SILVER}");
    }

    #[test]
    fn gold_test_input() {
        // TODO: Add test with example input from puzzle description
        assert_eq!(gold("example").to_string(), "expected");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input({YEAR}, {DAY});
        assert_eq!(gold(&input).to_string(), "{EXPECTED_GOLD}");
    }
}
```

### Step 4: Register the Module

Edit `src/y{year}/mod.rs`:

1. Add module declaration at top:
   ```rust
   mod day_{DD};
   ```

2. Add puzzle to the vector:
   ```rust
   static PUZZLES: LazyLock<Vec<AocPuzzle>> = LazyLock::new(|| {
       vec![
           day_01::puzzle(),
           day_02::puzzle(),  // Add new entry
       ]
   });
   ```

### Step 5: Port the Solution Logic

Translate TypeScript to Rust:

| TypeScript | Rust |
|------------|------|
| `input.split('\n')` | `input.lines()` |
| `parseInt(s)` | `s.parse::<i32>().unwrap()` |
| `s.split(' ')` | `s.split_whitespace()` or `s.split(' ')` |
| `arr.map(fn)` | `iter.map(fn).collect()` |
| `arr.filter(fn)` | `iter.filter(fn).collect()` |
| `arr.reduce(fn, init)` | `iter.fold(init, fn)` |
| `Math.min(a, b)` | `a.min(b)` or `std::cmp::min(a, b)` |
| `[...arr]` (spread) | `arr.clone()` |
| `arr.includes(x)` | `arr.contains(&x)` |
| `new Set()` | `HashSet::new()` |
| `new Map()` | `HashMap::new()` |

### Step 6: Run Tests

```bash
# Run specific year/day tests
cargo test y{YEAR}::day_{DD}

# Run all tests
cargo test

# Check formatting and lints
cargo fmt && cargo clippy
```

### Step 7: Verify Output

```bash
# Run the puzzle
cargo run -- -y {YEAR} -d {DAY}

# Compare with TypeScript output (should match exactly)
```

### Step 8: Optional - Add Combined Solution

If the TypeScript version has an optimized `both()` function:

```rust
pub fn both(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    // Optimized solution that solves both parts together
    (Box::new(silver_result), Box::new(gold_result))
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new({YEAR}, {DAY}, silver, gold)
        .with_both(both)
}
```

### Adding a New Year

When starting a new year (e.g., 2016):

1. Create `src/y2016/mod.rs`:
   ```rust
   mod day_01;

   use crate::aoc_puzzle::AocPuzzle;
   use std::sync::LazyLock;

   static PUZZLES: LazyLock<Vec<AocPuzzle>> = LazyLock::new(|| {
       vec![
           day_01::puzzle(),
       ]
   });

   pub fn puzzles() -> &'static Vec<AocPuzzle> {
       &PUZZLES
   }
   ```

2. Add to `src/main.rs`:
   ```rust
   mod y2016;
   ```

3. Update `src/puzzle_collection.rs` to include the new year's puzzles
