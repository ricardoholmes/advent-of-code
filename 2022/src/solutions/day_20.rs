pub fn run() {
    let input_str = include_str!("../../inputs/input_20.txt");

    let input: Vec<i64> = input_str
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[i64]) {
    let mut indexes: Vec<usize> = (0..input.len()).collect();
    mix(input, &mut indexes);

    let zero_index_input = input.iter().position(|&num| num == 0).unwrap();
    let zero_pos = indexes.iter().position(|&num| num == zero_index_input).unwrap();

    println!("Part one: {}", [1000, 2000, 3000].iter().map(|&x| input[indexes[(zero_pos + x) % input.len()]]).sum::<i64>());
}

fn part_two(input: &[i64]) {
    let input: Vec<i64> = input.to_vec().iter().map(|&x| x * 811589153).collect();
    let mut indexes: Vec<usize> = (0..input.len()).collect();

    for _ in 0..10 {
        mix(&input, &mut indexes);
    }

    let zero_index_input = input.iter().position(|&num| num == 0).unwrap();
    let zero_pos = indexes.iter().position(|&num| num == zero_index_input).unwrap();

    println!("Part two: {}", [1000, 2000, 3000].iter().map(|&x| input[indexes[(zero_pos + x) % input.len()]]).sum::<i64>());
}

fn mix(input: &[i64], indexes: &mut Vec<usize>) {
    for (i, &value) in input.iter().enumerate() {
        let index = indexes.iter().position(|&index| index == i).unwrap();
        indexes.remove(index);
        let new_index = (index as i64 + value).rem_euclid(indexes.len() as i64) as usize;
        indexes.insert(new_index, i);
    }
}
