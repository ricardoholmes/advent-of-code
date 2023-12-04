use crate::run_day;

mod day_1;
mod day_2;
mod day_3;
mod day_4;

pub fn run(day: u8) -> Result<(), String> {
    match day {
        1 => run_day!(day_1, 1),
        2 => run_day!(day_2, 2),
        3 => run_day!(day_3, 3),
        4 => run_day!(day_4, 4),
        _ => Err(String::from("Invalid day number")),
    }
}
