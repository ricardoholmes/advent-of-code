mod day_1;
mod day_2;
mod day_3;

pub fn run(day: u16) {
    println!("\n--- Day {day} ---");

    let t = std::time::Instant::now();
    match day {
        1 => day_1::run(),
        2 => day_2::run(),
        3 => day_3::run(),
        _ => println!("Invalid day number"),
    };

    println!("Time taken: {:?}", std::time::Instant::now() - t)
}
