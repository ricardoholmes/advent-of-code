pub fn run() {
    let input_str = include_str!("../../inputs/input_8.txt");

    let input: Vec<Vec<u32>> = input_str
        .lines()
        .map(|s| s
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>())
        .collect();

    part_one(&input)
}

fn part_one(input: &Vec<Vec<u32>>) {
    let mut count: u32 = (input.len()*2 + input[0].len()*2 - 4) as u32;

    for row in 1..input.len()-1 {
        for column in 1..input[0].len()-1 {
            let cell = input[row][column];
            let column_cells = input.iter().map(|x| x[column]);

            let cells_left = input[row].iter().take(column+1);
            let cells_right = input[row].iter().skip(column);
            let cells_up = column_cells.clone().take(row+1);
            let cells_down = column_cells.clone().skip(row);

            if (*cells_left.clone().max().unwrap() == cell && cells_left.filter(|i| **i == cell).count() == 1) ||
                (*cells_right.clone().max().unwrap() == cell && cells_right.filter(|i| **i == cell).count() == 1) ||
                (cells_up.clone().max().unwrap() == cell && cells_up.filter(|i| *i == cell).count() == 1) ||
                (cells_down.clone().max().unwrap() == cell && cells_down.filter(|i| *i == cell).count() == 1)
            {
                count += 1;
            }
        }
    }

    println!("Part one: {count}");
}
