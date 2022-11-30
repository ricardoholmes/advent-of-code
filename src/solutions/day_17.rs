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
    part_two(&bounds);
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
        y_velocity = bounds.y[0];
    }

    let highest_point: i32 = (y_velocity*y_velocity + y_velocity) / 2;

    println!("Part 1: {}", highest_point);
}

fn part_two(bounds: &Bounds) {
    // store amount of initial velocities found
    let mut count: u32 = 0;

    // gets lower bound for velocity using now-not-entirely-useless equation from part 1
    let min_vx: i32 = ((-1.0 + (1.0 + 8.0 * bounds.x[0] as f64).sqrt()) / 2.0).ceil() as i32;

    for vx_start in min_vx..(bounds.x[1] + 1) {
        let mut time = 0;

        let mut x = 0;
        let mut vx = vx_start;

        // calculate time, vx, and x when the probe first enters the target area
        loop {
            x += vx;
            vx = if vx > 0 { vx - 1 } else { 0 };
            time += 1;
            if x >= bounds.x[0] {
                break;
            }
        }

        // if x is outside the boundaries, go to the next vx_start
        // no need to account for x not reaching the boundaries (< bounds.x[0]) since min_vx is being used
        if x > bounds.x[1] {
            continue;
        }

        for vy_start in bounds.y[0]..1000 {
            // set variables to shadow upper variables
            // so that upper ones dont get changed within this nested loop
            let mut time = time;

            let mut x = x;
            let mut vx = vx;

            // calculate y position when x first enters target area
            let mut y = 0;
            for i in 0..time {
                y += vy_start - i;
            }
            let mut vy = vy_start - time;

            // loop while probe could still potentially hit
            while x <= bounds.x[1] && (vx > 0 || vy > 0 || bounds.y[0] <= y) {
                // if target hit, increment count and go to next vx_start
                if y >= bounds.y[0] && y <= bounds.y[1] {
                    count += 1;
                    break;
                }

                // update position, velocity, and time
                x += vx;
                y += vy;
                vy -= 1;
                vx = if vx > 0 { vx - 1 } else { 0 };
                time += 1;
            }
        }
    }

    println!("Part 2: {count}");
}
