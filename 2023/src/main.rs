extern crate colored;
extern crate scanf;

mod common;
mod solutions;

use std::{
    env,
    time::Instant,
};
use scanf::scanf;
use common::util::*;

fn main() -> Result<(), String> {
    match get_day_num()? {
        Some(day) => run_day(day),
        None => run_all(),
    }
}

fn run_day(day: u8) -> Result<(), String> {
    println!("\n--- Day {day} ---");

    let start_time = Instant::now();

    solutions::run(day)?;

    println!("Time taken: {:?}", Instant::now() - start_time);
    Ok(())
}

fn run_all() -> Result<(), String> {
    let day_nums = get_all_day_nums()?;

    let start_time = Instant::now();

    println!("======================================");
    println!("  A D V E N T  O F  C O D E  2 0 2 3  ");
    println!("======================================");

    for day_num in day_nums {
        run_day(day_num)?;
    }

    println!();
    println!("=================");
    println!(" F I N I S H E D ");
    println!("=================");

    println!();
    println!("Done in {:?}", Instant::now() - start_time);

    Ok(())
}

fn get_day_num() -> Result<Option<u8>, String> {
    // check sys_args for day num
    let sys_args: Vec<String> = env::args().collect();

    let day_num: String = match sys_args.get(1) {
        Some(x) => x.to_owned(),
        None => {
            let mut user_input = String::new();
            print!("\nEnter the day number: ");
            try_get_ok!(scanf!("{}", user_input));
            user_input
        }
    };

    match day_num.parse() {
        Ok(n) => Ok(Some(n)),
        Err(e) => {
            if day_num == "all" {
                Ok(None)
            } else {
                Err(e.to_string())
            }
        }
    }
}
