use std::fs;

#[derive(Debug)]
struct Bounds {
    x: [i32; 2],
    y: [i32; 2],
}

pub fn run() {
    let input_str: String = fs::read_to_string("inputs/input_17.txt")
        .expect("Failed to read file");

    let bounds: Bounds = parse_input(input_str);

    part_one(&bounds);
}

fn parse_input(mut input_str: String) -> Bounds {
    input_str = input_str[input_str.chars().position(|c| c == '=').unwrap()+1..].to_string();
    
    let x_bounds: Vec<i32> = input_str[..input_str.chars().position(|c| c == ',').unwrap()]
        .split("..")
        .map(|s| s.parse::<i32>().expect("Couldn't parse x bounds"))
        .collect();

    let y_bounds: Vec<i32> = input_str[input_str.chars().position(|c| c == '=').unwrap()+1..]
        .split("..")
        .map(|s| s.parse::<i32>().expect("Couldn't parse y bounds"))
        .collect();

    Bounds {
        x: [x_bounds[0], x_bounds[1]],
        y: [y_bounds[0], y_bounds[1]],
    }
}

fn part_one(bounds: &Bounds) {
    // calculate x velocity
    // final xpos = vx^2 - (vx*(vx-1))/2 (triangle nums) => (vx^2 + vx) / 2
    // therefore:
    let x_velocity: i32 = ((-1.0 + (1.0 + 8.0 * bounds.x[0] as f64).sqrt()) / 2.0).ceil() as i32;
    println!("vx = {}", x_velocity);
    // above is unnecessary but i feel bad removing it because i put effort into it :(

    let y_velocity: i32; // launch y velocity

    // vy of object at y=0 is y_velocity...
    if bounds.y[0] < 0 { 
        // ...if bounds are below then distance to lowest bound must be y_velocity + 1
        y_velocity = -bounds.y[0] - 1;
    }
    else {
        // ...if bounds are above then distance to lowest bound must be y_velocity
        y_velocity = bounds.y[0]
    }

    let highest_point: i32 = (y_velocity*y_velocity + y_velocity) / 2;

    println!("Part 1: {}", highest_point);
}
