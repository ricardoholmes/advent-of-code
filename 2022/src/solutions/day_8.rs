pub fn run() {
    let input_str = include_str!("../../inputs/input_8.txt");

    // split input into vector of vectors of numbers
    let input: Vec<Vec<u32>> = input_str
        .lines()
        .map(|s| s
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>())
        .collect();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vec<Vec<u32>>) {
    let mut count: u32 = (input.len()*2 + input[0].len()*2 - 4) as u32;

    for row in 1..input.len()-1 {
        for column in 1..input[0].len()-1 {
            let cell = input[row][column];
            let column_cells = input.iter().map(|x| x[column]);

            // get list of cells in each direction
            let cells_left = input[row].iter().take(column+1);
            let cells_right = input[row].iter().skip(column);
            let cells_up = column_cells.clone().take(row+1);
            let cells_down = column_cells.clone().skip(row);

            // check if theres a larger cell in any direction
            if (*cells_left.clone().max().unwrap() == cell && cells_left.filter(|&&i| i == cell).count() == 1) ||
                (*cells_right.clone().max().unwrap() == cell && cells_right.filter(|&&i| i == cell).count() == 1) ||
                (cells_up.clone().max().unwrap() == cell && cells_up.filter(|&i| i == cell).count() == 1) ||
                (cells_down.clone().max().unwrap() == cell && cells_down.filter(|&i| i == cell).count() == 1)
            {
                count += 1;
            }
        }
    }

    println!("Part one: {count}");
}

fn part_two(input: &Vec<Vec<u32>>) {
    let mut highest_scenic_score: u32 = 0;

    for row in 1..input.len()-1 {
        for column in 1..input[0].len()-1 {
            let cell = input[row][column];

            // count viewable trees to the left
            let mut viewable_left: u32 = 0;
            for &tree in input[row].iter().take(column).rev() {
                viewable_left += 1;
                if tree >= cell {
                    break;
                }
            }

            // count viewable trees to the right
            let mut viewable_right: u32 = 0;
            for &tree in input[row].iter().skip(column+1) {
                viewable_right += 1;
                if tree >= cell {
                    break;
                }
            }

            // count viewable trees upwards
            let mut viewable_up: u32 = 0;
            for tree_row in input.iter().take(row).rev() {
                viewable_up += 1;
                if tree_row[column] >= cell {
                    break;
                }
            }

            // count viewable trees downwards
            let mut viewable_down: u32 = 0;
            for tree_row in input.iter().skip(row+1) {
                viewable_down += 1;
                if tree_row[column] >= cell {
                    break;
                }
            }

            // calculate scenic score and check if it's the new highest
            let scenic_score = viewable_left * viewable_right * viewable_up * viewable_down;
            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
    }

    println!("Part two: {highest_scenic_score}");
}
