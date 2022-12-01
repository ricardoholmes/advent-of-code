use std::fs;

pub fn run() {
    // read input
    let input_str = include_str!("../../inputs/input_0.txt")
        .strip_suffix("\n")
        .unwrap();

    // if input is a list of strings
    let input: Vec<&str> = input_str
        .split(",")
        .collect();

    // if input is a list of integers
    let input: Vec<i32> = input_str
        .split(",")
        .map(|s| s.parse().expect("parse error"))
        .collect();

    // if input is a list of lists of integers
    let input: Vec<Vec<u32>> = input_str
        .split("\n")
        .map(|s| s.split(",").map(|x| x.parse().unwrap()).collect::<Vec<u32>>())
        .collect();
}
