use std::io;
use std::env;

mod solutions;

fn main() {
    // get sysargs for day num
    let sysargs: Vec<String> = env::args().collect();

    let mut day_num = String::new();

    // if no sysarg then:
    if sysargs.len() < 2 {
        println!("Enter the question number:");

        io::stdin()
            .read_line(&mut day_num)
            .expect("Failed to read line");
            
        println!("");

    }
    
    // otherwise
    else {
        day_num = (*sysargs[1]).to_string();
    }
    
    let day_num: u16 = day_num
        .trim()
        .parse()
        .expect("Please enter a number");
    
    solutions::run(day_num);
}
