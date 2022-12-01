pub fn run() {
    let input_str = include_str!("../../inputs/input_1.txt");

    let input: Vec<Vec<u32>> = input_str
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|s| s.split("\n").map(|x| x.parse().unwrap()).collect::<Vec<u32>>())
        .collect();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vec<Vec<u32>>) {
    let mut sum;
    let mut highest =  0;
    for i in input {
        sum = 0;
        for j in i {
            sum += j;
        }
        if sum > highest {
            highest = sum;
        }
    }

    println!("Part one: {highest}");
}

fn part_two(input: &Vec<Vec<u32>>) {
    let mut sum;
    let mut highest =  0;
    for i in input {
        sum = 0;
        for j in i {
            sum += j;
        }
        if sum > highest {
            highest = sum;
        }
    }

    let mut second_highest =  0;
    for i in input {
        sum = 0;
        for j in i {
            sum += j;
        }

        if sum == highest {
            continue;
        }

        if sum > second_highest {
            second_highest = sum;
        }
    }

    let mut third_highest =  0;
    for i in input {
        sum = 0;
        for j in i {
            sum += j;
        }

        if sum == highest || sum == second_highest {
            continue;
        }

        if sum > third_highest {
            third_highest = sum;
        }
    }

    let total = highest + second_highest + third_highest;
    println!("Part two: {total}");
}
