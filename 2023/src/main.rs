extern crate colored;
extern crate scanf;

mod common;
mod solutions;

use std::{env, time::Instant};

use common::util::*;
use scanf::scanf;

use crate::common::times_taken::TimesTaken;

#[derive(PartialEq)]
enum Mode {
    Day(u8),
    All,
    Benchmark(u8),
    BenchmarkAll,
}

struct Settings {
    mode: Mode,
    quiet: bool,
    verbose: bool,
}

const BENCHMARK_REPETITIONS: usize = 1000;
const PROGRESS_BAR_LENGTH: usize = 50;

fn main() -> Result<(), String> {
    let settings = get_user_settings()?;

    match settings.mode {
        Mode::Day(day) => run_day(day, &settings),
        Mode::All => run_all(&settings),
        Mode::Benchmark(_) => benchmark(settings.mode),
        Mode::BenchmarkAll => benchmark(settings.mode),
    }
}

fn run_day(day: u8, settings: &Settings) -> Result<(), String> {
    println!("\n--- Day {day} ---");

    let (part_one, part_two, times) = solutions::run(day)?;

    if !settings.quiet {
        println!("Part one: {}", part_one);
        println!("Part two: {}", part_two);
    }

    if settings.verbose {
        print_times_nicely(times);
    }
    else {
        println!("Time taken: {}", color_time_taken(times.total));
    }

    Ok(())
}

fn run_all(settings: &Settings) -> Result<(), String> {
    let day_nums = get_all_day_nums()?;

    let start_time = Instant::now();

    println!("======================================");
    println!("  A D V E N T  O F  C O D E  2 0 2 3  ");
    println!("======================================");

    for day_num in day_nums {
        run_day(day_num, settings)?;
    }

    println!();
    println!("=================");
    println!(" F I N I S H E D ");
    println!("=================");

    println!();
    println!("Done in {:?}", Instant::now() - start_time);

    Ok(())
}

fn benchmark(mode: Mode) -> Result<(), String> {
    let day_nums = get_all_day_nums()?;

    let mut total_times = TimesTaken::ZERO;

    match mode {
        Mode::Benchmark(day) => println!("\n--- Benchmarking Day {day} ---\n"),
        Mode::BenchmarkAll => println!("\n--- Benchmarking All Days ---\n"),
        _ => return Err(String::from("Invalid mode."))
    }

    print!("[{}]\r", "-".repeat(PROGRESS_BAR_LENGTH));
    for i in 0..BENCHMARK_REPETITIONS {
        let (_, _, times) = match mode {
            Mode::Benchmark(day) => solutions::run(day)?,
            Mode::BenchmarkAll => {
                let mut total_times = TimesTaken::ZERO;
                for day in &day_nums {
                    let (_, _, times) = solutions::run(*day)?;
                    total_times += times;
                }
                (0, 0, total_times)
            },
            _ => return Err(String::from("Invalid mode.")),
        };

        total_times += times;

        let bar_completion = PROGRESS_BAR_LENGTH * i / BENCHMARK_REPETITIONS;
        print!("[{}{}]\r", "#".repeat(bar_completion), "-".repeat(PROGRESS_BAR_LENGTH - bar_completion));
    }
    println!("[{}]\n", "#".repeat(PROGRESS_BAR_LENGTH));

    let mut average_times = TimesTaken::divide(&total_times, BENCHMARK_REPETITIONS as u32);

    if mode == Mode::BenchmarkAll {
        average_times = TimesTaken::divide(&average_times, day_nums.len() as u32);
    }

    println!("Total times taken:");
    print_times_nicely(total_times);
    println!();
    println!("Average times taken:");
    print_times_nicely(average_times);
    println!();

    Ok(())
}

fn get_user_settings() -> Result<Settings, String> {
    let sys_args: Vec<String> = env::args().collect();
    let arg_count = sys_args.len();

    if arg_count <= 1 {
        let day_num: String = match sys_args.get(1) {
            Some(x) => x.to_owned(),
            None => {
                let mut user_input = String::new();
                print!("\nEnter the day number: ");
                try_get_ok!(scanf!("{}", user_input));
                user_input
            }
        };
    
        let mode = match day_num.parse() {
            Ok(n) => Mode::Day(n),
            Err(e) => {
                if day_num == "all" {
                    Mode::All
                } else {
                    return Err(e.to_string());
                }
            }
        };

        let settings = Settings {
            mode,
            quiet: false,
            verbose: false,
        };

        return Ok(settings);
    }

    let mode = match sys_args[1].to_ascii_lowercase().as_str() {
        "all" => Mode::All,
        "bench" => if arg_count >= 3 {
            if let Ok(day_num) = sys_args[2].parse() {
                Mode::Benchmark(day_num)
            }
            else {
                Mode::BenchmarkAll
            }
        } else {
            Mode::BenchmarkAll
        },
        value => match value.parse() {
            Ok(day_num) => Mode::Day(day_num),
            _ => return Err(format!("Failed to parse day number given ({value}).")),
        }
    };

    let mut quiet = false;
    let mut verbose = false;
    for arg in sys_args {
        match arg.as_str() {
            "-q" => quiet = true,
            "-v" => verbose = true,
            _ => (),
        }
    }

    let settings = Settings {
        mode,
        quiet,
        verbose,
    };

    Ok(settings)
}
