mod day_1;
mod day_2;

pub fn run(day: u16) {
    match day {
        1 => day_1::run(),
        2 => day_2::run(),
        _ => println!("Invalid day number"),
    }
}
