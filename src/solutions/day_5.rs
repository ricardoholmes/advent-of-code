pub fn run() {
    let input_str = include_str!("../../inputs/input_5.txt");

    let input: Vec<&str> = input_str
        .split("\n\n")
        .collect();

    let lines: Vec<Vec<String>> = input[0]
        .replace(&['[', ']'], "")
        .lines()
        .take(input[0].lines().count()-1)
        .map(|line| line
            .replace("    ", " ")
            .split(' ')
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
        )
        .collect();

    let stack_count: usize = input[0]
        .lines()
        .skip(input[0].lines().count()-1)
        .collect::<Vec<&str>>()[0]
        .split("   ")
        .count();

    let mut stacks: Vec<Vec<String>> = vec!();

    for _ in 0..stack_count {
        stacks.push(vec!());
    }

    for line in lines {
        for i in 0..stack_count {
            if !line[i].is_empty() {
                stacks[i].insert(0, line[i].clone());
            }
        }
    }

    let commands: Vec<Vec<usize>> = input[1]
        .replace("move ", "")
        .replace(" from ", " ")
        .replace(" to ", " ")
        .lines()
        .map(|line| line
            .split(' ')
            .map(|num| num
                .parse::<usize>()
                .unwrap()
            ).collect::<Vec<usize>>()
        )
        .collect();

    part_one(&stacks, &commands);
    part_two(&stacks, &commands);
}

fn part_one(stacks: &[Vec<String>], commands: &Vec<Vec<usize>>) {
    let mut stacks = stacks.to_owned();

    for command in commands {
        for _ in 0..command[0] {
            let value = stacks[command[1]-1].pop().unwrap();
            stacks[command[2]-1].push(value);
        }
    }

    let mut out: String = "".to_owned();

    for stack in stacks {
        out.push_str(stack.last().unwrap());
    }

    println!("Part one: {out}");
}

fn part_two(stacks: &[Vec<String>], commands: &Vec<Vec<usize>>) {
    let mut stacks = stacks.to_owned();

    for command in commands {
        let index = stacks[command[1]-1].len() - command[0];

        let mut values: Vec<String> = stacks[command[1]-1]
            .split_off(index);

        stacks[command[2]-1].append(&mut values);
    }

    let mut out: String = "".to_owned();

    for stack in stacks {
        out.push_str(stack.last().unwrap());
    }

    println!("Part two: {out}");
}
