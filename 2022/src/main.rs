use std::env;
use std::io;
use std::fs;
use std::io::Write;

mod solutions;

fn main() {
    // get sysargs for day num
    let sysargs: Vec<String> = env::args().collect();

    let day_num: String = match sysargs.get(1) {
        Some(x) => x.to_string(),
        None => {
            let mut buffer = String::new();
            print!("\nEnter the question number: ");
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin().read_line(&mut buffer).expect("Failed to read line");
            buffer.trim().to_string()
        }
    };

    if day_num == "all" {
        let dir = fs::read_dir("./src/solutions").unwrap();
        let latest_solution = dir.filter(|file| 
            file.as_ref().unwrap().file_name().to_str().unwrap().starts_with("day")
        ).fold(0, |sum, _| sum + 1);

        println!("======================================");
        println!("  A D V E N T  O F  C O D E  2 0 2 2  ");
        println!("======================================");
        let start_time = std::time::Instant::now();
        for day_num in 1..=latest_solution {
            if day_num == 16 || day_num == 18 {
                // too slow to run for now
                continue;
            }
            solutions::run(day_num);
        }
        println!("\n=================");
        println!(" F I N I S H E D ");
        println!("=================");
        println!("Done in {:?}", std::time::Instant::now() - start_time);
    }

    else {
        match day_num.parse() {
            Ok(day) => solutions::run(day),
            Err(_) => println!("\nERROR : Failed to parse day number"),
        }
    }
}
