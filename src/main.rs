use std::io;

mod modules;

fn main() {
    let mut question_number = String::new();
    println!("Enter the question number: ");

    io::stdin()
        .read_line(&mut question_number)
        .expect("Failed to read line");
        
    println!("");

    let question_number: u16 = question_number
        .trim()
        .parse()
        .expect("Please enter a number");

    match question_number {
        1 => modules::day_one(),
        _ => println!("Invalid question number"),
    }
}
