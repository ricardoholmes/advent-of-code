use std::fs;

pub fn get_all_day_nums() -> Result<Vec<u8>, String> {
    let dir = match fs::read_dir("src/solutions") {
        Ok(d) => d,
        Err(e) => return Err(String::from("Input file could not be found")),
    };

    let mut days: Vec<u8> = dir
        .filter_map(|file| match file {
                Ok(f) => match f.file_name().to_str() {
                    Some(filename) => extract_day_num(filename),
                    None => None,
                },
                Err(_) => return None,
            }
        ).collect();

    days.sort();
    Ok(days)
}

fn extract_day_num(filename: &str) -> Option<u8> {
    match filename.strip_prefix("day_") {
        Some(filename) => match filename.strip_suffix(".rs") {
            Some (filename) => match filename.parse() {
                Ok(day_num) => Some(day_num),
                Err(_) => None,
            },
            None => None
        },
        None => None
    }
}
