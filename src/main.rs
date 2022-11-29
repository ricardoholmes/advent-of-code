use std::env;
use std::io;
use std::fs;
use std::io::Write;

mod solutions;

fn main() {
    // get sysargs for day num
    let sysargs: Vec<String> = env::args().collect();

    let day_num: String;

    day_num = match sysargs.get(1) {
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
        let mut latest_solution = 0;
        for _ in dir {
            latest_solution += 1;
        }

        // directory also contains mod.rs and template.rs so must account for them
        // but for loop has exclusive top, so - 2 + 1 => - 1
        latest_solution -= 1;
        for day_num in 1..latest_solution {
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
