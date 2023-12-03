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
    ($day_mod:ident, $day_num:expr) => {
        {
            use std::fs;
            use std::path::PathBuf;

            let input_file = format!("inputs/input_{}.txt", $day_num);
            let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            input_path.push(input_file);
            let raw_input = crate::try_get_ok!(fs::read_to_string(input_path));

            $day_mod::run(raw_input.as_str())
        }
    }
}

#[macro_export]
macro_rules! try_get_ok {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(e) => return Err(e.to_string()),
        }
    }
}
