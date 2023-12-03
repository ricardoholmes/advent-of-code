use std::{fs, time::Duration};

use colored::{Color, ColoredString, Colorize};

pub fn get_all_day_nums() -> Result<Vec<u8>, String> {
    let dir = match fs::read_dir("src/solutions") {
        Ok(d) => d,
        _ => return Err(String::from("Input file could not be found")),
    };

    let mut days: Vec<u8> = dir
        .filter_map(|file| match file {
            Ok(f) => match f.file_name().to_str() {
                Some(filename) => extract_day_num(filename),
                None => None,
            },
            Err(_) => return None,
        })
        .collect();

    days.sort();
    Ok(days)
}

fn extract_day_num(filename: &str) -> Option<u8> {
    match filename.strip_prefix("day_") {
        Some(filename) => match filename.strip_suffix(".rs") {
            Some(filename) => match filename.parse() {
                Ok(day_num) => Some(day_num),
                Err(_) => None,
            },
            None => None,
        },
        None => None,
    }
}

pub fn color_time_taken(time_taken: Duration) -> ColoredString {
    let boundaries = [500, 1000, 10_000, 100_000, 500_000, 1_000_000, 10_000_000];

    let color_order = [
        Color::Cyan,
        Color::BrightGreen,
        Color::Green,
        Color::BrightYellow,
        Color::Yellow,
        Color::Magenta,
        Color::Red,
        Color::BrightRed
    ];

    for i in 0..boundaries.len() {
        let micros = boundaries[i];
        if time_taken < Duration::from_micros(micros) {
            return format!("{time_taken:?}").color(color_order[i]);
        }
    }

    format!("{time_taken:?}").color(color_order[color_order.len() - 1])
}
