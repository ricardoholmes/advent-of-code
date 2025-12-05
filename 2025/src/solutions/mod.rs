use crate::{run_day, common::times_taken::TimesTaken};

mod day_1;

pub fn run(day: u8) -> Result<(String, String, TimesTaken), String> {
    match day {
        1  => run_day!(day_1,   1),
        _  => Err(format!("Invalid day number ({day})")),
    }
}
