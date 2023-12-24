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
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;

pub fn run(day: u8) -> Result<(i128, i128, TimesTaken), String> {
    match day {
        1 => run_day!(day_1, 1),
        2 => run_day!(day_2, 2),
        3 => run_day!(day_3, 3),
        4 => run_day!(day_4, 4),
        5 => run_day!(day_5, 5),
        6 => run_day!(day_6, 6),
        7 => run_day!(day_7, 7),
        8 => run_day!(day_8, 8),
        9 => run_day!(day_9, 9),
        10 => run_day!(day_10, 10),
        11 => run_day!(day_11, 11),
        12 => run_day!(day_12, 12),
        13 => run_day!(day_13, 13),
        14 => run_day!(day_14, 14),
        15 => run_day!(day_15, 15),
        16 => run_day!(day_16, 16),
        17 => run_day!(day_17, 17),
        18 => run_day!(day_18, 18),
        19 => run_day!(day_19, 19),
        20 => run_day!(day_20, 20),
        21 => run_day!(day_21, 21),
        22 => run_day!(day_22, 22),
        23 => run_day!(day_23, 23),
        24 => run_day!(day_24, 24),
        _ => Err(String::from("Invalid day number")),
    }
}
