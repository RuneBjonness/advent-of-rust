use std::fmt::Display;
use std::fs;
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PuzzlePart {
    Silver = 0,
    Gold = 1,
    Both = 2,
}

impl PuzzlePart {
    fn as_str(&self) -> &'static str {
        match self {
            PuzzlePart::Silver => "silver",
            PuzzlePart::Gold => "gold",
            PuzzlePart::Both => "both",
        }
    }
}

pub type SilverFn = fn(&str) -> Box<dyn Display>;
pub type GoldFn = fn(&str) -> Box<dyn Display>;
pub type BothFn = fn(&str) -> (Box<dyn Display>, Box<dyn Display>);

#[derive(Clone)]
pub struct AocPuzzle {
    pub year: u16,
    pub day: u8,
    silver: SilverFn,
    gold: GoldFn,
    both: Option<BothFn>,
    skip_parts: [Option<&'static str>; 3],
}

impl AocPuzzle {
    pub fn new(year: u16, day: u8, silver: SilverFn, gold: GoldFn) -> Self {
        Self {
            year,
            day,
            silver,
            gold,
            both: None,
            skip_parts: [None; 3],
        }
    }

    pub fn with_both(mut self, both: BothFn) -> Self {
        self.both = Some(both);
        self
    }

    pub fn skip(mut self, part: PuzzlePart, reason: &'static str) -> Self {
        self.skip_parts[part as usize] = Some(reason);
        self
    }

    fn default_input_path(&self) -> String {
        format!("./input/{}_{:02}.txt", self.year, self.day)
    }

    pub fn read_input(&self, path: Option<&str>) -> String {
        let default_path = self.default_input_path();
        let path = path.unwrap_or(&default_path);
        fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Failed to read input file: {}", path))
            .trim_end()
            .to_string()
    }

    pub fn solve_part(&self, part: PuzzlePart, input: &str, dry_run: bool) -> f64 {
        let mut result_value: String;
        let mut duration_ms = 0.0;

        if let Some(skip_reason) = self.skip_parts[part as usize] {
            result_value = skip_reason.to_string();
        } else {
            let start = Instant::now();

            if dry_run {
                result_value = String::new();
            } else {
                match part {
                    PuzzlePart::Silver => {
                        result_value = (self.silver)(input).to_string();
                    }
                    PuzzlePart::Gold => {
                        result_value = (self.gold)(input).to_string();
                    }
                    PuzzlePart::Both => {
                        let (silver_result, gold_result) = if let Some(both) = self.both {
                            both(input)
                        } else {
                            ((self.silver)(input), (self.gold)(input))
                        };
                        result_value = format!("{}\n{:>35}{}", silver_result, "", gold_result);
                    }
                }
            }

            duration_ms = start.elapsed().as_secs_f64() * 1000.0;

            if result_value.is_empty() {
                result_value = "Not solved".to_string();
            }
        }

        let duration_str = if duration_ms > 0.0 {
            format!("{:.1} ms", duration_ms)
        } else {
            "--".to_string()
        };

        println!(
            "{} | {:>2} | {:<7} | {:>10} | {}",
            self.year,
            self.day,
            part.as_str(),
            duration_str,
            result_value
        );

        duration_ms
    }

    pub fn solve(&self, input: &str, dry_run: bool) -> f64 {
        let mut duration_ms = 0.0;
        duration_ms += self.solve_part(PuzzlePart::Silver, input, dry_run);
        duration_ms += self.solve_part(PuzzlePart::Gold, input, dry_run);
        duration_ms += self.solve_part(PuzzlePart::Both, input, dry_run);
        duration_ms
    }
}
