pub fn run() {
    let input_str = include_str!("../../inputs/input_18.txt");

    let input: Vec<Vec<i32>> = input_str
        .lines()
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();

    part_one(&input)
}

fn part_one(input: &Vec<Vec<i32>>) {
    let mut count = 0;

    for cube in input {
        for i in 0..3 {
            for j in &[-1, 1] {
                let mut new_cube = cube.clone();
                new_cube[i] += j;
                if !input.contains(&new_cube) {
                    count += 1;
                }
            }
        }
    }

    println!("Part one: {count}");
}
