extern crate colored;

use crate::common::day_info::*;

mod common;
mod solutions;

use std::{
    env,
    io,
    io::Write,
};

fn main() -> Result<(), String> {
    match get_day_num() {
        Ok(day_num) => match day_num {
            Some(day) => run_day(day),
            None => run_all(),
        },
        Err(e) => Err(e),
    }
}

fn run_day(day: u8) -> Result<(), String> {
    println!("\n--- Day {day} ---");

    let t = std::time::Instant::now();
    match solutions::run(day) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    println!("Time taken: {:?}", std::time::Instant::now() - t);
    Ok(())
}

fn run_all() -> Result<(), String> {

    let day_nums = match get_all_day_nums() {
        Ok(days) => days,
        Err(e) => return Err(e),
    };

    println!("======================================");
    println!("  A D V E N T  O F  C O D E  2 0 2 3  ");
    println!("======================================");
    let start_time = std::time::Instant::now();
    for day_num in day_nums {
        match run_day(day_num) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }
    println!();

    println!("=================");
    println!(" F I N I S H E D ");
    println!("=================");

    println!("Done in {:?}", std::time::Instant::now() - start_time);
    Ok(())
}

fn get_day_num() -> Result<Option<u8>, String> {
    // check sys_args for day num
    let sys_args: Vec<String> = env::args().collect();

    let day_num: String = match sys_args.get(1) {
        Some(x) => x.to_owned(),
        None => {
            let mut buffer = String::new();
            print!("\nEnter the day number: ");
            match io::stdout().flush() {
                Ok(_) => (),
                Err(e) => return Err(e.to_string()),
            };
            match io::stdin().read_line(&mut buffer) {
                Ok(_) => (),
                Err(e) => return Err(e.to_string()),
            };
            buffer.trim().to_string()
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
