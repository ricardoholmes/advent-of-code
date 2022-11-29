use std::io;
use std::env;

mod solutions;

fn main() {
    // get sysargs for day num
    let sysargs: Vec<String> = env::args().collect();

    let day_num: String;

    day_num = match sysargs.get(1) {
        Some(x) => x.to_string(),
        None => {
            let mut buffer = String::new();
            println!("Enter the question number:");
            io::stdin().read_line(&mut buffer).expect("Failed to read line");
            println!("");
            buffer.trim().to_string()
        }
    };

    if day_num == "all" {
        for day_num in 1..26 {
            println!("\n--- Day {} ---", day_num);
            solutions::run(day_num);
        }
    }

    else {
        println!("\n--- Day {} ---", day_num);
        let day_num: u16 = day_num
            .trim()
            .to_string()
            .parse()
            .unwrap_or(u16::MAX);

        solutions::run(day_num);
    }
}
