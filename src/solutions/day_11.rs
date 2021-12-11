use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/input_11.txt")
        .expect("Failed to read file");

    // split input into vector of vectors of numbers
    let input: Vec<Vec<u32>> = input
        .split("\n")
        .map(|s| s.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect();

    part_one(input.clone());
    part_two(input.clone());
}

fn part_one(mut grid: Vec<Vec<u32>>) {
    let mut total_flashes: u32 = 0;
    for _turn in 0..100 {
        // grid is always 10x10 so no need to use grid.len()
        for y in 0..10 {
            for x in 0..10 {
                // increase octopus charge by 1, but if go back to 0 if charge becomes 10
                grid[y][x] = (grid[y][x] + 1) % 10;
            }
        }

        // vector of octopi/octopodes/octopuses (coords) that have already flashed
        let mut flashed: Vec<(usize, usize)> = vec!();
        loop {
            // find all coordinates that are 0
            let mut zeros: Vec<(usize, usize)> = get_zeros(&grid);
            // filter out octopi that have already flashed
            zeros.retain(|e| !flashed.contains(e));

            // break from loop if no octopus is at 0
            if zeros.len() == 0 {
                break;
            }

            for (x, y) in zeros {
                flashed.push((x, y));
                
                // calculate bounds of loop (to prevent overflow)
                let lower_x_bound = if x == 0 { 0 } else { x - 1 };
                let upper_x_bound = if x == 9 { 10 } else { x + 2 };

                let lower_y_bound = if y == 0 { 0 } else { y - 1 };
                let upper_y_bound = if y == 9 { 10 } else { y + 2 };

                // iterate through all adjacent cells
                for y1 in lower_y_bound..upper_y_bound {
                    for x1 in lower_x_bound..upper_x_bound {
                        // if cell isn't at 0, increase charge by 1
                        if grid[y1][x1] != 0 {
                            grid[y1][x1] = (grid[y1][x1] + 1) % 10;
                        }
                    }
                }
            }
        }
        total_flashes += flashed.len() as u32;
    }

    println!("Part 1: {}", total_flashes);
}

fn part_two(mut grid: Vec<Vec<u32>>) {
    let mut turn = 0;
    loop {
        turn += 1;

        // grid is always 10x10 so no need to use grid.len()
        for y in 0..10 {
            for x in 0..10 {
                // increase octopus charge by 1, but if go back to 0 if charge becomes 10
                grid[y][x] = (grid[y][x] + 1) % 10;
            }
        }

        // vector of octopi (coords) that have already flashed
        let mut flashed: Vec<(usize, usize)> = vec!();
        loop {
            // find all coordinates that are 0
            let mut zeros: Vec<(usize, usize)> = get_zeros(&grid);
            // filter out octopi that have already flashed
            zeros.retain(|e| !flashed.contains(e));

            // break from loop if no octopus is at 0
            if zeros.len() == 0 {
                break;
            }

            for (x, y) in zeros {
                flashed.push((x, y));
                
                // calculate bounds of loop (to prevent overflow)
                let lower_x_bound = if x == 0 { 0 } else { x - 1 };
                let upper_x_bound = if x == 9 { 10 } else { x + 2 };

                let lower_y_bound = if y == 0 { 0 } else { y - 1 };
                let upper_y_bound = if y == 9 { 10 } else { y + 2 };

                // iterate through all adjacent cells
                for y1 in lower_y_bound..upper_y_bound {
                    for x1 in lower_x_bound..upper_x_bound {
                        // if cell isn't at 0, increase charge by 1
                        if grid[y1][x1] != 0 {
                            grid[y1][x1] = (grid[y1][x1] + 1) % 10;
                        }
                    }
                }
            }
        }

        // if all octopus have flashed this turn, break
        if flashed.len() == 100 {
            break;
        }
    }

    println!("Part 2: {}", turn);
}

// get all coordinates of cells that have 0 charge
fn get_zeros(grid: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut zero_coords: Vec<(usize, usize)> = vec!();
    for y in 0..10 {
        for x in 0..10 {
            if grid[y][x] == 0 {
                zero_coords.push((x, y));
            }
        }
    }

    zero_coords
}
