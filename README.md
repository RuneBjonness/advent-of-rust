# advent-of-rust

Rust port of [advent-of-code](https://github.com/RuneBjonness/advent-of-code) (TypeScript).

This repository is an experiment in porting a codebase entirely using AI coding agents, with no manual code writing. It also serves as a benchmark for comparing Rust and TypeScript performance on identical algorithms.

```
> cargo run -- [options]
```

## Options:

| Short | Long          | Description                                                                              |
| ----- | ------------- | ---------------------------------------------------------------------------------------- |
| -y    | --year <year> | Specify the year (e.g., 2025)                                                            |
| -d    | --day <day>   | Specify the day (1-25)                                                                   |
|       | --silver      | Run part 1 individually (the silver star)                                                |
|       | --gold        | Run part 2 individually (the gold star)                                                  |
|       | --both        | Run both parts combined                                                                  |
| -p    | --path <path> | Specify a custom input file path (requires --year and --day to be set)                   |
|       | --dryrun      | Run the solution in dry run mode (no actual computation). Useful for measuring overhead. |
| -s    | --summary     | Print a summary                                                                          |

## Examples:

- Run all solutions for every year and day available:

```
  > cargo run --release
```

- Run solutions for silver, gold and both combined for day 1, 2025 - using custom input file

```
  > cargo run --release -- -y 2025 -d 1 --path ./input/2025_01.txt
```

- Run silver solution for day 24, 2024

```
  > cargo run --release -- -y 2024 -d 24 --silver
```

- Run every solution for 2025 and print a summary with total calculation time

```
  > cargo run --release -- -y 2025 --both -s
```
