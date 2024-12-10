use crate::{run_day, common::times_taken::TimesTaken};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;

pub fn run(day: u8) -> Result<(String, String, TimesTaken), String> {
    match day {
        1  => run_day!(day_1,   1),
        2  => run_day!(day_2,   2),
        3  => run_day!(day_3,   3),
        4  => run_day!(day_4,   4),
        5  => run_day!(day_5,   5),
        6  => run_day!(day_6,   6),
        7  => run_day!(day_7,   7),
        8  => run_day!(day_8,   8),
        9  => run_day!(day_9,   9),
        10 => run_day!(day_10, 10),
        _  => Err(format!("Invalid day number ({day})")),
    }
}
