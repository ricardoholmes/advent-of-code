use crate::{run_day, common::times_taken::TimesTaken};

mod day_1;
mod day_2;

pub fn run(day: u8) -> Result<(String, String, TimesTaken), String> {
    match day {
        1 => run_day!(day_1, 1),
        2 => run_day!(day_2, 2),
        _ => Err(String::from("Invalid day number")),
    }
}
