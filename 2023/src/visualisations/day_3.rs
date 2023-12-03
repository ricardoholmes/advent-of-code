use std::collections::HashSet;

use colored::Colorize;

fn visualise(input: &Vec<Vec<char>>, num_coords: &HashSet<(usize, usize)>, symbol_coords: &HashSet<(usize, usize)>) {
    for line_index in 0..input.len() {
        let line = input.get(line_index).unwrap();
        for char_index in 0..line.len() {
            let c = line.get(char_index).unwrap().to_owned();
            if num_coords.contains(&(line_index, char_index)) {
                print!("{}", c.to_string().cyan());
            }
            else if c.is_numeric() {
                print!("{}", c.to_string().red());
            }
            else if symbol_coords.contains(&(line_index, char_index)) {
                print!("{}", c.to_string().yellow());
            }
            else {
                print!("{c}");
            }
        }
        println!();
    }
}
