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
        9 => day_9::run(),
        10 => day_10::run(),
        11 => day_11::run(),
        12 => day_12::run(),
        13 => day_13::run(),
        14 => day_14::run(),
        15 => day_15::run(),
        16 => day_16::run(),
        17 => day_17::run(),
        18 => day_18::run(),
        19 => day_19::run(),
        20 => day_20::run(),
        21 => day_21::run(),
        22 => day_22::run(),
        23 => day_23::run(),
        24 => day_24::run(),
        _ => println!("Invalid day number"),
    };

    println!("Time taken: {:?}", std::time::Instant::now() - t);
}
