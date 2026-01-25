mod aoc_puzzle;
mod puzzle_collection;
mod y2015;
mod y2016;
mod y2017;

use aoc_puzzle::PuzzlePart;
use clap::Parser;
use puzzle_collection::get_puzzles;

#[derive(Parser)]
#[command(name = "aoc")]
#[command(about = "Advent of Code solutions")]
struct Args {
    /// Specify the year (e.g., 2025)
    #[arg(short = 'y', long)]
    year: Option<u16>,

    /// Specify the day (1-25)
    #[arg(short = 'd', long)]
    day: Option<u8>,

    /// Run part 1 individually (the silver star)
    #[arg(long)]
    silver: bool,

    /// Run part 2 individually (the gold star)
    #[arg(long)]
    gold: bool,

    /// Run both parts combined
    #[arg(long)]
    both: bool,

    /// Specify a custom input file path (requires --year and --day to be set)
    #[arg(short = 'p', long)]
    path: Option<String>,

    /// Run the solution in dry run mode (no actual computation)
    #[arg(long)]
    dryrun: bool,

    /// Print a summary
    #[arg(short = 's', long)]
    summary: bool,
}

fn main() {
    let args = Args::parse();

    let year = parse_year_filter(args.year);
    let day = parse_day_filter(args.day);

    let puzzles = get_puzzles(year, day);

    if puzzles.is_empty() {
        println!("No puzzles found");
        return;
    }

    let mut total_duration_ms = 0.0;
    let path = if puzzles.len() == 1 {
        args.path.as_deref()
    } else {
        None
    };

    for puzzle in &puzzles {
        let input = puzzle.read_input(path);

        if args.silver {
            total_duration_ms += puzzle.solve_part(PuzzlePart::Silver, &input, args.dryrun);
        }
        if args.gold {
            total_duration_ms += puzzle.solve_part(PuzzlePart::Gold, &input, args.dryrun);
        }
        if args.both {
            total_duration_ms += puzzle.solve_part(PuzzlePart::Both, &input, args.dryrun);
        }
        if !args.silver && !args.gold && !args.both {
            total_duration_ms += puzzle.solve(&input, args.dryrun);
        }
    }

    if args.summary {
        println!("----------------------------------------------------");
        println!("Total duration: {:.1} ms", total_duration_ms);
    }
}

fn parse_year_filter(year: Option<u16>) -> Option<u16> {
    match year {
        Some(y) if y >= 2015 => Some(y),
        Some(_) => panic!("Invalid year filter"),
        None => None,
    }
}

fn parse_day_filter(day: Option<u8>) -> Option<u8> {
    match day {
        Some(d) if (1..=25).contains(&d) => Some(d),
        Some(_) => panic!("Invalid day filter"),
        None => None,
    }
}
