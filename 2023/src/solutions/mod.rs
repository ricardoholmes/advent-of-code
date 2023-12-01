mod day_1;

pub fn run(day: u16) -> Result<(), String> {
    match day {
        1 => day_1::run(),
        _ => Err(String::from("Invalid day number")),
    }
}
