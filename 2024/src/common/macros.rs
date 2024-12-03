#[macro_export]
macro_rules! safe_unpack {
    ($list:tt, ()) => { };
    ($list:tt, $name:tt) => {
        let $name = match $list.next() {
            Some(val) => val,
            None => return Err(String::from("Not enough values in list.")),
        };
    };
    ($list:tt, $name:tt $(, $vars:tt)+) => {
        safe_unpack!($list, $name);
        safe_unpack!($list $(, $vars)+)
    };
    ($e:expr $(, $vars:tt)+) => {
        let mut temp_iter = $e;
        safe_unpack!(temp_iter $(, $vars)+);
    };
}

#[macro_export]
macro_rules! run_day {
    ($day_mod:ident, $day_num:expr) => {{
        use std::fs;
        use std::time::Instant;
        use std::path::PathBuf;

        let input_file = format!("inputs/input_{}.txt", $day_num);
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push(input_file);
        let raw_input = $crate::try_get_ok!(fs::read_to_string(input_path));

        let start_time = Instant::now();

        // parse the input file
        let parsed = $day_mod::parse(raw_input.as_str())?;
        let parse_time = Instant::now() - start_time;

        // run part one
        let part_one_start_time = Instant::now();
        let answer_part_one = $day_mod::part_one(&parsed)?;
        let part_one_time = Instant::now() - part_one_start_time;

        // run part two
        let part_two_start_time = Instant::now();
        let answer_part_two = $day_mod::part_two(&parsed)?;
        let part_two_time = Instant::now() - part_two_start_time;

        let total_time = Instant::now() - start_time;

        Ok((answer_part_one.to_string(), answer_part_two.to_string(), TimesTaken {
            total: total_time,
            parsing: parse_time,
            part_one: part_one_time,
            part_two: part_two_time,
        }))
    }};
}

#[macro_export]
macro_rules! try_get_ok {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(e) => return Err(e.to_string()),
        }
    };
}
