mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

pub fn run(day: u16) {
    println!("\n--- Day {day} ---");

    let t = std::time::Instant::now();
    match day {
        1 => day_1::run(),
        2 => day_2::run(),
        3 => day_3::run(),
        4 => day_4::run(),
        5 => day_5::run(),
        6 => day_6::run(),
        7 => day_7::run(),
        8 => day_8::run(),
        _ => println!("Invalid day number"),
    };

    println!("Time taken: {:?}", std::time::Instant::now() - t)
}
