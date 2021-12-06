mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

pub fn run(day: u16) {
    match day {
        1 => day_1::run(),
        2 => day_2::run(),
        3 => day_3::run(),
        4 => day_4::run(),
        5 => day_5::run(),
        6 => day_6::run(),
        _ => println!("Invalid day number"),
    }
}
