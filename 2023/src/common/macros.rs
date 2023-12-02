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
