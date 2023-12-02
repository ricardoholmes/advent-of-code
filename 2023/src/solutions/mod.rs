mod day_1;
mod day_2;

pub fn run(day: u16) -> Result<(), String> {
    match day {
        1 => day_1::run(),
        2 => day_2::run(),
        _ => Err(String::from("Invalid day number")),
    }
}
