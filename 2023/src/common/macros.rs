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
            let input_file = format!("../../inputs/input_{}.txt", $day_num);
            let raw_input = match std::fs::read_to_string(input_file) {
                Ok(input) => input,
                Err(err) => return Err(err.to_string()),
            };

            $day_mod::run(raw_input.as_str())
        }
    }
}
