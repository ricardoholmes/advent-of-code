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
    
    if day_num.trim() == "all" {
        for day_num in 1..26 {
            println!("\nDay {}", day_num);
            solutions::run(day_num);
        }
    }

    else {
        let day_num: u16 = day_num
            .trim()
            .parse()
            .expect("Please enter a number or 'all'");
        
        solutions::run(day_num);
    }
}
