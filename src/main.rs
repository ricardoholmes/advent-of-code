use std::io;

mod modules;

fn main() {
    let mut day_num = String::new();
    println!("Enter the question number:");

    io::stdin()
        .read_line(&mut day_num)
        .expect("Failed to read line");
        
    println!("");

    let day_num: u16 = day_num
        .trim()
        .parse()
        .expect("Please enter a number");

    modules::run(day_num);
}
