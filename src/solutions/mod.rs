mod day_1;

pub fn run(day: u16) {
    match day {
        1 => day_1::run(),
        _ => println!("Invalid day number"),
    }
}
